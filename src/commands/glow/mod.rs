pub mod artifacts;
pub mod events;
pub mod uploads;

use anyhow::Result;
use crate::glow::GlowClient;
use crate::output::{self, OutputMode};

// ── Health ────────────────────────────────────────────────────

pub(crate) fn cmd_health(client: &GlowClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.health()?;
    output::print_result(mode, &response, |resp| {
        let indicator = if resp.status == "healthy" || resp.status == "ok" {
            "●".green()
        } else {
            "●".red()
        };
        println!("{} Glow instance: {}", indicator, resp.status.bold());
        if let Some(ref v) = resp.version {
            println!("  Version: {}", v.dimmed());
        }
    });
    Ok(())
}

// ── Persona commands ─────────────────────────────────────────

pub(crate) fn cmd_persona_list(client: &GlowClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_search()?;
    output::print_result(mode, &response, |resp| {
        println!("{} ({} total)\n", "Personas".bold(), resp.total_count);
        for p in &resp.personas {
            let status = if p.is_inactive {
                "inactive".red().to_string()
            } else {
                "active".green().to_string()
            };
            println!("  {} {} [{}]", p.persona_id.dimmed(), p.name.bold(), status);
            if let Some(desc) = &p.description {
                println!("    {}", desc.dimmed());
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_persona_get(client: &GlowClient, id: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_get(id)?;
    output::print_result(mode, &response, |resp| {
        println!("{}", "Persona".bold());
        println!("  Name:        {}", resp.names.current_name.bold());
        if let Some(desc) = &resp.descriptions.current_description {
            println!("  Description: {}", desc);
        }
        println!("  Can Edit:    {}", resp.can_edit);
        println!("  Group ID:    {}", resp.group_id.dimmed());
    });
    Ok(())
}

pub(crate) fn cmd_persona_create(
    client: &GlowClient,
    name: &str,
    description: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_create(name, description)?;
    output::print_result(mode, &response, |resp| {
        for result in &resp.results {
            if result.success {
                println!(
                    "{} Created persona {}",
                    "OK".green().bold(),
                    result.persona_id.dimmed()
                );
            } else {
                println!("{} {}", "FAIL".red().bold(), result.message);
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_persona_delete(
    client: &GlowClient,
    id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct DeleteResult {
        persona_id: String,
        deleted: bool,
    }

    if !output::confirm(&format!("Delete persona {}?", id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    client.persona_delete(id)?;
    output::print_result(
        mode,
        &DeleteResult {
            persona_id: id.to_string(),
            deleted: true,
        },
        |_| {
            println!("{} Deleted persona {}", "OK".green().bold(), id.dimmed());
        },
    );
    Ok(())
}
