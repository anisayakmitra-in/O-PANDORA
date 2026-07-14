use std::collections::HashMap;

use pandora_model::OllamaClient;
use pandora_types::HarnessSpec;

fn valid_harness(

    specs:
        &[HarnessSpec],

    selected:
        &str,

) -> bool {

    specs
        .iter()
        .any(
            |s| {

                s.name
                    .to_lowercase()

                    ==

                    selected
            }
        )
}

fn safe_fallback(

    specs:
        &[HarnessSpec],

) -> String {

    specs
        .first()
        .map(
            |s| s.name.clone()
        )
        .unwrap_or_else(
            || String::from(
                "unavailable"
            )
        )
}

pub async fn select_harness(

    client:
        &OllamaClient,

    model:
        &str,

    input:
        &str,

    specs:
        &[HarnessSpec],

) -> String {

    if specs.is_empty() {

        return String::from(
            "unavailable"
        );
    }

    let options = specs
        .iter()
        .map(
            |s| {

                format!(
                    "{} specializes in {}",
                    s.name,
                    s.domain
                )
            }
        )
        .collect::<Vec<_>>()
        .join(", ");

    let prompt = format!(
        "You are a strict routing system.\n\
Available harnesses: {}\n\
Task: {}\n\
Return EXACTLY one harness name.\n\
No explanation.\n\
No extra text.",
        options,
        input
    );

    match client
        .chat(
            model,
            &prompt
        )
        .await
    {

        Ok(res) => {

            let selected =

                res.message
                    .content
                    .trim()
                    .trim_matches(
                        |c: char| {

                            !c.is_alphanumeric()

                            &&

                            c != '-'

                            &&

                            c != '_'
                        }
                    )
                    .to_lowercase();

            if valid_harness(
                specs,
                &selected,
            ) {

                return specs
                    .iter()
                    .find(
                        |s| {

                            s.name
                                .to_lowercase()

                                ==

                                selected
                        }
                    )
                    .map(
                        |s| s.name.clone()
                    )
                    .unwrap_or_else(
                        || safe_fallback(
                            specs
                        )
                    );
            }

            safe_fallback(
                specs
            )
        }

        Err(_) => {

            safe_fallback(
                specs
            )
        }
    }
}

pub fn select_best_by_performance(

    scores:
        &HashMap<
            String,
            Vec<i32>
        >,

) -> Option<String> {

    let mut best_name =
        None;

    let mut best_avg =
        f32::MIN;

    for (name, values)
    in scores {

        if values.is_empty() {

            continue;
        }

        let sum: i32 =
            values.iter().sum();

        let avg =
            sum as f32
            /
            values.len() as f32;

        if avg > best_avg {

            best_avg =
                avg;

            best_name =
                Some(
                    name.clone()
                );
        }
    }

    best_name
}
