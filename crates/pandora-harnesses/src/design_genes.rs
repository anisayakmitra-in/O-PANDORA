//! Design genes — studied from impeccable, taste-skill, UI UX Pro Max,
//! emilkowalski/skills, GSAP skills, Motion.dev, claudedesignskills.
//!
//! Each gene implements specific design domain knowledge extracted from
//! these repositories. The genes provide deterministic guidance and patterns
//! rather than generative design — they ensure AI-generated output meets
//! professional design standards.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};

fn mk(id: &str, kind: GeneKind, desc: &str) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id).name(id).kind(kind).version("0.1.0").author("pandora")
        .description(desc).build().unwrap()
}

// ── Design Review Gene (impeccable-inspired) ──
// impeccable: 46 deterministic detector rules, 23 commands, live browser iteration
#[derive(Debug)]
pub struct DesignReviewGene { m: GeneManifest }
impl DesignReviewGene {
    pub fn new() -> Self { Self { m: mk("design-review", GeneKind::Tool,
        "Design review with deterministic rules (impeccable-inspired)") } }
}
impl Gene for DesignReviewGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: describe a design to review".into()); }
        Ok(format!(
            "Design Review: {}
            
46 Deterministic Rules Check:
  Layout & Spacing:
  - Is there consistent 4/8px grid alignment?
  - Is content width constrained (60-80ch for text)?
  - Are touch targets ≥44x44px?
  
  Typography:
  - Is type scale hierarchical (12/14/16/20/24/32/48/64)?
  - Is line height adequate (1.5 body, 1.2 headings)?
  - Is max line length 60-75 characters?
  
  Color:
  - Is text contrast ≥4.5:1 (WCAG AA)?
  - Are there sufficient color signals (error, success, warning)?
  - Is there a clear visual hierarchy?
  
  Anti-Patterns Detected:
  - Inter font for everything? → Use system font stack or pair 2 typefaces
  - Purple-to-blue gradients on CTAs? → Use brand colors instead
  - Gray text on colored backgrounds? → Check contrast ratio
  - Cards nested in cards? → Simplify hierarchy
  - Rounded-square icon tile above heading? → Consider different pattern
  - Generic SaaS template layout? → Add distinct brand elements

Impeccable-inspired polish commands available:
  /polish - refine spacing, alignment, rhythm
  /distill - remove visual clutter
  /bolder - increase visual weight
  /quieter - reduce visual noise",
            input))
    }
}

// ── Brand Identity Gene (taste-skill inspired) ──
// taste-skill: brand kits, anti-slop frontend, premium design direction
#[derive(Debug)]
pub struct BrandIdentityGene { m: GeneManifest }
impl BrandIdentityGene {
    pub fn new() -> Self { Self { m: mk("brand-identity", GeneKind::Tool,
        "Brand identity kit creation (taste-skill inspired)") } }
}
impl Gene for BrandIdentityGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: brand name or description".into()); }
        Ok(format!(
            "Brand Identity Kit for: {}
            
Brand Strategy:
  - Define brand archetype (Innovator, Caregiver, Rebel, etc.)
  - Establish brand voice: tone, vocabulary, personality
  - Create brand anti-references (what you're NOT)
  
Visual Identity:
  - Primary palette: 3 colors (dominant, secondary, accent)
  - Neutral scale: 10 shades (50-950 in 100 increments)
  - Typography: 1 display + 1 body typeface
  - Logo variants: full, icon, favicon
  - Pattern library: 3-5 recurring visual motifs
  
Anti-Slop Rules (taste-skill):
  - Avoid generic SaaS templates
  - Every element must have a purpose
  - Typography must have hierarchy
  - Spacing must be intentional (not default)
  - Motion must serve function, not decoration
  - Color must guide, not just decorate
            
Brand Kit Deliverables:
  PRODUCT.md (impeccable-inspired brand doc)
  design-tokens.json
  component-library preview",
            input))
    }
}

// ── UI Patterns Gene (UI UX Pro Max + emilkowalski inspired) ──
// UI UX Pro Max: 161 reasoning rules, 67 UI styles, 20+ stacks
// emilkowalski: design engineer experience from Vercel/Linear
#[derive(Debug)]
pub struct UiPatternsGene { m: GeneManifest }
impl UiPatternsGene {
    pub fn new() -> Self { Self { m: mk("ui-patterns", GeneKind::Tool,
        "UI patterns and design system (UI UX Pro Max + emilkowalski-inspired)") } }
}
impl Gene for UiPatternsGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: describe the UI component or page".into()); }
        Ok(format!(
            "UI Design Patterns for: {input}
            
Style Selection (UI UX Pro Max):
  Variance dial 5: 1(minimal) → 10(bold)
  Motion dial 3: 1(static) → 10(animated)
  Density dial 5: 1(spacious) → 10(dense)
  
Recommended Style: {}
  
Design System Rules (161 rules):
  - Use semantic color tokens, not raw hex values
  - Maintain 4px spacing grid consistently
  - Type scale follows 1.25 ratio (Major Third)
  - All interactive elements have: default, hover, active, focus, disabled
  - Form inputs show: label, placeholder, help text, error, success states
  - Loading states: skeleton screens preferred over spinners
  - Empty states: illustration + message + action button
  
Component-Specific Patterns:
  Navigation: top bar (<5 items) | sidebar (5-15 items) | bottom tab (mobile)
  Data display: table (tabular) | cards (visual) | list (simple)
  Forms: single column preferred | group related fields | show progress
            
Stack-Adapted Output:
  Available stacks: html-tailwind, react, nextjs, astro, vue, svelte,
  shadcn, react-native, flutter, swiftui, jetpack-compose, threejs
            
Design Engineer Notes (emilkowalski):
  Every design decision should have a rationale
  Consistency > perfection
  Constraints enable creativity",
            input))
    }
}

// ── Animation Gene (GSAP + Motion.dev inspired) ──
// GSAP: core API, timelines, ScrollTrigger, performance (11k★)
// Motion.dev: 120fps spring physics, GPU-accelerated (10M+/month)
#[derive(Debug)]
pub struct AnimationGene { m: GeneManifest }
impl AnimationGene {
    pub fn new() -> Self { Self { m: mk("animation", GeneKind::Tool,
        "Animation patterns (GSAP + Motion.dev inspired)") } }
}
impl Gene for AnimationGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: describe the animation you want".into()); }
        Ok(format!(
            "Animation Spec for: {}
            
Animation Type:
  □ Entrance animation (fade, slide, scale)
  □ Gesture response (hover, tap, drag)
  □ Scroll-driven (parallax, reveal, progress)
  □ Layout transition (list, grid, page)
  □ Micro-interaction (button, toggle, notification)
  
GSAP Approach:
  Use gsap.to() for simple tweens, gsap.timeline() for sequences
  Always animate transform/opacity for 60fps performance
  Easing: power2.out (natural), back.out(1.7) (bouncy)
  Duration: 0.3s (micro) | 0.5s (standard) | 0.8s (emphatic)
  
  Timeline pattern:
  ```javascript
  const tl = gsap.timeline({{ defaults: {{ duration: 0.5, ease: \"power2.out\" }} }});
  tl.from(\".title\", {{ y: 30, opacity: 0 }})
    .from(\".content\", {{ y: 20, opacity: 0 }}, \"-=0.3\")
    .from(\".cta\", {{ scale: 0.8 }}, \"-=0.2\");
  ```
  
Motion.dev Approach (120fps Spring Physics):
  Use spring() for natural-feeling motion
  Defaults: stiffness=300, damping=20, mass=1
  GPU-accelerated: animate transform, opacity, filter
  
  ```tsx
  import {{ motion }} from \"motion/react\"
  <motion.div
    initial={{ opacity: 0, y: 20 }}
    animate={{ opacity: 1, y: 0 }}
    transition={{ type: \"spring\", stiffness: 300, damping: 20 }}
  />
  ```
  
Scroll Animation:
  GSAP ScrollTrigger: scrub, pin, markers for debugging
  Motion.dev: useInView, scroll animations
  Always check prefers-reduced-motion
  
Performance Rules (GSAP):
  ✅ Animate transform and opacity
  ✅ Use stagger instead of separate tweens
  ✅ Use gsap.quickTo() for mouse followers
  ❌ Avoid animating width/height/top/left
  ❌ Don't set will-change on every element
  ❌ Clean up off-screen animations",
            input))
    }
}

// ── Color Theory Gene (UI UX Pro Max + design systems) ──
#[derive(Debug)]
pub struct ColorTheoryGene { m: GeneManifest }
impl ColorTheoryGene {
    pub fn new() -> Self { Self { m: mk("color-theory", GeneKind::Tool,
        "Color palette generation and theory") } }
}
impl Gene for ColorTheoryGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: base color or brand name".into()); }
        Ok(format!(
            "Color Palette for: {}
            
60-30-10 Rule:
  - 60%: Neutral/background (white, off-white, dark gray)
  - 30%: Primary brand color (main UI elements)
  - 10%: Accent color (CTAs, highlights, key actions)
  
WCAG Compliance:
  - AA Normal text: 4.5:1 contrast ratio
  - AA Large text: 3:1 contrast ratio
  - AAA Normal text: 7:1 contrast ratio
  - Focus indicators: 3:1 minimum
  
Generated Palette:
  Primary: {} + shades (50-950)
  Neutrals: slate/warm gray scale
  Semantic:
    Success: green-600 (#059669)
    Warning: amber-500 (#F59E0B)
    Error: red-600 (#DC2626)
    Info: blue-500 (#3B82F6)
  
Color Usage Rules:
  - Text must pass WCAG AA on its background
  - Don't rely on color alone for information
  - Consider dark mode variants
  - Test for deuteranopia/protanopia
  - Brand colors for primary actions only",
            input, input))
    }
}

// ── Typography Gene ──
#[derive(Debug)]
pub struct TypographyGene { m: GeneManifest }
impl TypographyGene {
    pub fn new() -> Self { Self { m: mk("typography", GeneKind::Tool,
        "Typography system and font pairings") } }
}
impl Gene for TypographyGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: describe your brand or project".into()); }
        Ok(format!(
            "Typography System for: {}
            
Type Scale (Major Third 1.25):
  Display: 64/48/32px
  Heading: 24/20/16px
  Body:    14/12/10px
  
Recommended Pairings:
  Modern/SaaS: Inter (headings) + Inter (body) — same family
  Editorial: Playfair Display + Inter — serif + sans
  Technical: JetBrains Mono + Inter — monospace + sans
  Creative: DM Sans + DM Serif Display — geometric + serif
  Luxury: Cormorant Garamond + Montserrat — elegant + clean
  
Implementation Rules:
  - Line height: 1.5 for body, 1.2 for headings
  - Max line length: 60-75 characters (aim for 66)
  - Letter spacing: 0.02em for uppercase, 0 for body
  - Font weight: 400 body, 500 strong, 600 subheadings, 700 headings
  - Responsive: scale down by 2px on mobile
  
Common Mistakes:
  - Using too many typefaces (max 2)
  - Insufficient contrast between heading and body sizes
  - Line too long or too short (impacts readability)
  - Ignoring vertical rhythm / baseline grid",
            input))
    }
}

// ── Accessibility Design Gene ──
#[derive(Debug)]
pub struct A11yDesignGene { m: GeneManifest }
impl A11yDesignGene {
    pub fn new() -> Self { Self { m: mk("a11y-design", GeneKind::Tool,
        "Accessibility-first design (WCAG 2.2)") } }
}
impl Gene for A11yDesignGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: describe the component or page".into()); }
        Ok(format!(
            "Accessibility Audit for: {}
            
WCAG 2.2 Compliance Check:
  Perceivable:
  - All images have alt text
  - Color is not the only means of conveying info
  - Captions provided for audio/video
  - Text contrast ≥4.5:1 (AA) or 7:1 (AAA)
  
  Operable:
  - All interactive elements keyboard accessible
  - Focus indicators visible (3:1 contrast)
  - Touch targets ≥44x44px
  - No keyboard traps
  - Motion: respects prefers-reduced-motion
  
  Understandable:
  - Language attribute set on HTML
  - Form inputs have associated labels
  - Error messages are descriptive
  - Consistent navigation across pages
  
  Robust:
  - Semantic HTML elements used
  - ARIA landmarks for page structure
  - ARIA labels where visual cues insufficient
  - Tested with screen reader (VoiceOver/NVDA)
  
Implementation:
  ```html
  <!-- Proper button -->
  <button onClick={{handleClick}} aria-label=\"Close dialog\">
    <XIcon aria-hidden=\"true\" />
  </button>
  
  <!-- Skip link -->
  <a href=\"#main-content\" class=\"skip-link\">
    Skip to content
  </a>
  ```
  
Anti-Patterns:
  - Div/span used as interactive elements without ARIA
  - Placeholder as label replacement
  - Auto-playing video without controls
  - Timed interactions without warning",
            input))
    }
}

// ── Motion Design Gene (Seedance/motion inspired) ──
#[derive(Debug)]
pub struct MotionDesignGene { m: GeneManifest }
impl MotionDesignGene {
    pub fn new() -> Self { Self { m: mk("motion-design", GeneKind::Tool,
        "Motion design principles (Seedance, Motion.dev)") } }
}
impl Gene for MotionDesignGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: describe the motion you want".into()); }
        Ok(format!(
            "Motion Design: {}
            
Principles (Apple/Jon Ive):
  Purposeful: every animation serves a function
  Smooth: 60-120fps target
  Accessible: respects reduced-motion
  Performant: GPU-accelerated properties only
  Subtle: 200-500ms for UI, 500-1000ms for emphasis
  Consistent: unified timing and easing system
  
Spring Physics (Motion.dev/Seedance-inspired):
  Stiffness: 300-400 (controls speed)
  Damping: 20-30 (controls bounce)
  Mass: 1 (controls weight)
  
  Light spring:    stiffness=200, damping=25
  Natural spring:  stiffness=300, damping=20
  Bouncy spring:   stiffness=400, damping=10
  Heavy spring:    stiffness=600, damping=35
  
Animation Types:
  Entrance: elements fade/slide in (200-500ms)
  Exit: elements fade/slide out (150-300ms)
  Emphasis: scale pulse, shimmer (500-1000ms)
  Transition: layout change, page transition (300-500ms)
  Micro: hover, active, focus (100-200ms)
  
Stagger Patterns:
  List: items stagger by 50-100ms each
  Page: sections stagger by 100-200ms each
  Cards: grid items stagger by 30-50ms each
  
Storyboard:
  1. Title animates in (0ms, duration 500ms)
  2. Content fades up (300ms, duration 400ms)
  3. CTA scales in (600ms, duration 300ms)
  4. Background parallax (scroll-driven)",
            input))
    }
}
