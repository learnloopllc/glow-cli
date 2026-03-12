use anyhow::Result;
use colored::Colorize;
use crate::glow::{self, GlowClient};
use crate::output::{self, OutputMode};

pub(crate) fn cmd_events_stream(
    client: &GlowClient,
    artifact_type: Option<&str>,
    artifact_id: Option<&str>,
    operation: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    if mode == OutputMode::Human {
        eprintln!(
            "{} Connecting to SSE stream...",
            "▸".dimmed()
        );
    }

    let response = client.events_stream(artifact_type, artifact_id, operation)?;

    if mode == OutputMode::Human {
        eprintln!(
            "{} Connected. Streaming events (Ctrl+C to stop)\n",
            "✓".green()
        );
    }

    glow::read_sse_events(response, |data| {
        if mode == OutputMode::Json {
            println!("{}", data);
        } else {
            // Try to pretty-print JSON data, fall back to raw
            match serde_json::from_str::<serde_json::Value>(data) {
                Ok(val) => println!("{}", serde_json::to_string_pretty(&val).unwrap()),
                Err(_) => println!("{}", data),
            }
        }
    })?;

    Ok(())
}

pub(crate) fn cmd_events_poll(
    client: &GlowClient,
    body: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let parsed = match body {
        Some(s) => serde_json::from_str(s)
            .map_err(|e| anyhow::anyhow!("Invalid JSON body: {}", e))?,
        None => serde_json::json!({}),
    };

    let response = client.events_poll(parsed)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}

pub(crate) fn cmd_events_webhook_dispatch(
    client: &GlowClient,
    body: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let parsed = match body {
        Some(s) => serde_json::from_str(s)
            .map_err(|e| anyhow::anyhow!("Invalid JSON body: {}", e))?,
        None => serde_json::json!({}),
    };

    let response = client.events_webhook_dispatch(parsed)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}
