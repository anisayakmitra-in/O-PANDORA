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
            .id("research-domain").name("Research Domain").version(env!("CARGO_PKG_VERSION")).author("pandora")
            .kind(HarnessKind::Domain)
            .description("Literature review, experiment design, data analysis, paper writing, deep research")
            .capability("research").capability("literature").capability("experiment")
            .owned_gene("lit-review")
            .owned_gene("lit-search")
            .owned_gene("deep-research")
            .owned_gene("github-research")
            .owned_gene("experiment-design")
            .owned_gene("experiment-code")
            .owned_gene("data-analysis")
            .owned_gene("figure-gen")
            .owned_gene("table-gen")
            .owned_gene("paper-assembly")
            .owned_gene("paper-section")
            .owned_gene("related-work")
            .owned_gene("rebuttal")
            .owned_gene("novelty")
            .owned_gene("latex")
            .owned_gene("research-plan")
            .owned_gene("idea-gen")
            .owned_gene("survey-gen")
            .owned_gene("slides")
            .owned_gene("math-reason")
            .owned_gene("symbolic-eq")
            .owned_gene("atomic-decomp")
            .owned_gene("citation-mgmt")
            .owned_gene("backward-trace")
            .owned_gene("biopython")
            .owned_gene("anndata")
            .owned_gene("benchling")
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
        .version(env!("CARGO_PKG_VERSION"))
        .author("pandora")
        .description(desc)
        .capability("research")
        .owner_harness("research-domain")
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
            fn execute(&self, _input: &str) -> Result<String, pandora_types::PandoraError> {
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

pub fn preloaded_genes() -> Vec<Box<dyn Gene>> {
    vec![
        Box::new(LiteratureReviewGene::new()),
        Box::new(LiteratureSearchGene::new()),
        Box::new(DeepResearchGene::new()),
        Box::new(GithubResearchGene::new()),
        Box::new(ExperimentDesignGene::new()),
        Box::new(ExperimentCodeGene::new()),
        Box::new(DataAnalysisGene::new()),
        Box::new(FigureGenerationGene::new()),
        Box::new(TableGenerationGene::new()),
        Box::new(PaperAssemblyGene::new()),
        Box::new(PaperWritingGene::new()),
        Box::new(RelatedWorkGene::new()),
        Box::new(RebuttalWritingGene::new()),
        Box::new(NoveltyAssessmentGene::new()),
        Box::new(LatexFormattingGene::new()),
        Box::new(ResearchPlanningGene::new()),
        Box::new(IdeaGenerationGene::new()),
        Box::new(SurveyGenerationGene::new()),
        Box::new(SlidesGene::new()),
        Box::new(MathReasoningGene::new()),
        Box::new(SymbolicEquationGene::new()),
        Box::new(AtomicDecompGene::new()),
        Box::new(CitationGene::new()),
        Box::new(BackwardTraceGene::new()),
        Box::new(BioPythonGene::new()),
        Box::new(AnnDataGene::new()),
        Box::new(BenchlingGene::new()),
    ]
}
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
    #[test]
    fn research_owns_declared_genes() {
        let manifest = ResearchDomainHarness::new().manifest().clone();
        assert_eq!(manifest.owned_genes.len(), 27);
        assert_eq!(
            LiteratureReviewGene::new()
                .manifest()
                .owner_harness
                .as_deref(),
            Some("research-domain")
        );
    }
}
