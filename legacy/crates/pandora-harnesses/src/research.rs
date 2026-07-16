#![allow(clippy::new_without_default)]
//! Research Domain Harness — scientific research, literature, experiments.
//! Skills from: Orchestra-Research/AI-Research-SKILLs, agent-research-skills, K-Dense-AI

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]

pub struct ResearchDomainHarness {
    manifest: HarnessManifest,
}

impl ResearchDomainHarness {
    pub fn new() -> Self {
        Self { manifest: HarnessManifestBuilder::default()
            .id("research-domain").name("Research Domain").version("0.2.0").author("pandora")
            .kind(HarnessKind::Domain)
            .description("Literature review, experiment design, data analysis, paper writing, deep research")
            .capability("research").capability("literature").capability("experiment")
            .build().unwrap() }
    }
}
impl Harness for ResearchDomainHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.manifest
    }
}

fn mk(id: &str, desc: &str) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id)
        .name(desc)
        .kind(GeneKind::Tool)
        .version("0.1.0")
        .author("pandora")
        .description(desc)
        .build()
        .unwrap()
}

macro_rules! research_gene {
    ($name:ident, $id:expr, $desc:expr) => {
        #[derive(Debug)]
        pub struct $name {
            m: GeneManifest,
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $name {
            pub fn new() -> Self {
                Self { m: mk($id, $desc) }
            }
        }
        impl Gene for $name {
            fn manifest(&self) -> &GeneManifest {
                &self.m
            }
            fn execute(&self, _input: &str) -> Result<String, String> {
                Ok(format!("{}: research started", $id))
            }
        }
    };
}

// ── Research Genes (Orchestra + agent-research + K-Dense patterns) ──

// Literature & Discovery
research_gene!(
    LiteratureReviewGene,
    "lit-review",
    "Systematic literature review — search, filter, synthesize"
);
research_gene!(
    LiteratureSearchGene,
    "lit-search",
    "Targeted literature search across arXiv, PubMed, Semantic Scholar"
);
research_gene!(
    DeepResearchGene,
    "deep-research",
    "Multi-source deep research — recursive exploration with source tracking"
);
research_gene!(
    GithubResearchGene,
    "github-research",
    "Research GitHub repos — stars, forks, activity, code quality analysis"
);

// Experiment & Data
research_gene!(
    ExperimentDesignGene,
    "experiment-design",
    "Design experiments — hypothesis, variables, controls, power analysis"
);
research_gene!(
    ExperimentCodeGene,
    "experiment-code",
    "Generate experiment code — data loading, training loops, metrics"
);
research_gene!(
    DataAnalysisGene,
    "data-analysis",
    "Statistical analysis — distributions, correlations, significance tests"
);
research_gene!(
    FigureGenerationGene,
    "figure-gen",
    "Generate publication-quality figures and visualizations"
);
research_gene!(
    TableGenerationGene,
    "table-gen",
    "Generate formatted tables for academic papers"
);

// Writing & Publication
research_gene!(
    PaperAssemblyGene,
    "paper-assembly",
    "Assemble paper from sections — abstract, intro, methods, results"
);
research_gene!(
    PaperWritingGene,
    "paper-section",
    "Write specific paper sections — methods, results, discussion"
);
research_gene!(
    RelatedWorkGene,
    "related-work",
    "Write related work section — compare and contrast existing approaches"
);
research_gene!(
    RebuttalWritingGene,
    "rebuttal",
    "Write reviewer rebuttals — structured responses with evidence"
);
research_gene!(
    NoveltyAssessmentGene,
    "novelty",
    "Assess novelty — compare against existing literature, find gaps"
);
research_gene!(
    LatexFormattingGene,
    "latex",
    "Format paper for LaTeX — citations, figures, tables, templates"
);

// Planning & Ideation
research_gene!(
    ResearchPlanningGene,
    "research-plan",
    "Plan research roadmap — milestones, experiments, papers"
);
research_gene!(
    IdeaGenerationGene,
    "idea-gen",
    "Generate research ideas — gap analysis, brainstorming, feasibility"
);
research_gene!(
    SurveyGenerationGene,
    "survey-gen",
    "Generate comprehensive survey papers with structured taxonomy"
);
research_gene!(
    SlidesGene,
    "slides",
    "Generate presentation slides from paper — key points, figures, narrative"
);

// Math & Reasoning
research_gene!(
    MathReasoningGene,
    "math-reason",
    "Mathematical reasoning — proofs, derivations, symbolic computation"
);
research_gene!(
    SymbolicEquationGene,
    "symbolic-eq",
    "Symbolic equation manipulation and LaTeX rendering"
);

// Atomic Decomposition & Traceability
research_gene!(
    AtomicDecompGene,
    "atomic-decomp",
    "Decompose complex claims into atomic verifiable statements"
);
research_gene!(
    CitationGene,
    "citation-mgmt",
    "Citation management — BibTeX, tracking, verification"
);
research_gene!(
    BackwardTraceGene,
    "backward-trace",
    "Backward traceability — map conclusions to supporting evidence"
);

// K-Dense Scientific Skills
research_gene!(
    BioPythonGene,
    "biopython",
    "BioPython — sequence analysis, BLAST, phylogenetics"
);
research_gene!(
    AnnDataGene,
    "anndata",
    "AnnData — single-cell data analysis, scanpy integration"
);
research_gene!(
    BenchlingGene,
    "benchling",
    "Benchling integration — lab notebooks, sequence design"
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn research_domain() {
        assert_eq!(
            ResearchDomainHarness::new().manifest().id,
            "research-domain"
        );
    }
    #[test]
    fn lit_review() {
        assert!(!LiteratureReviewGene::new().manifest().id.is_empty());
    }
    #[test]
    fn idea_gen() {
        assert!(!IdeaGenerationGene::new().manifest().id.is_empty());
    }
}
