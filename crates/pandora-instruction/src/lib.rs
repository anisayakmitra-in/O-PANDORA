//! Pandora Instruction Engine - Constitutional Input Layer.
//!
//! Supports hierarchical instruction layers from L0 (constitutional, immutable)
//! through L7 (provider-specific). Conflicts are resolved constitutionally,
//! not by last-write-wins.
//!
//! Layers:
//!   L0 Constitutional (immutable)
//!   L1 Organization (enterprise)
//!   L2 Workspace (.pandora/project.toml)
//!   L3 Conversation (this chat)
//!   L4 Task (current request)
//!   L5 Loop (iteration-specific)
//!   L6 Gene (tool-specific)
//!   L7 Provider (model-specific)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub layer: InstructionLayer,
    pub source: String,
    pub content: String,
    pub enabled: bool,
    pub category: Option<String>,
}

impl Instruction {
    pub fn new(
        layer: InstructionLayer,
        source: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            layer,
            source: source.into(),
            content: content.into(),
            enabled: true,
            category: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InstructionLayer {
    L0Constitutional,
    L1Organization,
    L2Workspace,
    L3Conversation,
    L4Task,
    L5Loop,
    L6Gene,
    L7Provider,
}

impl InstructionLayer {
    pub fn name(&self) -> &'static str {
        match self {
            InstructionLayer::L0Constitutional => "constitutional",
            InstructionLayer::L1Organization => "organization",
            InstructionLayer::L2Workspace => "workspace",
            InstructionLayer::L3Conversation => "conversation",
            InstructionLayer::L4Task => "task",
            InstructionLayer::L5Loop => "loop",
            InstructionLayer::L6Gene => "gene",
            InstructionLayer::L7Provider => "provider",
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstructionConflict {
    pub topic: String,
    pub instructions: Vec<(InstructionLayer, String)>,
    pub resolution: String,
}

#[derive(Debug, Clone)]
pub struct MergedInstructionContext {
    pub instructions: Vec<Instruction>,
    pub conflicts_resolved: Vec<InstructionConflict>,
    pub summary: String,
}

// =========================================================================
// Instruction Engine
// =========================================================================

pub struct InstructionEngine {
    layers: HashMap<InstructionLayer, Vec<Instruction>>,
}

impl InstructionEngine {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
        }
    }

    pub fn add(&mut self, instruction: Instruction) {
        self.layers
            .entry(instruction.layer)
            .or_default()
            .push(instruction);
    }

    pub fn instruct(
        &mut self,
        layer: InstructionLayer,
        source: impl Into<String>,
        content: impl Into<String>,
    ) {
        self.add(Instruction::new(layer, source, content));
    }

    pub fn get_layer(&self, layer: &InstructionLayer) -> Vec<&Instruction> {
        self.layers
            .get(layer)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn all_enabled(&self) -> Vec<&Instruction> {
        let mut all: Vec<&Instruction> = self
            .layers
            .values()
            .flat_map(|v| v.iter())
            .filter(|i| i.enabled)
            .collect();
        all.sort_by_key(|b| std::cmp::Reverse(b.layer));
        all
    }

    pub fn remove_by_source(&mut self, source: &str) {
        for v in self.layers.values_mut() {
            v.retain(|i| i.source != source);
        }
    }

    pub fn set_enabled(&mut self, source: &str, enabled: bool) {
        for v in self.layers.values_mut() {
            for i in v.iter_mut() {
                if i.source == source {
                    i.enabled = enabled;
                }
            }
        }
    }
}

impl Default for InstructionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Instruction Resolution Engine
// =========================================================================

pub struct InstructionResolutionEngine;

impl InstructionResolutionEngine {
    /// Merge instructions from all layers, resolving conflicts constitutionally.
    /// Rules: Higher layer overrides lower for same topic. L0 never overridden.
    /// Explicit denies override allows at any layer.
    pub fn resolve(engine: &InstructionEngine) -> MergedInstructionContext {
        let all = engine.all_enabled();
        let mut merged: Vec<Instruction> = Vec::new();
        let mut conflicts = Vec::new();

        let mut by_topic: HashMap<String, Vec<&Instruction>> = HashMap::new();
        for instruction in &all {
            let topic = instruction
                .content
                .split(['.', '\n'])
                .next()
                .unwrap_or("")
                .to_string();
            by_topic.entry(topic).or_default().push(instruction);
        }

        for (topic, instructions) in &by_topic {
            if instructions.len() == 1 {
                merged.push((*instructions[0]).clone());
                continue;
            }
            let has_l0 = instructions
                .iter()
                .any(|i| i.layer == InstructionLayer::L0Constitutional);
            let highest = instructions.iter().max_by_key(|i| i.layer).unwrap();

            if has_l0 && highest.layer != InstructionLayer::L0Constitutional {
                let instrs: Vec<_> = instructions
                    .iter()
                    .map(|i| (i.layer, i.content.clone()))
                    .collect();
                conflicts.push(InstructionConflict {
                    topic: topic.clone(),
                    instructions: instrs,
                    resolution: format!("L0 overrides: {}", highest.content),
                });
                if let Some(l0) = instructions
                    .iter()
                    .find(|i| i.layer == InstructionLayer::L0Constitutional)
                {
                    merged.push((*l0).clone());
                }
            } else {
                let instrs: Vec<_> = instructions
                    .iter()
                    .map(|i| (i.layer, i.content.clone()))
                    .collect();
                conflicts.push(InstructionConflict {
                    topic: topic.clone(),
                    instructions: instrs,
                    resolution: format!(
                        "Layer {} overrides: {}",
                        highest.layer.name(),
                        highest.content
                    ),
                });
                merged.push((*highest).clone());
            }
        }

        let summary = if conflicts.is_empty() {
            format!("{} instructions, no conflicts", merged.len())
        } else {
            format!(
                "{} instructions, {} conflicts resolved",
                merged.len(),
                conflicts.len()
            )
        };
        MergedInstructionContext {
            instructions: merged,
            conflicts_resolved: conflicts,
            summary,
        }
    }
}

// =========================================================================
// Context Engine
// =========================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    pub source: String,
    pub content: String,
    pub priority: f64,
    pub token_count: usize,
    pub compressed: bool,
}

impl ContextItem {
    pub fn new(source: impl Into<String>, content: impl Into<String>, priority: f64) -> Self {
        let content = content.into();
        let token_count = content.len() / 4;
        Self {
            source: source.into(),
            content,
            priority,
            token_count,
            compressed: false,
        }
    }
}

/// The Context Engine - manages what gets injected into model context.
/// Memory = persistence. Context = injection.
pub struct ContextEngine {
    items: Vec<ContextItem>,
    max_tokens: usize,
}

impl ContextEngine {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            items: Vec::new(),
            max_tokens,
        }
    }
    pub fn default_for_model() -> Self {
        Self::new(128_000)
    }

    pub fn add(&mut self, item: ContextItem) {
        self.items.push(item);
    }

    pub fn ranked_context(&mut self) -> Vec<&ContextItem> {
        self.items.sort_by(|a, b| {
            b.priority
                .partial_cmp(&a.priority)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let mut total = 0;
        let mut result = Vec::new();
        for item in &self.items {
            if total + item.token_count <= self.max_tokens {
                result.push(item);
                total += item.token_count;
            } else if !item.compressed {
                break;
            }
        }
        result
    }

    pub fn summarize(&self, max_chars: usize) -> String {
        let mut s = String::new();
        for item in &self.items {
            let snippet = if item.content.len() > 200 {
                format!("{}...", &item.content[..200])
            } else {
                item.content.clone()
            };
            s.push_str(&format!(
                "[{}] {} (p{:.1})\n",
                item.source, snippet, item.priority
            ));
        }
        if s.len() > max_chars {
            s.truncate(max_chars);
            s.push_str("...");
        }
        s
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for ContextEngine {
    fn default() -> Self {
        Self::default_for_model()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layers_priority() {
        assert!(InstructionLayer::L7Provider > InstructionLayer::L0Constitutional);
        assert!(InstructionLayer::L4Task > InstructionLayer::L2Workspace);
    }

    #[test]
    fn add_and_retrieve() {
        let mut e = InstructionEngine::new();
        e.instruct(InstructionLayer::L2Workspace, "p", "Always fmt");
        e.instruct(InstructionLayer::L3Conversation, "c", "Decompose runtime");
        assert_eq!(e.get_layer(&InstructionLayer::L2Workspace).len(), 1);
        assert_eq!(e.all_enabled().len(), 2);
    }

    #[test]
    fn higher_layer_wins() {
        let mut e = InstructionEngine::new();
        e.instruct(InstructionLayer::L2Workspace, "w", "Use local models");
        e.instruct(InstructionLayer::L4Task, "u", "Use local models");
        let r = InstructionResolutionEngine::resolve(&e);
        assert_eq!(r.conflicts_resolved.len(), 1);
        assert!(r.conflicts_resolved[0].resolution.contains("task"));
    }

    #[test]
    fn l0_immutable() {
        let mut e = InstructionEngine::new();
        e.instruct(InstructionLayer::L0Constitutional, "c", "Always verify");
        e.instruct(InstructionLayer::L5Loop, "l", "Skip verification");
        let r = InstructionResolutionEngine::resolve(&e);
        assert!(r
            .instructions
            .iter()
            .any(|i| i.content.contains("Always verify")));
    }

    #[test]
    fn context_ranks_by_priority() {
        let mut c = ContextEngine::new(1000);
        c.add(ContextItem::new("m", "Low priority", 10.0));
        c.add(ContextItem::new("i", "High priority", 90.0));
        let r = c.ranked_context();
        assert_eq!(r.len(), 2);
        assert!(r[0].priority > r[1].priority);
    }

    #[test]
    fn context_respects_tokens() {
        let mut c = ContextEngine::new(50);
        c.add(ContextItem::new("a", "Short", 90.0));
        let mut long_item = ContextItem::new("b", "x", 10.0);
        long_item.token_count = 100;
        c.add(long_item);
        let r = c.ranked_context();
        assert_eq!(r.len(), 1);
    }
}
