//! Trust Verification — validates package trust levels.
//!
//! Every package has a set of TrustLevel badges. This module verifies
//! those badges against the package metadata and optional signatures.
//! ExecutionController can enforce trust policies via `TrustPolicy`.

use crate::package_format::{RegistryPackage, TrustLevel};
use serde::{Deserialize, Serialize};

/// A trust policy defines what the ExecutionController requires
/// before it will execute genes from a given package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustPolicy {
    /// Minimum required trust level.
    pub min_trust: TrustLevel,
    /// Require the publisher to be verified.
    pub require_publisher_verified: bool,
    /// Require cryptographic signature.
    pub require_signed: bool,
    /// Require source code to be available.
    pub require_source: bool,
    /// Require reproducible build.
    pub require_reproducible: bool,
    /// Require security audit.
    pub require_audit: bool,
    /// Only allow packages from these publishers (empty = any).
    pub allow_publishers: Vec<String>,
    /// Block packages from these publishers.
    pub block_publishers: Vec<String>,
    /// Maximum allowed price (0 = free only).
    pub max_price_usd: f64,
}

impl Default for TrustPolicy {
    fn default() -> Self {
        Self {
            min_trust: TrustLevel::None,
            require_publisher_verified: false,
            require_signed: false,
            require_source: false,
            require_reproducible: false,
            require_audit: false,
            allow_publishers: Vec::new(),
            block_publishers: Vec::new(),
            max_price_usd: 0.0,
        }
    }
}

impl TrustPolicy {
    /// Strict: require Pandora verified + signed + free.
    pub fn strict() -> Self {
        Self {
            min_trust: TrustLevel::PandoraVerified,
            require_signed: true,
            max_price_usd: 0.0,
            ..Default::default()
        }
    }

    /// Lenient: allow anything from any publisher.
    pub fn permissive() -> Self {
        Self::default()
    }

    /// Evaluate a package against this policy.
    /// Returns Ok(()) if the package passes, Err(reason) if not.
    pub fn evaluate(&self, pkg: &RegistryPackage) -> Result<(), String> {
        // Check publisher allow/block lists
        if !self.allow_publishers.is_empty() && !self.allow_publishers.contains(&pkg.publisher) {
            return Err(format!("Publisher {} not in allow list", pkg.publisher));
        }
        if self.block_publishers.contains(&pkg.publisher) {
            return Err(format!("Publisher {} is blocked", pkg.publisher));
        }

        // Check price
        if self.max_price_usd == 0.0 && pkg.is_paid {
            return Err("Package is paid — free only policy".into());
        }
        if let Some(price) = pkg.price_usd {
            if price > self.max_price_usd {
                return Err(format!(
                    "Price ${price:.2} exceeds max ${:.2}",
                    self.max_price_usd
                ));
            }
        }

        // Check trust levels
        let has = |lvl: TrustLevel| pkg.trust_levels.contains(&lvl);

        if self.require_publisher_verified && !has(TrustLevel::PublisherVerified) {
            return Err("Publisher not verified".into());
        }
        if self.require_signed && !has(TrustLevel::Signed) {
            return Err("Package not signed".into());
        }
        if self.require_source && !has(TrustLevel::SourceAvailable) {
            return Err("Source not available".into());
        }
        if self.require_reproducible && !has(TrustLevel::ReproducibleBuild) {
            return Err("Build not reproducible".into());
        }
        if self.require_audit && !has(TrustLevel::SecurityAudited) {
            return Err("No security audit".into());
        }

        // Check minimum trust rank
        if pkg.top_trust().rank() < self.min_trust.rank() {
            return Err(format!(
                "Trust level {:?} (rank {}) below minimum {:?} (rank {})",
                pkg.top_trust(),
                pkg.top_trust().rank(),
                self.min_trust,
                self.min_trust.rank()
            ));
        }

        Ok(())
    }

    /// Evaluate and return a verdict with detailed info.
    pub fn verdict(&self, pkg: &RegistryPackage) -> TrustVerdict {
        match self.evaluate(pkg) {
            Ok(()) => TrustVerdict {
                passed: true,
                package_id: pkg.full_id(),
                top_trust: Some(pkg.top_trust()),
                badges: pkg.trust_badges(),
                reason: Some("All trust checks passed".into()),
            },
            Err(reason) => TrustVerdict {
                passed: false,
                package_id: pkg.full_id(),
                top_trust: Some(pkg.top_trust()),
                badges: pkg.trust_badges(),
                reason: Some(reason),
            },
        }
    }
}

/// The result of evaluating a trust policy against a package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustVerdict {
    pub passed: bool,
    pub package_id: String,
    pub top_trust: Option<TrustLevel>,
    pub badges: String,
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_format::PackageManifest;

    fn pkg(paid: bool, trust: Vec<TrustLevel>, publisher: &str) -> RegistryPackage {
        RegistryPackage {
            manifest: PackageManifest {
                id: "test".into(),
                name: "Test".into(),
                version: "1.0".into(),
                publisher: publisher.into(),
                ..Default::default()
            },
            publisher: publisher.into(),
            published_at: String::new(),
            downloads: 0,
            weekly_downloads: 0,
            stars: 0,
            verified: true,
            trust_levels: trust,
            signature: None,
            checksum_sha256: String::new(),
            archive_url: String::new(),
            github_repo: None,
            forked_from: None,
            forks: vec![],
            reviews: 0,
            success_rate: 0.0,
            avg_latency_ms: 0.0,
            changelog: None,
            is_paid: paid,
            price_usd: if paid { Some(9.99) } else { None },
        }
    }

    #[test]
    fn permissive_accepts_anything() {
        assert!(TrustPolicy::permissive()
            .evaluate(&pkg(false, vec![], "any"))
            .is_ok());
    }
    #[test]
    fn strict_rejects_unsigned() {
        assert!(TrustPolicy::strict()
            .evaluate(&pkg(false, vec![], "any"))
            .is_err());
    }
    #[test]
    fn strict_accepts_verified_signed() {
        assert!(TrustPolicy::strict()
            .evaluate(&pkg(
                false,
                vec![TrustLevel::PandoraVerified, TrustLevel::Signed],
                "pandora"
            ))
            .is_ok());
    }
    #[test]
    fn free_only_rejects_paid() {
        let pol = TrustPolicy {
            max_price_usd: 0.0,
            ..Default::default()
        };
        assert!(pol.evaluate(&pkg(true, vec![], "any")).is_err());
    }
    #[test]
    fn publisher_allow_list() {
        let pol = TrustPolicy {
            allow_publishers: vec!["pandora".into()],
            ..Default::default()
        };
        assert!(pol.evaluate(&pkg(false, vec![], "pandora")).is_ok());
        assert!(pol.evaluate(&pkg(false, vec![], "other")).is_err());
    }
    #[test]
    fn verdict_returns_package_id() {
        let v = TrustPolicy::permissive().verdict(&pkg(false, vec![], "pandora"));
        assert_eq!(v.package_id, "pandora/test");
        assert!(v.passed);
    }
}
