//! Design genes — UI/UX, animation, review, accessibility.
//!
//! Each gene implements a specific design capability. All share the same
//! pattern: receive a design prompt/task, return guidance or generated output.
//! ponytail: no design engines — we delegate to LLM knowledge + linting tools.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

fn mk(id: &str, kind: GeneKind) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id)
        .name(id)
        .kind(kind)
        .version("0.1.0")
        .author("pandora")
        .description(match id {
            "ui-design" => "UI/UX design patterns, layout, responsive design",
            "design-review" => "Design critique and review (impeccable + taste-skill inspired)",
            "css-animation" => "CSS animations, transitions, transforms (GSAP-inspired)",
            "motion-design" => "Motion design principles and animation (Seedance-inspired)",
            "typography" => "Typeface selection, hierarchy, readability",
            "color-theory" => "Color palettes, contrast, harmony",
            "a11y-design" => "Accessibility-first design (WCAG, ARIA, inclusive)",
            "design-system" => "Design system tokens, components, documentation",
            _ => "Design gene",
        })
        .build().unwrap()
}

macro_rules! design_gene {
    ($name:ident, $id:expr, $body:expr) => {
        #[derive(Debug)]
        pub struct $name { m: GeneManifest }
        impl $name {
            pub fn new() -> Self { Self { m: mk($id, GeneKind::Tool) } }
        }
        impl Gene for $name {
            fn id(&self) -> &str { &self.m.id }
            fn manifest(&self) -> &GeneManifest { &self.m }
            fn execute(&self, input: &str) -> Result<String, String> {
                if input.trim().is_empty() { return Err(format!("Usage: {} <prompt>", $id)); }
                Ok($body(input))
            }
        }
    };
}

design_gene!(UiDesignGene, "ui-design", |input: &str| {
    format!("UI/UX Design: {}
- Apply responsive layout principles
- Consider component hierarchy and spacing
- Follow established design patterns
- Ensure touch targets meet minimum sizes", input)
});

design_gene!(DesignReviewGene, "design-review", |input: &str| {
    format!("Design Review: {}
- Evaluate visual hierarchy and balance
- Check color contrast and readability
- Assess spacing and alignment consistency
- Review typography scale and hierarchy
- Identify accessibility issues
- Suggest concrete improvements", input)
});

design_gene!(CssAnimationGene, "css-animation", |input: &str| {
    format!("CSS Animation: {}
- Use CSS transitions for simple state changes
- Keyframe animations for complex sequences
- Consider performance (GPU-accelerated properties only)
- Respect prefers-reduced-motion
- Timeline: ease-in-out for natural movement", input)
});

design_gene!(MotionDesignGene, "motion-design", |input: &str| {
    format!("Motion Design: {}
- Define easing curves (ease-in-out for natural motion)
- Stagger animations for visual interest
- Duration: 200-500ms for UI, 500-1000ms for emphasis
- Use spring animations for natural feel
- Ensure motion tells a story, not just decoration", input)
});

design_gene!(TypographyGene, "typography", |input: &str| {
    format!("Typography: {}
- Establish type scale: 8/10/12/14/16/20/24/32/48/64
- Line height: 1.5 for body, 1.2 for headings
- Max line length: 60-75 characters
- Pair complementary typefaces (sans + serif or within same family)
- Consider variable font axes for fine-tuned control", input)
});

design_gene!(ColorTheoryGene, "color-theory", |input: &str| {
    format!("Color: {}
- Use 60-30-10 rule (dominant, secondary, accent)
- Ensure WCAG AA contrast (4.5:1 for text)
- Consider dark mode variants
- Limit palette to 3-5 core colors
- Test for color blindness (deuteranopia, protanopia)", input)
});

design_gene!(A11yDesignGene, "a11y-design", |input: &str| {
    format!("Accessibility: {}
- All interactive elements must be keyboard accessible
- Add ARIA labels where visual cues aren't sufficient
- Ensure focus indicators are visible (3:1 contrast ratio)
- Support screen readers with semantic HTML
- Test with VoiceOver / NVDA
- Respect forced-colors and prefers-reduced-motion", input)
});

design_gene!(DesignSystemGene, "design-system", |input: &str| {
    format!("Design System: {}
- Define design tokens (color, spacing, typography, shadows)
- Create component library with variants and states
- Document usage guidelines and examples
- Ensure consistency across platforms
- Version the system independently of products", input)
});
