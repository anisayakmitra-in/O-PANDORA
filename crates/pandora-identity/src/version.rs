use std::fmt;

use serde::{Deserialize, Serialize};

/// A semver-compatible version. Used for constitutional
/// identity versioning.  is intentionally
/// minimal: major.minor.patch plus an optional pre-release
/// label and build metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IdentityVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

impl IdentityVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        IdentityVersion {
            major,
            minor,
            patch,
            pre_release: None,
            build_metadata: None,
        }
    }

    pub fn with_pre_release(mut self, pre: impl Into<String>) -> Self {
        self.pre_release = Some(pre.into());
        self
    }

    pub fn with_build_metadata(mut self, build: impl Into<String>) -> Self {
        self.build_metadata = Some(build.into());
        self
    }

    /// Parse a semver string. Returns  on parse
    /// failure. Format: .
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        // Split off build metadata
        let (s, build) = match s.split_once('+') {
            Some((a, b)) => (a, Some(b.to_string())),
            None => (s, None),
        };
        // Split off pre-release
        let (s, pre) = match s.split_once('-') {
            Some((a, b)) => (a, Some(b.to_string())),
            None => (s, None),
        };
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        let mut v = IdentityVersion {
            major,
            minor,
            patch,
            pre_release: pre,
            build_metadata: build,
        };
        // Sanitize empty strings
        if let Some(ref p) = v.pre_release {
            if p.is_empty() {
                v.pre_release = None;
            }
        }
        if let Some(ref b) = v.build_metadata {
            if b.is_empty() {
                v.build_metadata = None;
            }
        }
        Some(v)
    }

    /// True if this is a pre-1.0 version. The runtime
    /// treats pre-1.0 versions as unstable.
    pub fn is_unstable(&self) -> bool {
        self.major == 0
    }

    /// True if this version is compatible with another
    /// (same major, this >= other on minor/patch).
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.major == other.major && self >= other
    }
}

impl Default for IdentityVersion {
    fn default() -> Self {
        IdentityVersion::new(0, 1, 0)
    }
}

impl fmt::Display for IdentityVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref p) = self.pre_release {
            write!(f, "-{}", p)?;
        }
        if let Some(ref b) = self.build_metadata {
            write!(f, "+{}", b)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_display() {
        let v = IdentityVersion::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn with_pre_release() {
        let v = IdentityVersion::new(1, 0, 0).with_pre_release("alpha");
        assert_eq!(v.to_string(), "1.0.0-alpha");
    }

    #[test]
    fn parse_simple() {
        let v = IdentityVersion::parse("1.2.3").unwrap();
        assert_eq!((v.major, v.minor, v.patch), (1, 2, 3));
        assert!(v.pre_release.is_none());
    }

    #[test]
    fn parse_with_pre_and_build() {
        let v = IdentityVersion::parse("2.0.0-beta.1+exp.sha.5114f85").unwrap();
        assert_eq!(v.major, 2);
        assert_eq!(v.pre_release.as_deref(), Some("beta.1"));
        assert_eq!(v.build_metadata.as_deref(), Some("exp.sha.5114f85"));
    }

    #[test]
    fn unstable_0_x() {
        let v = IdentityVersion::new(0, 1, 0);
        assert!(v.is_unstable());
        let v = IdentityVersion::new(1, 0, 0);
        assert!(!v.is_unstable());
    }

    #[test]
    fn ordering() {
        let a = IdentityVersion::new(1, 0, 0);
        let b = IdentityVersion::new(1, 2, 0);
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn default_is_0_1_0() {
        let v = IdentityVersion::default();
        assert_eq!(v.to_string(), "0.1.0");
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert!(IdentityVersion::parse("not-a-version").is_none());
        assert!(IdentityVersion::parse("1.2").is_none());
    }
}
