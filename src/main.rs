// main.rs — Entry point for the "glow" CLI
//
// This CLI is an API client — it talks to the Glow server (FastAPI)
// the same way the Next.js frontend does, just from the terminal.

mod api;     // our HTTP client module (like importing api.py)
mod ledger;  // hash-based ledger service (built into the CLI)
mod types;   // response types (like importing types.py)

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "glow", about = "Glow CLI — interact with the Glow API")]
struct Cli {
    /// API base URL (e.g. https://glowbeta.learn-loop.org)
    #[arg(long, env = "GLOW_API_URL", default_value = "http://localhost:8000")]
    api_url: String,

    /// License key for authentication
    #[arg(long, env = "GLOW_LICENSE_KEY")]
    license_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

// Top-level commands: `glow personas ...`, `glow agents ...`, etc.
// For now, just personas as the example.
#[derive(Subcommand)]
enum Commands {
    /// Manage personas
    Personas {
        #[command(subcommand)]
        action: PersonaCommands,
    },
}

// Persona subcommands: `glow personas list`, `glow personas get <id>`, etc.
#[derive(Subcommand)]
enum PersonaCommands {
    /// List all personas (POST /api/v5/artifacts/personas/search)
    List,

    /// Get a single persona by ID (POST /api/v5/artifacts/personas/get)
    Get {
        /// Persona ID (UUID)
        id: String,
    },

    /// Create a new persona (POST /api/v5/artifacts/personas/create)
    Create {
        /// Persona name
        #[arg(long)]
        name: String,

        /// Persona description
        #[arg(long)]
        description: Option<String>,
    },

    /// Delete a persona (POST /api/v5/artifacts/personas/delete)
    Delete {
        /// Persona ID (UUID)
        id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Build the API client with the provided config
    // This is like instantiating a class:
    //   client = ApiClient(base_url=..., license_key=...)
    let client = api::Client::new(
        &cli.api_url,
        cli.license_key.as_deref(),  // Option<String> → Option<&str>
    );

    match cli.command {
        Commands::Personas { action } => match action {
            PersonaCommands::List => cmd_persona_list(&client)?,
            PersonaCommands::Get { id } => cmd_persona_get(&client, &id)?,
            PersonaCommands::Create { name, description } => {
                cmd_persona_create(&client, &name, description.as_deref())?
            }
            PersonaCommands::Delete { id } => cmd_persona_delete(&client, &id)?,
        },
    }

    Ok(())
}

// ── Command Implementations ────────────────────────────────────

fn cmd_persona_list(client: &api::Client) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_search()?;

    println!(
        "{} ({} total)\n",
        "Personas".bold(),
        response.total_count
    );

    // Print each persona as a row
    for p in &response.personas {
        let status = if p.is_inactive {
            "inactive".red().to_string()
        } else {
            "active".green().to_string()
        };

        println!(
            "  {} {} [{}]",
            p.persona_id.dimmed(),
            p.name.bold(),
            status,
        );

        if let Some(desc) = &p.description {
            println!("    {}", desc.dimmed());
        }
    }

    Ok(())
}

fn cmd_persona_get(client: &api::Client, id: &str) -> Result<()> {
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

fn cmd_persona_create(client: &api::Client, name: &str, description: Option<&str>) -> Result<()> {
    use colored::Colorize;

    let response = client.persona_create(name, description)?;

    for result in &response.results {
        if result.success {
            println!(
                "{} Created persona {}",
                "OK".green().bold(),
                result.persona_id.dimmed(),
            );
        } else {
            println!(
                "{} {}",
                "FAIL".red().bold(),
                result.message,
            );
        }
    }

    Ok(())
}

fn cmd_persona_delete(client: &api::Client, id: &str) -> Result<()> {
    use colored::Colorize;

    client.persona_delete(id)?;
    println!("{} Deleted persona {}", "OK".green().bold(), id.dimmed());

    Ok(())
}
