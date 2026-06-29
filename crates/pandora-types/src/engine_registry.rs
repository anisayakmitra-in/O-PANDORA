//! Engine constitutional metadata registry.
//!
//! Maps every engine to its owning Source Harness,
//! owning Meta Harness, required capabilities,
//! execution profile, and GEPA/DSR support.
//!
//! This is metadata only. No runtime changes.

use crate::universal::{EngineMetadata, ExecutionProfile};

/// Look up an engine by name and return its
/// constitutional metadata.
pub fn lookup_engine(name: &str) -> Option<EngineMetadata> {
    ENGINES.iter().find(|e| e.0 == name).map(|e| (e.1)())
}

/// List all registered engine names.
pub fn all_engine_names() -> Vec<&'static str> {
    ENGINES.iter().map(|e| e.0).collect()
}

/// Total number of registered engines.
pub fn engine_count() -> usize {
    ENGINES.len()
}

type EngineEntry = (&'static str, fn() -> EngineMetadata);

const ENGINES: &[EngineEntry] = &[
    // --- PHOENIX engines ---
    ("ExecutionArchaeologyEngine", || {
        em(
            "phoenix",
            "execution",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ExecutionRankingEngine", || {
        em(
            "phoenix",
            "execution",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ExecutionSurvivabilityEngine", || {
        em(
            "phoenix",
            "execution",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ExecutionStateSynthesisEngine", || {
        em(
            "phoenix",
            "execution",
            &[],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("ConstitutionalExecutionLicenseEngine", || {
        em(
            "phoenix",
            "capability",
            &["execution"],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    // --- ANUBIS engines ---
    ("SemanticMemoryEngine", || {
        em(
            "anubis",
            "semantic_memory",
            &["memory", "embedding"],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("RepositoryMemoryGraphEngine", || {
        em(
            "anubis",
            "graph",
            &["memory"],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("RepositorySearchEngine", || {
        em(
            "anubis",
            "retrieval",
            &["memory"],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("MemoryAwarePromptEngine", || {
        em(
            "anubis",
            "semantic_memory",
            &["memory", "provider"],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("MemoryConsolidationEngine", || {
        em(
            "anubis",
            "consolidation",
            &["memory"],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    // --- MOIRA engines ---
    ("RecursivePlanningEngine", || {
        em(
            "moira",
            "planning",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("AutonomousReasoningEngine", || {
        em(
            "moira",
            "reasoning",
            &["provider"],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ContextRoutingEngine", || {
        em(
            "moira",
            "context",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ContextResetEngine", || {
        em(
            "moira",
            "context",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    // --- SHANI: Civilization engines ---
    ("ConstitutionalCivilizationAxiologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationCosmologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationEpistemologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationFabricEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationGenesisEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationMemoryEngine", || {
        em(
            "shani",
            "civilization",
            &["memory"],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationMetanoeticsEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationMythologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationNoologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationOntologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationPhilosophyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationPraxeologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationRebirthEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("CivilizationRegenerationEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("CivilizationResilienceEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationSuccessionEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationTeleologyEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationTerminationEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalCivilizationTranscendenceEngine", || {
        em(
            "shani",
            "civilization",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    // --- SHANI: Swarm engines ---
    ("SwarmDreamEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("SwarmEvolutionEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("GenomeEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("InstinctEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("PhenotypeEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("SwarmSpecializationEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateful,
            true,
            true,
        )
    }),
    ("EntropyCollapseEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("UncertaintyTopologyEngine", || {
        em(
            "shani",
            "swarm",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    // --- SHANI: Evolution engines ---
    ("MutationTournamentEngine", || {
        em(
            "shani",
            "mutation",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("FitnessEngine", || {
        em(
            "shani",
            "fitness",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalMetaEvolutionEngine", || {
        em(
            "shani",
            "evolution",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalReliabilityBenchmarkEngine", || {
        em(
            "shani",
            "evolution",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    // --- PANOPTES engines ---
    ("PanoptesOversightEngine", || {
        em(
            "panoptes",
            "oversight",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ShadowCouncilEngine", || {
        em(
            "panoptes",
            "governance",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("SandboxGovernanceEngine", || {
        em(
            "panoptes",
            "governance",
            &["sandbox"],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    ("ConstitutionalAutonomyEngine", || {
        em(
            "panoptes",
            "governance",
            &[],
            ExecutionProfile::Stateless,
            true,
            true,
        )
    }),
    // --- Infrastructure engines (remain in pandora-runtime) ---
    ("AdaptiveOrchestrationEngine", || {
        em(
            "infrastructure",
            "orchestration",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("CapabilityResolutionEngine", || {
        em(
            "infrastructure",
            "capability",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("CompilerFeedbackEngine", || {
        em(
            "infrastructure",
            "compiler",
            &["compiler"],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("AstEngine", || {
        em(
            "infrastructure",
            "compiler",
            &["compiler"],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("AutonomousCodingEngine", || {
        em(
            "infrastructure",
            "coding",
            &["compiler"],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("DependencyGraphEngine", || {
        em(
            "infrastructure",
            "dependency",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("DelegationEngine", || {
        em(
            "infrastructure",
            "delegation",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("RemoteExecutionEngine", || {
        em(
            "infrastructure",
            "execution",
            &["network"],
            ExecutionProfile::Distributed,
            false,
            false,
        )
    }),
    ("WorkflowEngine", || {
        em(
            "infrastructure",
            "workflow",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("ToolCognitionEngine", || {
        em(
            "infrastructure",
            "tool",
            &["provider"],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("MultiModelArbitrationEngine", || {
        em(
            "infrastructure",
            "model",
            &["provider"],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("DockerSandboxEngine", || {
        em(
            "infrastructure",
            "sandbox",
            &["docker"],
            ExecutionProfile::Stateful,
            false,
            false,
        )
    }),
    ("EmbeddingEngine", || {
        em(
            "infrastructure",
            "embedding",
            &["embedding"],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("EntropyEngine", || {
        em(
            "infrastructure",
            "telemetry",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("TraceEngine", || {
        em(
            "infrastructure",
            "tracing",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("RollbackEngine", || {
        em(
            "infrastructure",
            "rollback",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("RepositoryEvolutionEngine", || {
        em(
            "infrastructure",
            "evolution",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("RepositorySearchEngine", || {
        em(
            "infrastructure",
            "search",
            &["filesystem"],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("ConstitutionalRealityConsensusEngine", || {
        em(
            "infrastructure",
            "reality",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("ConstitutionalRealitySimulationEngine", || {
        em(
            "infrastructure",
            "reality",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("ConstitutionalArtifactProvenanceEngine", || {
        em(
            "infrastructure",
            "provenance",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("SovereignStrategicDirectiveEngine", || {
        em(
            "infrastructure",
            "strategy",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
    ("SurvivabilityConstitutionEngine", || {
        em(
            "infrastructure",
            "survivability",
            &[],
            ExecutionProfile::Stateless,
            false,
            false,
        )
    }),
];

fn em(
    owner: &str,
    meta: &str,
    caps: &[&str],
    profile: ExecutionProfile,
    gepa: bool,
    dsr: bool,
) -> EngineMetadata {
    EngineMetadata {
        owning_source_harness: owner.to_string(),
        owning_meta_harness: meta.to_string(),
        required_capabilities: caps.iter().map(|s| s.to_string()).collect(),
        execution_profile: profile,
        gepa_support: gepa,
        dsr_support: dsr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_count_check() {
        assert_eq!(engine_count(), 72);
    }

    #[test]
    fn phoenix_engine_lookup() {
        let e = lookup_engine("ExecutionArchaeologyEngine").unwrap();
        assert_eq!(e.owning_source_harness, "phoenix");
        assert_eq!(e.owning_meta_harness, "execution");
        assert!(e.gepa_support);
        assert!(e.dsr_support);
    }

    #[test]
    fn anubis_engine_lookup() {
        let e = lookup_engine("SemanticMemoryEngine").unwrap();
        assert_eq!(e.owning_source_harness, "anubis");
        assert!(e.required_capabilities.contains(&"memory".to_string()));
    }

    #[test]
    fn shani_civilization_lookup() {
        let e = lookup_engine("ConstitutionalCivilizationAxiologyEngine").unwrap();
        assert_eq!(e.owning_source_harness, "shani");
        assert_eq!(e.owning_meta_harness, "civilization");
    }

    #[test]
    fn shani_swarm_lookup() {
        let e = lookup_engine("GenomeEngine").unwrap();
        assert_eq!(e.owning_source_harness, "shani");
        assert_eq!(e.owning_meta_harness, "swarm");
    }

    #[test]
    fn panoptes_lookup() {
        let e = lookup_engine("ShadowCouncilEngine").unwrap();
        assert_eq!(e.owning_source_harness, "panoptes");
        assert_eq!(e.owning_meta_harness, "governance");
    }

    #[test]
    fn infrastructure_lookup() {
        let e = lookup_engine("AdaptiveOrchestrationEngine").unwrap();
        assert_eq!(e.owning_source_harness, "infrastructure");
        assert!(!e.gepa_support);
        assert!(!e.dsr_support);
    }

    #[test]
    fn nonexistent_engine() {
        assert!(lookup_engine("NonexistentEngine").is_none());
    }

    #[test]
    fn all_names_list() {
        let names = all_engine_names();
        assert_eq!(names.len(), 72);
        assert!(!names.contains(&"PhoenixHarness")); // not a harness
        assert!(names.contains(&"ExecutionArchaeologyEngine"));
    }
}
