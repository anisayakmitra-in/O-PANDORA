//! Pandora Web Dashboard — serves the web UI via tiny_http.

mod dashboard;

use std::time::Instant;
use tiny_http::{Header, Response, Server};

fn main() -> anyhow::Result<()> {
    let port = std::env::var("PANDORA_WEB_PORT").unwrap_or_else(|_| "6789".into());
    let addr = format!("0.0.0.0:{}", port);
    let server =
        Server::http(&addr).map_err(|e| anyhow::anyhow!("Failed to bind {}: {}", addr, e))?;
    let start = Instant::now();

    eprintln!("🌐 Pandora Dashboard: http://localhost:{}", port);
    eprintln!("   Press Ctrl+C to stop");

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        if url == "/" || url == "/dashboard" {
            let html = build_dashboard(start);
            let len = html.len();
            let response = Response::from_string(html)
                .with_header(
                    Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap(),
                )
                .with_header(
                    Header::from_bytes("Content-Length", len.to_string().as_bytes()).unwrap(),
                );
            let _ = request.respond(response);
        } else {
            let response = Response::from_string("404 Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
    Ok(())
}

fn build_dashboard(start: Instant) -> String {
    use pandora_kuber::builtin;
    use pandora_shadow_council::ShadowCouncil;

    let sc = ShadowCouncil::new();
    let s = sc.summary();
    let genes = builtin::all();

    let uptime = {
        let secs = start.elapsed().as_secs();
        let d = secs / 86400;
        let h = (secs % 86400) / 3600;
        let m = (secs % 3600) / 60;
        let s2 = secs % 60;
        format!("{}d {:02}h {:02}m {:02}s", d, h, m, s2)
    };

    let mut html = dashboard::DASHBOARD_HTML.to_string();

    html = html.replace("%%VERSION%%", &format!("v{}", env!("CARGO_PKG_VERSION")));
    html = html.replace("%%UPTIME%%", &uptime);
    html = html.replace("%%SERVICE_COUNT%%", "10");
    html = html.replace("%%GENE_COUNT%%", &genes.len().to_string());
    html = html.replace("%%HARNESS_TOTAL%%", &s.total_harnesses.to_string());
    html = html.replace("%%HARNESS_SOURCE%%", &s.source_count.to_string());
    html = html.replace("%%HARNESS_META%%", &s.meta_count.to_string());
    html = html.replace("%%HARNESS_DOMAIN%%", &s.domain_count.to_string());
    html = html.replace("%%SLASH_COUNT%%", &s.slash_commands.to_string());

    // Build harness list
    let harnesses = crate::build_harness_list();
    html = html.replace("<!-- populated by server -->", &harnesses);

    html
}

fn build_harness_list() -> String {
    let names = [
        (
            "Cognition Source",
            "Handles cognition ingress and semantic intake",
        ),
        ("Planning Source", "Decomposition, strategy, orchestration"),
        (
            "Execution Source",
            "Execution lifecycle and sandbox control",
        ),
        ("Governance Source", "Policy, trust scoring, and governance"),
        ("Identity Source", "Identity management and lineage"),
        (
            "Coordination Meta",
            "Inter-harness coordination and routing",
        ),
        ("Coding Domain", "Developer workflows and build automation"),
        ("Research Domain", "Search, browse, summarize, extract"),
    ];
    let mut out = String::new();
    for (name, desc) in &names {
        out.push_str(&format!(r#"<div class="flex justify-between items-start gap-2">
            <div><div class="text-purple-300 font-semibold text-[11px]">{}</div>
            <div class="text-[10px] text-purple-400/50">{}</div></div>
            <span class="text-green-400 text-[10px] font-medium tracking-wide">OPERATIONAL</span></div>"#, name, desc));
    }
    out
}
