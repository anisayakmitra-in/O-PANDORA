//! Constitutional compatibility layer for providers.
//!
//! The provider crate has its own ProviderManifest,
//! ModelCapabilities, and LanguageSupport types. The
//! constitutional types in pandora-types are the
//! canonical source of truth. This module provides
//! From conversions from the provider types to the
//! constitutional types.
//!
//! Existing provider types are NOT modified. New code
//! can use the From conversions to migrate to the
//! constitutional types.

use pandora_types::constitutional::{
    ConstitutionalManifest, ManifestCapability, ManifestKind, ManifestVersion,
};

use crate::capability::{LanguageSupport, ModelCapabilities};
use crate::manifest::ProviderManifest;

fn parse_version(s: &str) -> ManifestVersion {
    let trimmed = s.trim().trim_start_matches('v');
    let parts: Vec<&str> = trimmed.split('.').collect();
    let major = parts.first().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
    ManifestVersion::new(major, minor, patch)
}

impl From<&ModelCapabilities> for Vec<ManifestCapability> {
    fn from(c: &ModelCapabilities) -> Self {
        let mut out = Vec::new();
        if c.multilingual {
            out.push(ManifestCapability::new(
                "multilingual",
                "supports multiple languages",
            ));
        }
        if c.supports_streaming {
            out.push(ManifestCapability::new(
                "streaming",
                "supports token streaming",
            ));
        }
        if c.supports_embeddings {
            out.push(ManifestCapability::new(
                "embeddings",
                "supports embedding generation",
            ));
        }
        if c.supports_tools {
            out.push(ManifestCapability::new(
                "tools",
                "supports tool/function calling",
            ));
        }
        if c.context_window > 0 {
            out.push(ManifestCapability::new(
                "context",
                format!("context window: {} tokens", c.context_window).as_str(),
            ));
        }
        for lang in &c.supported_languages {
            out.push(ManifestCapability::from(lang));
        }
        out
    }
}

impl From<&LanguageSupport> for ManifestCapability {
    fn from(l: &LanguageSupport) -> Self {
        ManifestCapability::new(
            format!("lang.{}", l.language_code),
            format!("supports {} ({})", l.language_name, l.country),
        )
    }
}

impl From<&ProviderManifest> for ConstitutionalManifest {
    fn from(m: &ProviderManifest) -> Self {
        let version: ManifestVersion = parse_version(&m.version);
        let mut cm = ConstitutionalManifest::new(
            m.name.clone(),
            ManifestKind::Provider,
            version,
            m.id.clone(),
        );
        cm.capabilities = (&m.capabilities).into();
        // Provider id is preserved as an extension so it
        // can be retrieved by callers that need it.
        cm.extensions
            .values
            .insert("__provider_id".to_string(), m.id.clone());
        if let Some(endpoint) = &m.endpoint {
            cm.extensions
                .values
                .insert("__endpoint".to_string(), endpoint.clone());
        }
        for model in &m.models {
            cm.extensions
                .values
                .insert(format!("__model.{}", model), "supported".to_string());
        }
        cm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caps() -> ModelCapabilities {
        ModelCapabilities {
            multilingual: true,
            supported_languages: vec![],
            context_window: 8192,
            supports_streaming: true,
            supports_embeddings: false,
            supports_tools: true,
        }
    }

    fn lang() -> LanguageSupport {
        LanguageSupport {
            language_code: "en".to_string(),
            language_name: "English".to_string(),
            country: "US".to_string(),
            confidence: 1.0,
        }
    }

    fn manifest() -> ProviderManifest {
        ProviderManifest {
            id: "ollama".to_string(),
            name: "Ollama".to_string(),
            version: "1.0.0".to_string(),
            models: vec!["llama3".to_string()],
            capabilities: caps(),
            endpoint: Some("http://localhost".to_string()),
            locality: crate::target::Locality::Local,
        }
    }

    #[test]
    fn model_caps_to_vec_empty() {
        let empty = ModelCapabilities::default();
        let v: Vec<ManifestCapability> = (&empty).into();
        assert!(v.is_empty());
    }

    #[test]
    fn model_caps_to_vec_streaming() {
        let v: Vec<ManifestCapability> = (&caps()).into();
        let names: Vec<&str> = v.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"multilingual"));
        assert!(names.contains(&"streaming"));
        assert!(names.contains(&"tools"));
        assert!(!names.contains(&"embeddings"));
        assert!(names.iter().any(|n| n.starts_with("context")));
    }

    #[test]
    fn language_to_capability() {
        let c: ManifestCapability = (&lang()).into();
        assert_eq!(c.name, "lang.en");
        assert!(c.description.contains("English"));
    }

    #[test]
    fn model_caps_with_language() {
        let mut c = caps();
        c.supported_languages.push(lang());
        let v: Vec<ManifestCapability> = (&c).into();
        assert!(v.iter().any(|c| c.name == "lang.en"));
    }

    #[test]
    fn parse_version_basic() {
        let v = parse_version("2.3.4");
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 3);
        assert_eq!(v.patch, 4);
    }

    #[test]
    fn parse_version_v_prefix() {
        let v = parse_version("v1.0");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);
    }

    #[test]
    fn provider_manifest_to_constitutional() {
        let m = manifest();
        let cm: ConstitutionalManifest = (&m).into();
        assert_eq!(cm.identity.name, "Ollama");
        assert_eq!(cm.identity.kind, ManifestKind::Provider);
        assert_eq!(cm.identity.version.major, 1);
        assert_eq!(
            cm.extensions.values.get("__provider_id"),
            Some(&"ollama".to_string())
        );
        assert_eq!(
            cm.extensions.values.get("__model.llama3"),
            Some(&"supported".to_string())
        );
    }

    #[test]
    fn provider_manifest_capabilities_included() {
        let m = manifest();
        let cm: ConstitutionalManifest = (&m).into();
        assert!(cm.capabilities.iter().any(|c| c.name == "streaming"));
        assert!(cm.capabilities.iter().any(|c| c.name == "tools"));
    }
}
