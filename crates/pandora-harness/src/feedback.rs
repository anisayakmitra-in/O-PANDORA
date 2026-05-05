pub fn score_response(output: &str) -> i32 {
    let mut score = 0;

    if output.len() > 50 {
        score += 1;
    }

    if output.contains("Rust") || output.contains("system") {
        score += 1;
    }

    if output.contains("error") {
        score -= 1;
    }

    score
}

