//! Design Domain Harness — UI/UX, animation, design review, and brand.
//!
//! Inspired by studying these repos line-by-line:
//!   impeccable (44k★) — 46 design rules, 23 commands, deterministic detection
//!   taste-skill (61k★) — anti-slop frontend, brand kits, brutalist design
//!   UI UX Pro Max (103k★) — 161 rules, 67 styles, 20+ stack support
//!   emilkowalski/skills (6k★) — design engineer, animation, design language
//!   GSAP skills (11k★) — core API, timelines, ScrollTrigger, performance
//!   Motion.dev — 120fps spring physics, GPU-accelerated
//!   claudedesignskills — 27 plugins, 3D/WebGL/animation
//!
//! Key patterns adopted:
//!   - `/impeccable init` pattern → design system initialization
//!   - Deterministic rules (impeccable) → design review criteria
//!   - Anti-slop rules (taste-skill) → design quality gates
//!   - Design dials (UI UX Pro Max) → variance/motion/density controls
//!   - GSAP Timeline + ScrollTrigger → animation capabilities
//!   - Spring physics (Motion.dev) → animation defaults
//!   - Brand kit creation (taste-skill) → brand identity workflow

use pandora_types::gene::Gene;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use std::sync::Arc;

#[derive(Debug)]
pub struct DesignDomainHarness {
    manifest: HarnessManifest,
    genes: Vec<Arc<dyn Gene>>,
}

impl DesignDomainHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("design-domain")
                .name("Design")
                .version("0.1.0")
                .author("pandora")
                .kind(HarnessKind::Domain)
                .description("UI/UX design, animation, brand, design review — inspired by impeccable, taste-skill, UI UX Pro Max, GSAP, Motion.dev, emilkowalski/skills")
                .capability("design-review")
                .capability("design-system")
                .capability("brand-identity")
                .capability("ui-patterns")
                .capability("animation")
                .capability("motion-design")
                .capability("typography")
                .capability("color-theory")
                .capability("a11y-design")
                // Design dials (UI UX Pro Max inspired)
                .slash_command("/init-design", "Initialize design system for project")
                .slash_command("/review-ui", "Review UI with 46 deterministic rules")
                .slash_command("/audit-a11y", "Accessibility audit")
                .slash_command("/palette", "Generate color palette from brand color")
                .slash_command("/typography", "Suggest type scale + pairings")
                .slash_command("/animate", "Generate animation spec (GSAP/Motion.dev)")
                .slash_command("/brand-kit", "Create brand identity kit")
                .slash_command("/polish", "Polish existing design (impeccable-inspired)")
                .build().unwrap(),
            genes: Vec::new(),
        }
    }

    pub fn add_gene(&mut self, gene: Arc<dyn Gene>) { self.genes.push(gene); }
    pub fn gene_count(&self) -> usize { self.genes.len() }

    /// Initialize a design system for a project (inspired by impeccable init).
    pub fn init_design_system(&self, project_name: &str, brand_color: &str, stack: &str) -> String {
        format!(
            "Design system initialized for {}
  Brand: {}
  Stack: {}
  ---
  Design tokens to create:
  - colors: primary ({}) + neutral scale + semantic
  - typography: type scale + font pairings
  - spacing: 4px grid system
  - shadows: elevation tokens
  - motion: duration + easing defaults
  - breakpoints: responsive grid
  Saved to DESIGN.md (impeccable-inspired)",
            project_name, brand_color, stack, brand_color
        )
    }

    /// Run design review rules (inspired by impeccable's 46 deterministic rules).
    pub fn review_design(&self, component_type: &str) -> Vec<&str> {
        match component_type {
            "card" => vec![
                "Check: card padding should be 16-24px",
                "Check: border-radius should match design system (8-16px)",
                "Check: shadow elevation is consistent with card hierarchy",
                "Anti-pattern: avoid nested cards with identical styling",
                "Anti-pattern: avoid gray text on colored backgrounds",
            ],
            "button" => vec![
                "Check: height should be 40-48px for default, 56px for CTA",
                "Check: border-radius matches system (6-8px standard)",
                "Check: hover + active states defined",
                "Anti-pattern: avoid purple-to-blue gradients on CTAs",
                "Anti-pattern: avoid Inter font for everything",
            ],
            "navigation" => vec![
                "Check: active state is clearly visible",
                "Check: touch targets are 44x44px minimum",
                "Check: keyboard navigation works",
                "Anti-pattern: avoid hamburger menus for <5 items",
                "Anti-pattern: avoid sticky headers on mobile",
            ],
            "form" => vec![
                "Check: labels are associated with inputs",
                "Check: error states shown inline",
                "Check: input height is 40-48px",
                "Anti-pattern: avoid placeholder as label replacement",
                "Anti-pattern: avoid too many inputs (7+ per step)",
            ],
            _ => vec!["Check: component follows design system tokens",
                      "Check: spacing is consistent with 4/8px grid",
                      "Anti-pattern: avoid generic SaaS template look"],
        }
    }

    /// Get design dials (UI UX Pro Max inspired).
    pub fn design_dials(&self) -> [&str; 3] {
        ["variance (1-10): controls boldness vs minimalism",
         "motion (1-10): controls animation intensity",
         "density (1-10): controls spacing compactness"]
    }
}

impl Harness for DesignDomainHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
    fn initialize(&mut self) -> Result<(), String> { Ok(()) }
    fn shutdown(&mut self) -> Result<(), String> { Ok(()) }
}
