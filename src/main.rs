// main.rs — Entry point for "learnloop" / "ll" CLI
//
// Two APIs:
//   - LearnLoop API (central): licensing, billing, OAuth — one instance
//   - Glow API (instance): personas, agents, etc. — many on customer machines
//
// The CLI talks to both.

mod api;
mod config;
mod ledger;
mod output;
mod types;

use anyhow::Result;
use clap::{Parser, Subcommand};
use output::OutputMode;

#[derive(Parser)]
#[command(name = "learnloop", about = "LearnLoop CLI — manage your platform")]
struct Cli {
    /// LearnLoop API URL (central platform)
    #[arg(long, env = "LEARNLOOP_API_URL")]
    api_url: Option<String>,

    /// License key for the LearnLoop API
    #[arg(long, env = "LEARNLOOP_LICENSE_KEY")]
    license_key: Option<String>,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Skip confirmation prompts for destructive actions
    #[arg(long, short = 'y', global = true)]
    yes: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with LearnLoop (OAuth)
    Login,

    /// Check network connectivity and airgapped mode status
    #[command(alias = "net")]
    Network,

    /// Show connection and authentication status
    Status,

    /// Hash-based ledger operations
    #[command(alias = "l")]
    Ledger {
        #[command(subcommand)]
        action: LedgerCommands,
    },

    /// Manage a Glow instance (talks to a Glow API)
    #[command(alias = "g")]
    Glow {
        /// Glow instance API URL
        #[arg(long, env = "GLOW_API_URL")]
        instance_url: Option<String>,

        #[command(subcommand)]
        action: GlowCommands,
    },
}

#[derive(Subcommand)]
enum LedgerCommands {
    /// Verify the integrity of a ledger chain
    Verify {
        /// Path to the ledger directory
        #[arg(long, env = "LEARNLOOP_LEDGER_PATH")]
        path: String,
    },
    /// Show ledger chain status and statistics
    Status {
        /// Path to the ledger directory
        #[arg(long, env = "LEARNLOOP_LEDGER_PATH")]
        path: String,
    },
    /// Sync local ledger entries to LearnLoop API for billing
    Sync {
        /// Path to the ledger directory
        #[arg(long, env = "LEARNLOOP_LEDGER_PATH")]
        path: String,
    },
}

// Glow subcommands — each artifact type is a subcommand
#[derive(Subcommand)]
enum GlowCommands {
    /// Manage personas
    #[command(alias = "p")]
    Personas {
        #[command(subcommand)]
        action: PersonaCommands,
    },
    // Future: Agents, Scenarios, Sessions, etc.
}

#[derive(Subcommand)]
enum PersonaCommands {
    /// List all personas
    #[command(alias = "ls")]
    List,
    /// Get a single persona by ID
    Get { id: String },
    /// Create a new persona
    #[command(alias = "new")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a persona
    #[command(aliases = ["rm", "remove"])]
    Delete { id: String },
}

// ── Main ───────────────────────────────────────────────────────

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = config::Config::load()?;
    let mode = OutputMode::resolve(cli.json);

    // Resolve URLs: CLI flag/env > config file > default
    let api_url = config::resolve_option(
        cli.api_url.as_deref(),
        cfg.api_url.as_deref(),
        "https://api.learn-loop.org",
    );
    let license_key = cli.license_key.or(cfg.license_key);

    match cli.command {
        Commands::Login => cmd_login(&api_url, mode)?,
        Commands::Network => cmd_network(&api_url, mode)?,
        Commands::Status => cmd_status(&api_url, &license_key, mode)?,
        Commands::Ledger { action } => {
            let key = license_key
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("License key required for ledger operations. Use --license-key or set LEARNLOOP_LICENSE_KEY."))?;
            match action {
                LedgerCommands::Verify { path } => cmd_ledger_verify(&path, key, mode)?,
                LedgerCommands::Status { path } => cmd_ledger_status(&path, key, mode)?,
                LedgerCommands::Sync { path } => {
                    let ll_client = api::LearnLoopClient::new(&api_url, Some(key));
                    cmd_ledger_sync(&path, key, &ll_client, mode)?
                }
            }
        }
        Commands::Glow {
            instance_url,
            action,
        } => {
            let glow_url = config::resolve_option(
                instance_url.as_deref(),
                cfg.glow_url.as_deref(),
                "http://localhost:8000",
            );
            let client = api::GlowClient::new(&glow_url, license_key.as_deref());
            match action {
                GlowCommands::Personas { action } => match action {
                    PersonaCommands::List => cmd_persona_list(&client, mode)?,
                    PersonaCommands::Get { id } => cmd_persona_get(&client, &id, mode)?,
                    PersonaCommands::Create { name, description } => {
                        cmd_persona_create(&client, &name, description.as_deref(), mode)?
                    }
                    PersonaCommands::Delete { id } => {
                        cmd_persona_delete(&client, &id, cli.yes, mode)?
                    }
                },
            }
        }
    }

    Ok(())
}

// ── Login ──────────────────────────────────────────────────────

fn cmd_login(api_url: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct LoginInfo {
        api_url: String,
        status: String,
    }

    output::print_result(
        mode,
        &LoginInfo {
            api_url: api_url.to_string(),
            status: "not_implemented".into(),
        },
        |_| {
            println!(
                "{} Opening browser for OAuth login...",
                "LOGIN".green().bold()
            );
            println!("  API: {}", api_url.dimmed());
            // TODO: Open browser → OAuth flow → store token locally
        },
    );
    Ok(())
}

// ── Network ────────────────────────────────────────────────────

fn cmd_network(api_url: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct NetworkStatus {
        api_url: String,
        api_status: String,
    }

    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let api_status = match http.get(format!("{}/health", api_url)).send() {
        Ok(resp) if resp.status().is_success() => "connected",
        _ => "unreachable",
    };

    output::print_result(
        mode,
        &NetworkStatus {
            api_url: api_url.to_string(),
            api_status: api_status.to_string(),
        },
        |s| {
            println!("{}", "Network Status".bold());
            match s.api_status.as_str() {
                "connected" => println!("  LearnLoop API: {}", "connected".green()),
                _ => {
                    println!(
                        "  LearnLoop API: {} (airgapped mode available)",
                        "unreachable".red()
                    );
                    println!("    Tip: check that {} is correct", api_url.dimmed());
                }
            }
        },
    );
    Ok(())
}

// ── Status ────────────────────────────────────────────────────

fn cmd_status(api_url: &str, license_key: &Option<String>, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct StatusReport {
        config_file: String,
        config_exists: bool,
        api_url: String,
        api_status: String,
        license_key_set: bool,
    }

    let config_path = config::Config::config_path();
    let config_exists = config_path.exists();

    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()?;

    let api_status = match http.get(format!("{}/health", api_url)).send() {
        Ok(r) if r.status().is_success() => "connected",
        _ => "unreachable",
    };

    let report = StatusReport {
        config_file: config_path.display().to_string(),
        config_exists,
        api_url: api_url.to_string(),
        api_status: api_status.to_string(),
        license_key_set: license_key.is_some(),
    };

    output::print_result(mode, &report, |r| {
        println!("{}", "LearnLoop Status".bold());
        println!(
            "  Config:      {} {}",
            r.config_file,
            if r.config_exists {
                "(found)".green().to_string()
            } else {
                "(not found)".dimmed().to_string()
            }
        );
        println!("  API URL:     {}", r.api_url);
        println!(
            "  API Status:  {}",
            if r.api_status == "connected" {
                "connected".green().to_string()
            } else {
                "unreachable".red().to_string()
            }
        );
        println!(
            "  License Key: {}",
            if r.license_key_set {
                "set".green().to_string()
            } else {
                "not set".yellow().to_string()
            }
        );
    });
    Ok(())
}

// ── Ledger ─────────────────────────────────────────────────────

fn cmd_ledger_verify(path: &str, license_key: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let chain = ledger::read_chain(std::path::Path::new(path))?;
    let status = ledger::verify_chain(&chain, license_key);

    output::print_result(mode, &status, |s| {
        if s.valid {
            println!(
                "{} Chain verified — {} entries, all valid",
                "OK".green().bold(),
                s.total_entries
            );
        } else {
            println!(
                "{} Chain verification failed — {} errors in {} entries",
                "FAIL".red().bold(),
                s.errors.len(),
                s.total_entries
            );
            for err in &s.errors {
                println!("  - {}", err);
            }
        }
    });

    if !status.valid {
        anyhow::bail!("Ledger verification failed");
    }
    Ok(())
}

fn cmd_ledger_status(path: &str, license_key: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct StatusReport {
        path: String,
        total_entries: usize,
        chain_valid: bool,
        latest_hash: String,
        latest_attempt: Option<String>,
    }

    let chain = ledger::read_chain(std::path::Path::new(path))?;
    let verification = ledger::verify_chain(&chain, license_key);
    let latest_attempt = chain.last().map(|e| e.attempt.attempt_id.clone());

    let report = StatusReport {
        path: path.to_string(),
        total_entries: verification.total_entries,
        chain_valid: verification.valid,
        latest_hash: verification.latest_hash.clone(),
        latest_attempt,
    };

    output::print_result(mode, &report, |r| {
        println!("{}", "Ledger Status".bold());
        println!("  Path:           {}", r.path);
        println!("  Total Entries:  {}", r.total_entries);
        println!(
            "  Chain Valid:    {}",
            if r.chain_valid {
                "yes".green().to_string()
            } else {
                "NO".red().to_string()
            }
        );
        if let Some(id) = &r.latest_attempt {
            println!("  Latest Attempt: {}", id.dimmed());
        }
        println!("  Latest Hash:    {}", r.latest_hash.dimmed());
    });
    Ok(())
}

fn cmd_ledger_sync(
    path: &str,
    license_key: &str,
    ll_client: &api::LearnLoopClient,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct SyncResult {
        total_local: usize,
        synced: usize,
        license_id: String,
    }

    // 1. Read and verify local chain
    let chain = ledger::read_chain(std::path::Path::new(path))?;
    let verification = ledger::verify_chain(&chain, license_key);
    if !verification.valid {
        anyhow::bail!(
            "Local chain is invalid — fix integrity issues before syncing. Run 'learnloop ledger verify --path {}'",
            path
        );
    }

    // 2. Validate license key with LearnLoop API
    let license_resp = ll_client.validate_license()?;
    if !license_resp.valid {
        anyhow::bail!("License key is not valid. Check your LEARNLOOP_LICENSE_KEY.");
    }
    let license_id = license_resp
        .license
        .ok_or_else(|| anyhow::anyhow!("License validation returned no license info."))?
        .id;

    // 3. Upload each entry
    let mut synced = 0;
    for entry in &chain {
        ll_client.usage_report(&license_id, &entry.chain_hash, 1)?;
        synced += 1;
    }

    let result = SyncResult {
        total_local: chain.len(),
        synced,
        license_id: license_id.clone(),
    };

    output::print_result(mode, &result, |r| {
        println!(
            "{} Synced {} entries to LearnLoop API (license: {})",
            "OK".green().bold(),
            r.synced,
            r.license_id.dimmed()
        );
    });
    Ok(())
}

// ── Glow: Personas ─────────────────────────────────────────────

fn cmd_persona_list(client: &api::GlowClient, mode: OutputMode) -> Result<()> {
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

fn cmd_persona_get(client: &api::GlowClient, id: &str, mode: OutputMode) -> Result<()> {
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

fn cmd_persona_create(
    client: &api::GlowClient,
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

fn cmd_persona_delete(
    client: &api::GlowClient,
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
