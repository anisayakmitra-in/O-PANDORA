use crate::intent::{Intent, IntentConfidence, IntentKind};

const VOCABULARY: &[(&str, IntentKind, f32)] = &[
    ("install", IntentKind::Install, 0.95),
    ("remove", IntentKind::Remove, 0.95),
    ("uninstall", IntentKind::Remove, 0.95),
    ("delete", IntentKind::Delete, 0.90),
    ("modify", IntentKind::Modify, 0.90),
    ("update", IntentKind::Modify, 0.90),
    ("change", IntentKind::Modify, 0.85),
    ("edit", IntentKind::Modify, 0.85),
    ("rename", IntentKind::Modify, 0.85),
    ("fix", IntentKind::Modify, 0.80),
    ("patch", IntentKind::Modify, 0.80),
    ("create", IntentKind::Create, 0.95),
    ("build", IntentKind::Create, 0.95),
    ("write", IntentKind::Create, 0.90),
    ("make", IntentKind::Create, 0.90),
    ("generate", IntentKind::Create, 0.90),
    ("scaffold", IntentKind::Create, 0.90),
    ("author", IntentKind::Create, 0.85),
    ("read", IntentKind::Read, 0.90),
    ("show", IntentKind::Read, 0.85),
    ("list", IntentKind::Read, 0.85),
    ("get", IntentKind::Read, 0.80),
    ("find", IntentKind::Read, 0.80),
    ("search", IntentKind::Read, 0.80),
    ("describe", IntentKind::Read, 0.80),
    ("inspect", IntentKind::Read, 0.80),
    ("run", IntentKind::Execute, 0.95),
    ("execute", IntentKind::Execute, 0.95),
    ("launch", IntentKind::Execute, 0.90),
    ("start", IntentKind::Execute, 0.85),
    ("invoke", IntentKind::Execute, 0.85),
    ("test", IntentKind::Execute, 0.80),
    ("reflect", IntentKind::Reflect, 0.95),
    ("summarize", IntentKind::Reflect, 0.85),
    ("recap", IntentKind::Reflect, 0.80),
    ("what", IntentKind::Ask, 0.85),
    ("why", IntentKind::Ask, 0.85),
    ("how", IntentKind::Ask, 0.80),
    ("explain", IntentKind::Ask, 0.80),
    ("verify", IntentKind::Verify, 0.95),
    ("check", IntentKind::Verify, 0.85),
    ("audit", IntentKind::Verify, 0.85),
    ("validate", IntentKind::Verify, 0.85),
];

pub fn extract_intent(user_input: &str) -> Intent {
    if user_input.trim().is_empty() {
        return Intent::new(
            IntentKind::Unknown,
            String::new(),
            user_input.to_string(),
            IntentConfidence::new(0.0),
        );
    }

    let normalized = user_input.to_ascii_lowercase();
    let tokens: Vec<&str> = normalized
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();

    for (verb, kind, base_confidence) in VOCABULARY {
        if tokens.iter().any(|t| t == verb) {
            let confidence = boost_confidence(*base_confidence, &tokens);
            return Intent::new(
                *kind,
                extract_target(&tokens, verb),
                user_input.to_string(),
                confidence,
            );
        }
    }

    Intent::new(
        IntentKind::Unknown,
        extract_target(&tokens, ""),
        user_input.to_string(),
        IntentConfidence::new(0.1),
    )
}

fn boost_confidence(base: f32, tokens: &[&str]) -> IntentConfidence {
    if tokens.len() >= 3 {
        IntentConfidence::new((base + 0.04).min(0.99))
    } else {
        IntentConfidence::new(base)
    }
}

fn extract_target(tokens: &[&str], matched_verb: &str) -> String {
    let mut found_verb = matched_verb.is_empty();
    for token in tokens {
        if !found_verb {
            if *token == matched_verb {
                found_verb = true;
            }
            continue;
        }
        if is_stop_word(token) || token.len() < 2 {
            continue;
        }
        return (*token).to_string();
    }
    String::new()
}

fn is_stop_word(token: &str) -> bool {
    matches!(
        token,
        "a" | "an"
            | "the"
            | "of"
            | "to"
            | "for"
            | "in"
            | "on"
            | "at"
            | "by"
            | "with"
            | "and"
            | "or"
            | "i"
            | "you"
            | "we"
            | "they"
            | "my"
            | "your"
            | "our"
            | "this"
            | "that"
            | "these"
            | "those"
            | "is"
            | "are"
            | "was"
            | "were"
            | "be"
            | "been"
            | "being"
            | "do"
            | "does"
            | "did"
            | "doing"
            | "would"
            | "should"
            | "could"
            | "can"
            | "will"
            | "shall"
            | "may"
            | "might"
            | "must"
            | "it"
            | "me"
            | "us"
            | "them"
            | "he"
            | "she"
            | "his"
            | "her"
            | "its"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_is_unknown_with_zero_confidence() {
        let i = extract_intent("");
        assert_eq!(i.kind, IntentKind::Unknown);
        assert_eq!(i.confidence.value(), 0.0);
    }

    #[test]
    fn whitespace_only_is_unknown() {
        let i = extract_intent("   \t  \n ");
        assert_eq!(i.kind, IntentKind::Unknown);
    }

    #[test]
    fn build_command_is_create() {
        let i = extract_intent("build a hello world program");
        assert_eq!(i.kind, IntentKind::Create);
        assert!(i.confidence.is_plannable());
    }

    #[test]
    fn delete_command_is_delete() {
        let i = extract_intent("delete the temp file");
        assert_eq!(i.kind, IntentKind::Delete);
    }

    #[test]
    fn run_command_is_execute() {
        let i = extract_intent("run the test suite");
        assert_eq!(i.kind, IntentKind::Execute);
    }

    #[test]
    fn reflect_command_is_reflect() {
        let i = extract_intent("reflect on today's session");
        assert_eq!(i.kind, IntentKind::Reflect);
    }

    #[test]
    fn what_question_is_ask() {
        let i = extract_intent("what is a cognition runtime");
        assert_eq!(i.kind, IntentKind::Ask);
    }

    #[test]
    fn install_command_is_install() {
        let i = extract_intent("install the security harness");
        assert_eq!(i.kind, IntentKind::Install);
    }

    #[test]
    fn verify_command_is_verify() {
        let i = extract_intent("verify the audit log");
        assert_eq!(i.kind, IntentKind::Verify);
    }

    #[test]
    fn gibberish_is_unknown_but_not_zero() {
        let i = extract_intent("qqqzzz xxx");
        assert_eq!(i.kind, IntentKind::Unknown);
        assert!(i.confidence.value() > 0.0);
    }

    #[test]
    fn raw_input_is_preserved() {
        let raw = "Build a Hello World Program!!";
        let i = extract_intent(raw);
        assert_eq!(i.raw_input, raw);
    }

    #[test]
    fn case_insensitive() {
        let i = extract_intent("BUILD x");
        assert_eq!(i.kind, IntentKind::Create);
    }

    #[test]
    fn punctuation_does_not_break_match() {
        let i = extract_intent("build, a hello world");
        assert_eq!(i.kind, IntentKind::Create);
    }

    #[test]
    fn target_is_extracted() {
        let i = extract_intent("build a calculator app");
        assert_eq!(i.target, "calculator");
    }

    #[test]
    fn stop_words_skipped_in_target() {
        let i = extract_intent("delete the temp");
        assert_eq!(i.target, "temp");
    }

    #[test]
    fn longer_inputs_get_higher_confidence() {
        let short = extract_intent("build x");
        let long = extract_intent("build a calculator app for the project");
        assert!(long.confidence.value() > short.confidence.value());
    }
}
