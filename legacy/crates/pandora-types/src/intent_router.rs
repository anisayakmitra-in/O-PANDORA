//! Intent Router — routes user tasks to the right harness/gene/workflow.
//!
//! Data-driven: discovers capabilities from manifests, builds an intent index,
//! and matches user input to the best execution target via confidence scoring.
//! No keyword match statements — index is rebuilt when manifests change.
//!
//! Invariant: "Route tasks by intent and capability, not by harness kind."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A capability entry — what a gene/harness/skill can do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    /// Keywords that indicate this capability (lowercase).
    pub keywords: Vec<String>,
    /// Confidence weight [0.0, 1.0] for matching.
    pub weight: f32,
    /// Which component provides this capability.
    pub provider_id: String,
    pub provider_kind: CapabilityProviderKind,
}

/// What kind of component provides the capability.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilityProviderKind {
    Gene,
    Harness,
    Skill,
    Workflow,
    Evaluator,
    Custom(String),
}

/// A match result from the router.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentMatch {
    pub capability: Capability,
    pub score: f32,
    pub reason: String,
}

/// The intent router — matches user input to capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntentRouter {
    capabilities: Vec<Capability>,
    /// keyword → list of capability indices
    keyword_index: HashMap<String, Vec<usize>>,
    initialized: bool,
}

/// Stoplist — common words that don't indicate intent.
const STOPLIST: &[&str] = &[
    "a", "an", "the", "this", "that", "with", "from", "into", "about",
    "and", "or", "not", "but", "for", "to", "of", "in", "on", "at",
    "is", "are", "was", "were", "be", "been", "being", "have", "has",
    "had", "do", "does", "did", "will", "would", "could", "should",
    "may", "might", "can", "shall", "i", "you", "he", "she", "it",
    "we", "they", "me", "him", "her", "us", "them", "my", "your",
    "his", "its", "our", "their", "just", "please", "now", "then",
    "get", "make", "use", "find", "show", "check", "run", "start",
    "stop", "create", "add", "remove", "update", "need", "want",
    "help", "like", "think", "know", "see", "go", "come", "take",
    "give", "tell", "ask", "try", "let", "put", "set", "say",
    "http", "https", "www", "com", "org", "net",
];

impl IntentRouter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a capability.
    pub fn register(&mut self, cap: Capability) {
        self.capabilities.push(cap);
        self.initialized = false; // needs reindex
    }

    /// Build the keyword index from registered capabilities.
    pub fn build_index(&mut self) {
        self.keyword_index.clear();
        for (i, cap) in self.capabilities.iter().enumerate() {
            for kw in &cap.keywords {
                let kw = kw.to_lowercase();
                if !STOPLIST.contains(&kw.as_str()) {
                    self.keyword_index.entry(kw).or_default().push(i);
                }
            }
            // Also index words from the description
            for word in cap.description.to_lowercase().split_whitespace() {
                let word = word.trim_matches(|c: char| !c.is_alphanumeric());
                if !word.is_empty() && !STOPLIST.contains(&word) && word.len() > 2 {
                    self.keyword_index.entry(word.to_string()).or_default().push(i);
                }
            }
        }
        self.initialized = true;
    }

    /// Match user input against capabilities. Returns scored results.
    pub fn match_input(&mut self, input: &str) -> Vec<IntentMatch> {
        if !self.initialized {
            self.build_index();
        }

        let words: Vec<String> = input
            .to_lowercase()
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|w| !w.is_empty() && !STOPLIST.contains(&w.as_str()) && w.len() > 2)
            .collect();

        if words.is_empty() {
            return vec![];
        }

        let mut scores: HashMap<usize, f32> = HashMap::new();
        let mut reasons: HashMap<usize, Vec<String>> = HashMap::new();

        for word in &words {
            if let Some(indices) = self.keyword_index.get(word) {
                for &idx in indices {
                    let cap = &self.capabilities[idx];
                    let boost = cap.weight;
                    *scores.entry(idx).or_insert(0.0) += 0.3 + boost;
                    reasons.entry(idx).or_default().push(format!("keyword match: '{word}'"));
                }
            }
        }

        // Build results sorted by score descending
        let mut results: Vec<IntentMatch> = scores
            .into_iter()
            .map(|(idx, score)| IntentMatch {
                capability: self.capabilities[idx].clone(),
                score,
                reason: reasons.get(&idx).cloned().unwrap_or_default().join(", "),
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Find capabilities by kind.
    pub fn by_kind(&self, kind: &CapabilityProviderKind) -> Vec<&Capability> {
        self.capabilities.iter().filter(|c| &c.provider_kind == kind).collect()
    }

    /// Count registered capabilities.
    pub fn count(&self) -> usize {
        self.capabilities.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_router() {
        let mut router = IntentRouter::new();
        let results = router.match_input("build a web app");
        assert!(results.is_empty());
    }

    #[test]
    fn keyword_match() {
        let mut router = IntentRouter::new();
        router.register(Capability {
            name: "code".into(),
            description: "Generate and edit source code in any language".into(),
            keywords: vec!["code".into(), "generate".into(), "write".into()],
            weight: 0.8,
            provider_id: "coding-domain".into(),
            provider_kind: CapabilityProviderKind::Harness,
        });
        router.register(Capability {
            name: "browser".into(),
            description: "Navigate websites and extract data".into(),
            keywords: vec!["browser".into(), "navigate".into(), "scrape".into()],
            weight: 0.5,
            provider_id: "computer-use".into(),
            provider_kind: CapabilityProviderKind::Harness,
        });

        let results = router.match_input("write a python script");
        assert!(!results.is_empty());
        assert_eq!(results[0].capability.name, "code");
    }

    #[test]
    fn multi_keyword_boost() {
        let mut router = IntentRouter::new();
        router.register(Capability {
            name: "security".into(),
            description: "Security audit and vulnerability scanning".into(),
            keywords: vec!["security".into(), "audit".into(), "vulnerability".into()],
            weight: 0.9,
            provider_id: "security-domain".into(),
            provider_kind: CapabilityProviderKind::Harness,
        });

        let results = router.match_input("security audit of my codebase");
        assert!(!results.is_empty());
        // Multiple keywords should boost the score
        assert!(results[0].score > 0.5);
    }

    #[test]
    fn stoplist_filtered() {
        let mut router = IntentRouter::new();
        router.register(Capability {
            name: "research".into(),
            description: "Research papers and literature review".into(),
            keywords: vec!["research".into(), "paper".into(), "literature".into()],
            weight: 0.7,
            provider_id: "research-domain".into(),
            provider_kind: CapabilityProviderKind::Harness,
        });

        // Stopwords should not affect matching
        let results = router.match_input("I just want to research this topic");
        assert!(results.iter().any(|r| r.capability.name == "research"));
    }
}
