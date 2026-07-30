use std::env;

#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("invalid secret name: {0}")]
    InvalidName(String),
    #[error("secret is unavailable: {0}")]
    Unavailable(String),
    #[error("secret storage failed: {0}")]
    Storage(String),
}

pub trait SecretSource {
    fn name(&self) -> &str;
    fn get(&self, name: &str) -> Result<Option<String>, SecretError>;
    fn set(&self, name: &str, value: &str) -> Result<(), SecretError>;
    fn delete(&self, name: &str) -> Result<(), SecretError>;
}

pub struct EnvironmentSecretSource;

impl EnvironmentSecretSource {
    fn variable_name(name: &str) -> String {
        format!(
            "PANDORA_SECRET_{}",
            name.replace(['-', '.'], "_").to_ascii_uppercase()
        )
    }
}

impl SecretSource for EnvironmentSecretSource {
    fn name(&self) -> &str {
        "environment"
    }

    fn get(&self, name: &str) -> Result<Option<String>, SecretError> {
        validate_name(name)?;
        Ok(env::var(Self::variable_name(name))
            .ok()
            .filter(|value| !value.is_empty()))
    }

    fn set(&self, _name: &str, _value: &str) -> Result<(), SecretError> {
        Err(SecretError::Storage(
            "environment secrets are read-only".into(),
        ))
    }

    fn delete(&self, _name: &str) -> Result<(), SecretError> {
        Err(SecretError::Storage(
            "environment secrets are read-only".into(),
        ))
    }
}

pub struct LocalSecretSource;

impl LocalSecretSource {
    fn credential_dir() -> std::path::PathBuf {
        env::var_os("PANDORA_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|| env::var_os("USERPROFILE").map(std::path::PathBuf::from))
            .or_else(|| env::var_os("HOME").map(std::path::PathBuf::from))
            .map(|root| {
                if env::var_os("PANDORA_HOME").is_some() {
                    root.join("credentials")
                } else {
                    root.join(".pandora").join("credentials")
                }
            })
            .unwrap_or_else(|| std::path::PathBuf::from(".pandora/credentials"))
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    fn native_get(name: &str) -> Result<String, SecretError> {
        keyring::Entry::new("pandora", name)
            .map_err(|error| SecretError::Unavailable(error.to_string()))?
            .get_password()
            .map_err(|error| SecretError::Unavailable(error.to_string()))
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    fn native_set(name: &str, value: &str) -> Result<(), SecretError> {
        keyring::Entry::new("pandora", name)
            .map_err(|error| SecretError::Storage(error.to_string()))?
            .set_password(value)
            .map_err(|error| SecretError::Storage(error.to_string()))
    }

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    fn native_delete(name: &str) {
        if let Ok(entry) = keyring::Entry::new("pandora", name) {
            let _ = entry.delete_credential();
        }
    }
}

impl SecretSource for LocalSecretSource {
    fn name(&self) -> &str {
        "local"
    }

    fn get(&self, name: &str) -> Result<Option<String>, SecretError> {
        validate_name(name)?;
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        if let Ok(value) = Self::native_get(name) {
            return Ok(Some(value));
        }
        let path = Self::credential_dir().join(format!("{name}.enc"));
        match std::fs::read_to_string(path) {
            Ok(value) => Ok(Some(decrypt(&value)?)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(SecretError::Unavailable(error.to_string())),
        }
    }

    fn set(&self, name: &str, value: &str) -> Result<(), SecretError> {
        validate_name(name)?;
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        if Self::native_set(name, value).is_ok() {
            return Ok(());
        }
        let directory = Self::credential_dir();
        std::fs::create_dir_all(&directory)
            .map_err(|error| SecretError::Storage(error.to_string()))?;
        let path = directory.join(format!("{name}.enc"));
        std::fs::write(&path, encrypt(value)?)
            .map_err(|error| SecretError::Storage(error.to_string()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
                .map_err(|error| SecretError::Storage(error.to_string()))?;
        }
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), SecretError> {
        validate_name(name)?;
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        Self::native_delete(name);
        let path = Self::credential_dir().join(format!("{name}.enc"));
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(SecretError::Storage(error.to_string())),
        }
    }
}

pub struct SecretStore {
    sources: Vec<Box<dyn SecretSource + Send + Sync>>,
}

impl Default for SecretStore {
    fn default() -> Self {
        Self {
            sources: vec![
                Box::new(EnvironmentSecretSource),
                Box::new(LocalSecretSource),
            ],
        }
    }
}

impl SecretStore {
    pub fn get(&self, name: &str) -> Result<Option<String>, SecretError> {
        for source in &self.sources {
            if let Some(value) = source.get(name)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    pub fn set(&self, name: &str, value: &str) -> Result<&'static str, SecretError> {
        LocalSecretSource.set(name, value)?;
        Ok("local")
    }

    pub fn delete(&self, name: &str) -> Result<(), SecretError> {
        LocalSecretSource.delete(name)
    }
}

pub fn validate_name(name: &str) -> Result<(), SecretError> {
    if name.is_empty()
        || !name.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
    {
        return Err(SecretError::InvalidName(name.into()));
    }
    Ok(())
}

pub fn credential_name(connection_name: &str) -> Result<String, SecretError> {
    validate_name(connection_name)?;
    Ok(format!("provider-{connection_name}"))
}

fn key() -> Result<[u8; 32], SecretError> {
    let value = env::var("PANDORA_CREDENTIALS_KEY").map_err(|_| {
        SecretError::Unavailable(
            "PANDORA_CREDENTIALS_KEY is required for encrypted fallback".into(),
        )
    })?;
    if value.is_empty() {
        return Err(SecretError::Unavailable(
            "PANDORA_CREDENTIALS_KEY must not be empty".into(),
        ));
    }
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(value.as_bytes());
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    Ok(output)
}

fn encrypt(value: &str) -> Result<String, SecretError> {
    use ring::{
        aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM},
        rand::{SecureRandom, SystemRandom},
    };
    let key = LessSafeKey::new(
        UnboundKey::new(&AES_256_GCM, &key()?)
            .map_err(|_| SecretError::Storage("invalid encryption key".into()))?,
    );
    let mut nonce_bytes = [0u8; 12];
    SystemRandom::new()
        .fill(&mut nonce_bytes)
        .map_err(|_| SecretError::Storage("could not generate nonce".into()))?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let mut bytes = value.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut bytes)
        .map_err(|_| SecretError::Storage("encryption failed".into()))?;
    let mut encoded = nonce_bytes.to_vec();
    encoded.extend(bytes);
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        encoded,
    ))
}

fn decrypt(encoded: &str) -> Result<String, SecretError> {
    use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
    let mut bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)
        .map_err(|_| SecretError::Unavailable("invalid credential encoding".into()))?;
    if bytes.len() < 28 {
        return Err(SecretError::Unavailable("credential is truncated".into()));
    }
    let nonce_bytes: [u8; 12] = bytes[..12]
        .try_into()
        .map_err(|_| SecretError::Unavailable("invalid nonce".into()))?;
    let plaintext = LessSafeKey::new(
        UnboundKey::new(&AES_256_GCM, &key()?)
            .map_err(|_| SecretError::Unavailable("invalid encryption key".into()))?,
    )
    .open_in_place(
        Nonce::assume_unique_for_key(nonce_bytes),
        Aad::empty(),
        &mut bytes[12..],
    )
    .map_err(|_| SecretError::Unavailable("credential authentication failed".into()))?;
    String::from_utf8(plaintext.to_vec())
        .map_err(|_| SecretError::Unavailable("credential is not UTF-8".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credential_names_are_safe_and_stable() {
        assert_eq!(credential_name("openai").unwrap(), "provider-openai");
        assert!(credential_name("../outside").is_err());
    }

    #[test]
    fn environment_source_reads_named_secret() {
        std::env::set_var("PANDORA_SECRET_TEST_PROVIDER", "secret");
        assert_eq!(
            EnvironmentSecretSource.get("test-provider").unwrap(),
            Some("secret".into())
        );
        std::env::remove_var("PANDORA_SECRET_TEST_PROVIDER");
    }
}
