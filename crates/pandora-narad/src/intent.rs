use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntentKind {
    Create,
    Read,
    Modify,
    Delete,
    Execute,
    Ask,
    Reflect,
    Install,
    Remove,
    Verify,
    Unknown,
}

impl IntentKind {
    pub const ALL: &'static [IntentKind] = &[
        IntentKind::Create,
        IntentKind::Read,
        IntentKind::Modify,
        IntentKind::Delete,
        IntentKind::Execute,
        IntentKind::Ask,
        IntentKind::Reflect,
        IntentKind::Install,
        IntentKind::Remove,
        IntentKind::Verify,
    ];

    pub fn is_side_effecting(self) -> bool {
        matches!(
            self,
            IntentKind::Create
                | IntentKind::Modify
                | IntentKind::Delete
                | IntentKind::Execute
                | IntentKind::Install
                | IntentKind::Remove
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IntentConfidence(f32);

impl IntentConfidence {
    pub const MAX: IntentConfidence = IntentConfidence(1.0);
    pub const MIN: IntentConfidence = IntentConfidence(0.0);

    pub fn new(value: f32) -> Self {
        IntentConfidence(value.clamp(0.0, 1.0))
    }

    pub fn value(self) -> f32 {
        self.0
    }

    pub fn is_plannable(self) -> bool {
        self.0 >= 0.4
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    pub kind: IntentKind,
    pub target: String,
    pub raw_input: String,
    pub confidence: IntentConfidence,
}

impl Intent {
    pub fn new(
        kind: IntentKind,
        target: String,
        raw_input: String,
        confidence: IntentConfidence,
    ) -> Self {
        Intent {
            kind,
            target,
            raw_input,
            confidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_confidence_clamps() {
        assert_eq!(IntentConfidence::new(2.0).value(), 1.0);
        assert_eq!(IntentConfidence::new(-1.0).value(), 0.0);
        assert_eq!(IntentConfidence::new(0.5).value(), 0.5);
    }

    #[test]
    fn intent_confidence_plannable() {
        assert!(IntentConfidence::new(0.4).is_plannable());
        assert!(IntentConfidence::new(0.9).is_plannable());
        assert!(!IntentConfidence::new(0.39).is_plannable());
        assert!(!IntentConfidence::new(0.0).is_plannable());
    }

    #[test]
    fn intent_kind_side_effecting() {
        assert!(IntentKind::Create.is_side_effecting());
        assert!(IntentKind::Delete.is_side_effecting());
        assert!(IntentKind::Execute.is_side_effecting());
        assert!(IntentKind::Install.is_side_effecting());
        assert!(!IntentKind::Read.is_side_effecting());
        assert!(!IntentKind::Ask.is_side_effecting());
        assert!(!IntentKind::Reflect.is_side_effecting());
    }

    #[test]
    fn intent_kind_all_includes_known_kinds() {
        assert!(IntentKind::ALL.contains(&IntentKind::Create));
        assert!(IntentKind::ALL.contains(&IntentKind::Read));
        assert!(!IntentKind::ALL.contains(&IntentKind::Unknown));
    }
}
