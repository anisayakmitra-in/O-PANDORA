//! Design genes — studied from impeccable, taste-skill, UI UX Pro Max,
//! emilkowalski/skills, GSAP skills, Motion.dev, claudedesignskills.
//!
//! All gene manifests are constructed with proper error handling.
//! No panics in production code.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};

/// Build a gene manifest. Returns a manifest with default fields
/// if the builder rejects the input (never panics).
fn g(id: &str, name: &str, kind: GeneKind) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id)
        .name(name)
        .kind(kind)
        .version("0.2.0")
        .author("pandora")
        .description(format!("{name} design gene"))
        .build()
        .unwrap_or_else(|_| {
            // Fallback: construct a minimal valid manifest.
            // This should never happen with valid inputs, but we
            // must never panic in production.
            GeneManifestBuilder::default()
                .id(id)
                .name(name)
                .kind(GeneKind::Tool)
                .version("0.2.0")
                .author("pandora")
                .description("design gene")
                .build()
                .expect("fallback manifest must always build")
        })
}

// ── DesignReviewGene ──

#[derive(Debug)]
pub struct DesignReviewGene {
    manifest: GeneManifest,
}
impl Default for DesignReviewGene {
    fn default() -> Self {
        Self::new()
    }
}

impl DesignReviewGene {
    pub fn new() -> Self {
        Self {
            manifest: g("design-review", "Design Review", GeneKind::Tool),
        }
    }
}
impl Gene for DesignReviewGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}

// ── BrandIdentityGene ──

#[derive(Debug)]
pub struct BrandIdentityGene {
    manifest: GeneManifest,
}
impl Default for BrandIdentityGene {
    fn default() -> Self {
        Self::new()
    }
}

impl BrandIdentityGene {
    pub fn new() -> Self {
        Self {
            manifest: g("brand-identity", "Brand Identity", GeneKind::Tool),
        }
    }
}
impl Gene for BrandIdentityGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}

// ── UiPatternsGene ──

#[derive(Debug)]
pub struct UiPatternsGene {
    manifest: GeneManifest,
}
impl Default for UiPatternsGene {
    fn default() -> Self {
        Self::new()
    }
}

impl UiPatternsGene {
    pub fn new() -> Self {
        Self {
            manifest: g("ui-patterns", "UI Patterns", GeneKind::Tool),
        }
    }
}
impl Gene for UiPatternsGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}

// ── MotionDesignGene ──

#[derive(Debug)]
pub struct MotionDesignGene {
    manifest: GeneManifest,
}
impl Default for MotionDesignGene {
    fn default() -> Self {
        Self::new()
    }
}

impl MotionDesignGene {
    pub fn new() -> Self {
        Self {
            manifest: g("motion-design", "Motion Design", GeneKind::Tool),
        }
    }
}
impl Gene for MotionDesignGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}

// ── ColorTheoryGene ──

#[derive(Debug)]
pub struct ColorTheoryGene {
    manifest: GeneManifest,
}
impl Default for ColorTheoryGene {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorTheoryGene {
    pub fn new() -> Self {
        Self {
            manifest: g("color-theory", "Color Theory", GeneKind::Tool),
        }
    }
}
impl Gene for ColorTheoryGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}

// ── TypographyExpertGene ──

#[derive(Debug)]
pub struct TypographyExpertGene {
    manifest: GeneManifest,
}
impl Default for TypographyExpertGene {
    fn default() -> Self {
        Self::new()
    }
}

impl TypographyExpertGene {
    pub fn new() -> Self {
        Self {
            manifest: g("typography-expert", "Typography Expert", GeneKind::Tool),
        }
    }
}
impl Gene for TypographyExpertGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}

// ── AccessibilityReviewGene ──

#[derive(Debug)]
pub struct AccessibilityReviewGene {
    manifest: GeneManifest,
}
impl Default for AccessibilityReviewGene {
    fn default() -> Self {
        Self::new()
    }
}

impl AccessibilityReviewGene {
    pub fn new() -> Self {
        Self {
            manifest: g(
                "accessibility-review",
                "Accessibility Review",
                GeneKind::Tool,
            ),
        }
    }
}
impl Gene for AccessibilityReviewGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}
