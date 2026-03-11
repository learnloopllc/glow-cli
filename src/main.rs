// main.rs — Entry point for "learnloop" / "ll" CLI
//
// Two APIs:
//   - LearnLoop API (central): licensing, billing, OAuth — one instance
//   - Glow API (instance): personas, agents, etc. — many on customer machines
//
// The CLI talks to both.

mod api;
mod ledger;
mod types;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "learnloop", about = "LearnLoop CLI — manage your platform")]
struct Cli {
    /// LearnLoop API URL (central platform)
    #[arg(
        long,
        env = "LEARNLOOP_API_URL",
        default_value = "https://api.learn-loop.org"
    )]
    api_url: String,

    /// License key for the LearnLoop API
    #[arg(long, env = "LEARNLOOP_LICENSE_KEY")]
    license_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with LearnLoop (OAuth)
    Login,

    /// Check network connectivity and airgapped mode status
    Network,

    /// Hash-based ledger operations
    Ledger {
        #[command(subcommand)]
        action: LedgerCommands,
    },

    /// Manage a Glow instance (talks to a Glow API)
    Glow {
        /// Glow instance API URL
        #[arg(long, env = "GLOW_API_URL", default_value = "http://localhost:8000")]
        instance_url: String,

        #[command(subcommand)]
        action: GlowCommands,
    },
}

#[derive(Subcommand)]
enum LedgerCommands {
    /// Verify the integrity of a ledger chain
    Verify,
    /// Show ledger status
    Status,
}

// Glow subcommands — each artifact type is a subcommand
#[derive(Subcommand)]
enum GlowCommands {
    /// Manage personas
    Personas {
        #[command(subcommand)]
        action: PersonaCommands,
    },
    // Future: Agents, Scenarios, Sessions, etc.
}

#[derive(Subcommand)]
enum PersonaCommands {
    /// List all personas
    List,
    /// Get a single persona by ID
    Get { id: String },
    /// Create a new persona
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a persona
    Delete { id: String },
}

// ── Main ───────────────────────────────────────────────────────

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login => cmd_login(&cli.api_url)?,
        Commands::Network => cmd_network(&cli.api_url)?,
        Commands::Ledger { action } => match action {
            LedgerCommands::Verify => cmd_ledger_verify()?,
            LedgerCommands::Status => cmd_ledger_status()?,
        },
        Commands::Glow {
            instance_url,
            action,
        } => {
            let client = api::GlowClient::new(&instance_url, cli.license_key.as_deref());
            match action {
                GlowCommands::Personas { action } => match action {
                    PersonaCommands::List => cmd_persona_list(&client)?,
                    PersonaCommands::Get { id } => cmd_persona_get(&client, &id)?,
                    PersonaCommands::Create { name, description } => {
                        cmd_persona_create(&client, &name, description.as_deref())?
                    }
                    PersonaCommands::Delete { id } => cmd_persona_delete(&client, &id)?,
                },
            }
        }
    }

    Ok(())
}

// ── Login ──────────────────────────────────────────────────────

fn cmd_login(api_url: &str) -> Result<()> {
    use colored::Colorize;
    println!(
        "{} Opening browser for OAuth login...",
        "LOGIN".green().bold()
    );
    println!("  API: {}", api_url.dimmed());
    // TODO: Open browser → OAuth flow → store token locally
    Ok(())
}

// ── Network ────────────────────────────────────────────────────

fn cmd_network(api_url: &str) -> Result<()> {
    use colored::Colorize;
    println!("{}", "Network Status".bold());

    let http = reqwest::blocking::Client::new();
    match http.get(format!("{}/health", api_url)).send() {
        Ok(resp) if resp.status().is_success() => {
            println!("  LearnLoop API: {}", "connected".green());
        }
        _ => {
            println!(
                "  LearnLoop API: {} (airgapped mode available)",
                "unreachable".red()
            );
        }
    }

    Ok(())
}

// ── Ledger ─────────────────────────────────────────────────────

fn cmd_ledger_verify() -> Result<()> {
    use colored::Colorize;
    println!("{}", "Verifying ledger integrity...".green());
    // TODO: Walk chain and verify hashes
    Ok(())
}

fn cmd_ledger_status() -> Result<()> {
    use colored::Colorize;
    println!("{}", "Ledger Status".bold());
    // TODO: Show chain length, last entry, etc.
    Ok(())
}

// ── Glow: Personas ─────────────────────────────────────────────

fn cmd_persona_list(client: &api::GlowClient) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_search()?;
    println!("{} ({} total)\n", "Personas".bold(), response.total_count);

    for p in &response.personas {
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

    Ok(())
}

fn cmd_persona_get(client: &api::GlowClient, id: &str) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_get(id)?;
    println!("{}", "Persona".bold());
    println!("  Name:        {}", response.names.current_name.bold());
    if let Some(desc) = &response.descriptions.current_description {
        println!("  Description: {}", desc);
    }
    println!("  Can Edit:    {}", response.can_edit);
    println!("  Group ID:    {}", response.group_id.dimmed());

    Ok(())
}

fn cmd_persona_create(
    client: &api::GlowClient,
    name: &str,
    description: Option<&str>,
) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_create(name, description)?;
    for result in &response.results {
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

    Ok(())
}

fn cmd_persona_delete(client: &api::GlowClient, id: &str) -> Result<()> {
    use colored::Colorize;

    client.persona_delete(id)?;
    println!("{} Deleted persona {}", "OK".green().bold(), id.dimmed());

    Ok(())
}
