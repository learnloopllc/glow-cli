use anyhow::Result;
use crate::glow::GlowClient;
use crate::output::{self, OutputMode};

pub(crate) fn cmd_artifact_action(
    client: &GlowClient,
    artifact_type: &str,
    action: &str,
    body: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let parsed_body = match body {
        Some(s) => Some(
            serde_json::from_str(s)
                .map_err(|e| anyhow::anyhow!("Invalid JSON body: {}", e))?,
        ),
        None => None,
    };

    let response = client.artifact_action(artifact_type, action, parsed_body)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap_or_else(|_| format!("{:?}", resp))
        );
    });
    Ok(())
}
