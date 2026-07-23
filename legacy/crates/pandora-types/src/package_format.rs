//! Package format — `pandora.toml` manifest and archive types.
//!
//! Every distributable unit follows this format. Namespaced like GitHub:
//! `publisher/package-name` (e.g. `pandora/coding-domain`, `sayak/eda-skill`).
//!
//! ```text
//! pandora.toml       ← manifest (required)
//! genes/             ← gene implementations
//! harnesses/         ← harness implementations
//! evaluators/        ← evaluator implementations
//! skills/            ← skill bundles
//! profiles/          ← execution profiles (.toml)
//! plans/             ← execution plans (.toml)
//! assets/            ← documentation, icons, README, LICENSE
//! ```

use serde::{Deserialize, Serialize};

/// A namespace-prefixed package identifier: `publisher/package`.
/// Empty publisher means the default namespace.
#[inline]
pub fn package_id(publisher: &str, name: &str) -> String {
    if publisher.is_empty() {
        name.to_string()
    } else {
        format!("{publisher}/{name}")
    }
}

/// Trust level assigned to a package.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TrustLevel {
    #[default]
    None,
    /// Publisher is verified by Palace.
    PublisherVerified,
    /// Package is cryptographically signed.
    Signed,
    /// Source code is publicly available.
    SourceAvailable,
    /// Build is reproducible from source.
    ReproducibleBuild,
    /// Independently security audited.
    SecurityAudited,
    /// Full trust: verified + signed + source + reproducible + audited.
    PandoraVerified,
}

impl TrustLevel {
    pub fn badge(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::PublisherVerified => "✓ Publisher",
            Self::Signed => "🔏 Signed",
            Self::SourceAvailable => "📂 Source",
            Self::ReproducibleBuild => "🔁 Reproducible",
            Self::SecurityAudited => "🛡 Audited",
            Self::PandoraVerified => "🏷 Pandora Verified",
        }
    }
    pub fn rank(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::PublisherVerified => 1,
            Self::Signed => 2,
            Self::SourceAvailable => 3,
            Self::ReproducibleBuild => 4,
            Self::SecurityAudited => 5,
            Self::PandoraVerified => 6,
        }
    }
}

/// The canonical package manifest — `pandora.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PackageManifest {
    /// Package identifier without namespace (e.g. "coding-domain").
    /// The full ID is `publisher/id`.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Semantic version.
    pub version: String,
    /// Package kind.
    #[serde(default)]
    pub kind: PackageKind,
    /// Publisher namespace (e.g. "pandora", "sayak").
    #[serde(default)]
    pub publisher: String,
    /// Human-readable description.
    #[serde(default)]
    pub description: String,
    /// Author display name.
    #[serde(default)]
    pub author: String,
    /// SPDX license identifier.
    #[serde(default)]
    pub license: String,
    /// GitHub repository URL.
    #[serde(default)]
    pub repository: String,
    /// Documentation URL.
    #[serde(default)]
    pub documentation: String,
    /// Homepage / website URL.
    #[serde(default)]
    pub homepage: String,
    /// Minimum Pandora runtime version required.
    #[serde(default)]
    pub pandora_version: String,
    pub lifecycle: PackageLifecycle,
    /// Tags for discovery.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Categories for discovery.
    #[serde(default)]
    pub categories: Vec<String>,
    /// Package dependencies (namespace/id@version).
    #[serde(default)]
    pub dependencies: Vec<PackageDependency>,
    /// Genes provided.
    #[serde(default)]
    pub genes: Vec<GeneEntry>,
    /// Harnesses provided.
    #[serde(default)]
    pub harnesses: Vec<HarnessEntry>,
    /// Evaluators provided.
    #[serde(default)]
    pub evaluators: Vec<EvaluatorEntry>,
    /// Skills provided.
    #[serde(default)]
    pub skills: Vec<SkillEntry>,
    /// Profiles provided.
    #[serde(default)]
    pub profiles: Vec<String>,
    /// Plans provided.
    #[serde(default)]
    pub plans: Vec<PlanEntry>,
    /// Average execution success rate.
    #[serde(default)]
    pub success_rate: f64,
    /// Forked from another package (namespace/id).
    #[serde(default)]
    pub forked_from: Option<String>,
}

impl PackageManifest {
    pub fn full_id(&self) -> String {
        package_id(&self.publisher, &self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum PackageKind {
    #[default]
    Skill,
    SourceHarness,
    MetaHarness,
    DomainHarness,
    Gene,
    Evaluator,
    Profile,
    Plan,
    Theme,
    Template,
    Bundle,
}
impl PackageKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Skill => "skill",
            Self::SourceHarness => "source-harness",
            Self::MetaHarness => "meta-harness",
            Self::DomainHarness => "domain-harness",
            Self::Gene => "gene",
            Self::Evaluator => "evaluator",
            Self::Profile => "profile",
            Self::Plan => "plan",
            Self::Theme => "theme",
            Self::Template => "template",
            Self::Bundle => "bundle",
        }
    }
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Skill => "🧩",
            Self::SourceHarness => "🔌",
            Self::MetaHarness => "🔗",
            Self::DomainHarness => "🌐",
            Self::Gene => "🧬",
            Self::Evaluator => "✅",
            Self::Profile => "⚙️",
            Self::Plan => "📋",
            Self::Theme => "🎨",
            Self::Template => "📄",
            Self::Bundle => "📦",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    pub id: String,
    #[serde(default)]
    pub version_req: String,
    #[serde(default)]
    pub optional: bool,
}
impl PackageDependency {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version_req: "*".into(),
            optional: false,
        }
    }
    pub fn version(mut self, req: impl Into<String>) -> Self {
        self.version_req = req.into();
        self
    }
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub path: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessEntry {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub description: String,
    #[serde(default)]
    pub path: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatorEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub path: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub components: Vec<String>,
    #[serde(default)]
    pub path: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub path: String,
}

/// Registry metadata — what Palace stores about a package.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistryPackage {
    pub manifest: PackageManifest,
    pub publisher: String,
    pub published_at: String,
    pub downloads: u64,
    pub weekly_downloads: u64,
    pub stars: u64,
    pub verified: bool,
    pub trust_levels: Vec<TrustLevel>,
    pub signature: Option<String>,
    pub checksum_sha256: String,
    pub archive_url: String,
    /// GitHub repository linked to this package.
    pub github_repo: Option<String>,
    /// Forked from this package (namespace/id).
    pub forked_from: Option<String>,
    /// Packages that derive from this one.
    pub forks: Vec<String>,
    /// Review count.
    pub reviews: u64,
    /// Average execution success rate from telemetry.
    pub success_rate: f64,
    /// Average execution latency in ms.
    pub avg_latency_ms: f64,
    /// Changelog URL or content.
    pub changelog: Option<String>,
    /// Package is paid (requires license).
    pub is_paid: bool,
    /// Price in USD if paid.
    pub price_usd: Option<f64>,
}

impl RegistryPackage {
    pub fn full_id(&self) -> String {
        package_id(&self.publisher, &self.manifest.id)
    }
    pub fn trust_badges(&self) -> String {
        self.trust_levels
            .iter()
            .map(|t| t.badge())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("  ")
    }
    pub fn top_trust(&self) -> TrustLevel {
        self.trust_levels
            .iter()
            .max_by_key(|t| t.rank())
            .copied()
            .unwrap_or(TrustLevel::None)
    }
}

/// Rich search result from Palace.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PalaceListing {
    pub full_id: String,
    pub name: String,
    pub version: String,
    pub kind: PackageKind,
    pub description: String,
    pub publisher: String,
    pub downloads: u64,
    pub weekly_downloads: u64,
    pub stars: u64,
    pub verified: bool,
    pub trust_levels: Vec<TrustLevel>,
    pub tags: Vec<String>,
    pub success_rate: f64,
    pub avg_latency_ms: f64,
    pub is_paid: bool,
    pub price_usd: Option<f64>,
    pub updated_at: String,
}

/// Trending period for discovery.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TrendingPeriod {
    Week,
    Month,
    AllTime,
}

/// A publisher / organization profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PublisherProfile {
    pub id: String,
    pub display_name: String,
    pub bio: String,
    pub avatar_url: String,
    pub joined_at: String,
    pub verified: bool,
    pub package_count: usize,
    pub total_downloads: u64,
    pub followers: u64,
    pub following: u64,
    pub organizations: Vec<String>,
    pub github_username: Option<String>,
}

/// Lineage entry — records where a package came from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEntry {
    pub from_id: String,
    pub to_id: String,
    pub action: String,
    pub at: String,
    pub description: String,
}

// Re-exports for convenience

/// Search filters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    pub q: Option<String>,
    pub kind: Option<String>,
    pub publisher: Option<String>,
    pub verified: Option<bool>,
    pub min_installs: Option<u64>,
    pub tags: Vec<String>,
    pub sort: Option<String>,
    pub free_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalaceUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub joined_at: String,
    pub tier: AccountTier,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum AccountTier {
    #[default]
    Free,
    Pro,
    Enterprise,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    pub manifest: PackageManifest,
    pub archive_base64: String,
    pub signature: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResponse {
    pub id: String,
    pub version: String,
    pub url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}

/// Package lifecycle — like Cargo/NPM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PackageLifecycle {
    #[default]
    Draft,
    Published,
    Deprecated,
    Archived,
    Superseded,
    Broken,
    Yanked,
}
impl PackageLifecycle {
    pub fn can_install(&self) -> bool {
        matches!(self, Self::Draft | Self::Published)
    }
}
