// main.rs — Entry point for "glow" / "glw" CLI
//
// Two APIs:
//   - LearnLoop API (central): licensing, billing, OAuth — one instance
//   - Glow API (instance): personas, agents, etc. — many on customer machines
//
// The CLI talks to both.

mod api_common;
mod auth;
mod commands;
mod config;
mod glow;
mod learnloop;
mod ledger;
mod output;
mod resource;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use output::OutputMode;

#[derive(Parser)]
#[command(name = "glow", about = "Glow CLI — manage your platform")]
struct Cli {
    /// LearnLoop API URL (central platform)
    #[arg(long, env = "GLOW_API_URL")]
    api_url: Option<String>,

    /// Glow instance URL
    #[arg(long, env = "GLOW_INSTANCE_URL")]
    instance_url: Option<String>,

    /// License key
    #[arg(long, env = "GLOW_LICENSE_KEY")]
    license_key: Option<String>,

    /// OAuth client ID
    #[arg(long, env = "GLOW_CLIENT_ID")]
    client_id: Option<String>,

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
    /// Authenticate with Glow instance (OAuth)
    Login,

    /// Remove stored authentication token for Glow instance
    Logout,

    /// Check Glow instance health
    Health,

    /// Show current user context and identity
    Context,

    /// Emulate another user profile
    Emulate {
        /// Target profile ID to emulate
        target_profile_id: String,
        /// TTL in seconds
        #[arg(long, default_value = "120")]
        ttl: u32,
    },

    /// Stop emulating and return to your own profile
    Unemulate,

    /// Generate content for a group
    Generate {
        /// Group ID to generate for
        group_id: String,
        /// JSON body for additional options
        #[arg(long)]
        body: Option<String>,
    },

    /// Stream events via SSE (Server-Sent Events)
    Stream {
        /// Artifact type to stream
        #[arg(long)]
        artifact: String,
        /// Operation to stream (e.g. create, update, delete)
        #[arg(long)]
        operation: String,
        /// Filter by entity ID
        #[arg(long)]
        entity_id: Option<String>,
        /// Cursor for resuming from a position
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Manage configured Glow instances
    #[command(alias = "inst")]
    Instances {
        #[command(subcommand)]
        action: InstanceCommands,
    },

    /// Switch to a configured Glow instance
    Use {
        /// Instance name (as configured with 'glow instances add')
        name: String,
    },

    /// Hash-based ledger operations
    #[command(alias = "l")]
    Ledger {
        #[command(subcommand)]
        action: LedgerCommands,
    },

    /// Platform administration (LearnLoop management plane)
    Admin {
        #[command(subcommand)]
        action: AdminCommands,
    },

    /// Interact with a resource on the Glow instance (e.g. glow personas search)
    #[command(external_subcommand)]
    Resource(Vec<String>),

    /// Manage a Glow instance directly (talks to a Glow API)
    #[command(alias = "g")]
    Glow {
        /// Glow instance API URL
        #[arg(long, env = "GLOW_INSTANCE_URL")]
        instance_url: Option<String>,

        #[command(subcommand)]
        action: GlowCommands,
    },
}

// ── Admin subcommands (LearnLoop management plane) ───────────

#[derive(Subcommand)]
enum AdminCommands {
    /// Authenticate with LearnLoop API (OAuth)
    Login,

    /// Remove stored authentication token for LearnLoop API
    Logout,

    /// Show all active login sessions
    Sessions,

    /// Show current authenticated user
    Whoami,

    /// Check network connectivity and airgapped mode status
    #[command(alias = "net")]
    Network,

    /// Show connection and authentication status
    Status,

    /// License management
    #[command(alias = "lic")]
    Licenses {
        #[command(subcommand)]
        action: LicenseCommands,
    },

    /// Organization management
    #[command(alias = "org")]
    Orgs {
        #[command(subcommand)]
        action: OrgCommands,
    },

    /// View usage summary for a license
    Usage {
        /// License ID to view usage for
        license_id: String,
    },

    /// Deploy a new Glow instance
    Deploy {
        #[command(subcommand)]
        action: DeployCommands,
    },

    /// Billing and subscription management
    Billing {
        #[command(subcommand)]
        action: BillingCommands,
    },
}

#[derive(Subcommand)]
enum LicenseCommands {
    /// List all licenses
    #[command(alias = "ls")]
    List {
        /// Only show active licenses
        #[arg(long)]
        active: bool,
    },
    /// Validate your license key
    Validate,
    /// Create a new license
    Create {
        /// License key string
        #[arg(long)]
        key: String,
        /// Expiry date (e.g. 2026-12-31)
        #[arg(long)]
        expiry: String,
    },
    /// Update an existing license
    Update {
        /// License ID
        id: String,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        expiry: Option<String>,
        #[arg(long)]
        active: Option<bool>,
    },
    /// Delete a license
    #[command(alias = "rm")]
    Delete {
        /// License ID
        id: String,
    },
}

#[derive(Subcommand)]
enum OrgCommands {
    /// List your organizations
    #[command(alias = "ls")]
    List,
    /// Create a new organization
    #[command(alias = "new")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// Get organization details
    Get {
        /// Organization ID
        id: String,
    },
    /// Update an organization
    Update {
        /// Organization ID
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete an organization
    #[command(alias = "rm")]
    Delete {
        /// Organization ID
        id: String,
    },
    /// Manage organization members
    Members {
        /// Organization ID
        id: String,
        #[command(subcommand)]
        action: OrgMemberCommands,
    },
    /// Manage organization license
    License {
        /// Organization ID
        id: String,
        #[command(subcommand)]
        action: OrgLicenseCommands,
    },
    /// List organization deployments
    Deployments {
        /// Organization ID
        id: String,
    },
}

#[derive(Subcommand)]
enum OrgMemberCommands {
    /// List members
    #[command(alias = "ls")]
    List,
    /// Add a member by email
    Add {
        #[arg(long)]
        email: String,
    },
    /// Remove a member
    #[command(alias = "rm")]
    Remove {
        /// User ID to remove
        user_id: String,
    },
}

#[derive(Subcommand)]
enum OrgLicenseCommands {
    /// Show current license
    Get,
    /// Assign a license
    Set {
        /// License ID to assign
        license_id: String,
    },
    /// Remove the assigned license
    #[command(alias = "rm")]
    Remove,
}

#[derive(Subcommand)]
enum BillingCommands {
    /// List available plans
    Plans,
    /// Check billing status for an organization
    Status {
        /// Organization ID
        org_id: String,
    },
    /// Start checkout for a plan
    Checkout {
        /// Organization ID
        org_id: String,
        /// Plan ID (e.g. "pro")
        #[arg(long, default_value = "pro")]
        plan: String,
    },
    /// Open the billing portal
    Portal {
        /// Organization ID
        org_id: String,
    },
}

#[derive(Subcommand)]
enum DeployCommands {
    /// Create a new Glow deployment
    #[command(alias = "new")]
    Create {
        /// License ID to deploy under
        #[arg(long)]
        license_id: String,
        /// Deployment name
        #[arg(long)]
        name: String,
        /// Subdomain (e.g. "acme" for acme.learn-loop.org)
        #[arg(long)]
        subdomain: String,
        /// Base domain (default: learn-loop.org)
        #[arg(long)]
        base_domain: Option<String>,
        /// Make the repo public
        #[arg(long)]
        public: bool,
    },
    /// Stop a running deployment
    Stop {
        /// Deployment ID
        id: String,
    },
    /// Destroy a stopped deployment (removes repo, DNS, and marks as destroyed)
    #[command(alias = "rm")]
    Destroy {
        /// Deployment ID
        id: String,
    },
    /// List all deployments
    #[command(alias = "ls")]
    List {
        /// Include destroyed deployments
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand)]
enum InstanceCommands {
    /// List configured instances
    #[command(alias = "ls")]
    List,
    /// Add a new instance
    Add {
        /// Instance name (e.g. "prod", "staging")
        name: String,
        /// Instance URL
        #[arg(long)]
        url: String,
    },
    /// Remove a configured instance
    #[command(alias = "rm")]
    Remove {
        /// Instance name
        name: String,
    },
}

#[derive(Subcommand)]
enum LedgerCommands {
    /// Verify the integrity of a ledger chain
    Verify {
        /// Path to the ledger directory
        #[arg(long, env = "GLOW_LEDGER_PATH")]
        path: String,
    },
    /// Show ledger chain status and statistics
    Status {
        /// Path to the ledger directory
        #[arg(long, env = "GLOW_LEDGER_PATH")]
        path: String,
    },
    /// Sync local ledger entries to LearnLoop API for billing
    Sync {
        /// Path to the ledger directory
        #[arg(long, env = "GLOW_LEDGER_PATH")]
        path: String,
    },
}

// ── Glow instance subcommands (legacy, being promoted to top-level) ──

#[derive(Subcommand)]
enum GlowCommands {
    /// Authenticate with this Glow instance (OAuth)
    Login,

    /// Remove stored token for this Glow instance
    Logout,

    /// Check Glow instance health
    Health,

    /// Manage personas (typed shortcut)
    #[command(alias = "p")]
    Personas {
        #[command(subcommand)]
        action: PersonaCommands,
    },

    /// Generic artifact CRUD — works with any artifact type
    #[command(alias = "a")]
    Artifacts {
        /// Artifact type (e.g. personas, agents, scenarios, rubrics, sessions)
        artifact_type: String,
        /// Action (e.g. search, get, create, update, delete, duplicate, draft, drafts, export, refresh, docs)
        action: String,
        /// JSON body for the request
        #[arg(long)]
        body: Option<String>,
    },

    /// Event streaming, polling, and webhooks
    Events {
        #[command(subcommand)]
        action: EventCommands,
    },

    /// File upload operations (TUS, multipart, CSV)
    #[command(alias = "up")]
    Upload {
        #[command(subcommand)]
        action: UploadCommands,
    },
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

#[derive(Subcommand)]
enum EventCommands {
    /// Stream events via SSE (Server-Sent Events)
    Stream {
        /// Filter by artifact type
        #[arg(long)]
        artifact_type: Option<String>,
        /// Filter by artifact ID
        #[arg(long)]
        artifact_id: Option<String>,
        /// Filter by operation
        #[arg(long)]
        operation: Option<String>,
    },
    /// Poll for events
    Poll {
        /// JSON body (e.g. cursor, filters)
        #[arg(long)]
        body: Option<String>,
    },
    /// Dispatch a webhook event
    #[command(name = "webhooks-dispatch")]
    WebhooksDispatch {
        /// JSON body for the webhook dispatch
        #[arg(long)]
        body: Option<String>,
    },
}

#[derive(Subcommand)]
enum UploadCommands {
    /// Create a new upload (TUS initiation)
    #[command(alias = "new")]
    Create {
        /// Filename
        #[arg(long)]
        filename: String,
        /// MIME content type
        #[arg(long)]
        content_type: Option<String>,
        /// File size in bytes
        #[arg(long)]
        size: Option<u64>,
    },
    /// Upload file data to an existing upload (TUS PATCH)
    Send {
        /// Upload ID
        id: String,
        /// Path to file to upload
        file: String,
    },
    /// Check upload progress (TUS HEAD)
    Status {
        /// Upload ID
        id: String,
    },
    /// Download an uploaded file
    Download {
        /// Upload ID
        id: String,
        /// Output file path (default: stdout)
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Upload a file via multipart form
    Multipart {
        /// Path to file to upload
        file: String,
    },
    /// Upload raw binary data
    Raw {
        /// Path to file to upload
        file: String,
        /// MIME content type
        #[arg(long)]
        content_type: Option<String>,
    },
    /// Parse a CSV file on the server
    #[command(name = "csv-parse")]
    CsvParse {
        /// Path to CSV file
        file: String,
    },
    /// Preview upload results
    Preview,
    /// Download the upload template
    Template {
        /// Output file path (default: template.csv)
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Discover available upload types
    Discover,
    /// Finalize an upload
    Finalize {
        /// JSON body
        #[arg(long)]
        body: Option<String>,
    },
}

// ── CLI spec generation ───────────────────────────────────────

fn dump_command_spec(cmd: &clap::Command) -> serde_json::Value {
    use serde_json::json;
    let mut obj = serde_json::Map::new();
    obj.insert("name".into(), json!(cmd.get_name()));
    if let Some(about) = cmd.get_about() {
        obj.insert("about".into(), json!(about.to_string()));
    }

    let aliases: Vec<&str> = cmd.get_visible_aliases().collect();
    if !aliases.is_empty() {
        obj.insert("aliases".into(), json!(aliases));
    }

    let args: Vec<serde_json::Value> = cmd
        .get_arguments()
        .filter(|a| !["help", "version"].contains(&a.get_id().as_str()))
        .map(|a| {
            let mut arg = serde_json::Map::new();
            arg.insert("name".into(), json!(a.get_id().as_str()));
            if let Some(long) = a.get_long() {
                arg.insert("long".into(), json!(format!("--{}", long)));
            }
            if let Some(short) = a.get_short() {
                arg.insert("short".into(), json!(format!("-{}", short)));
            }
            if let Some(env) = a.get_env() {
                arg.insert("env".into(), json!(env.to_string_lossy()));
            }
            if let Some(help) = a.get_help() {
                arg.insert("help".into(), json!(help.to_string()));
            }
            arg.insert("required".into(), json!(a.is_required_set()));
            if a.is_global_set() {
                arg.insert("global".into(), json!(true));
            }
            serde_json::Value::Object(arg)
        })
        .collect();
    if !args.is_empty() {
        obj.insert("args".into(), json!(args));
    }

    let subs: Vec<serde_json::Value> = cmd
        .get_subcommands()
        .filter(|s| !s.is_hide_set())
        .map(dump_command_spec)
        .collect();
    if !subs.is_empty() {
        obj.insert("subcommands".into(), json!(subs));
    }

    serde_json::Value::Object(obj)
}

// ── Helpers ──────────────────────────────────────────────────

/// Resolve glow instance URL: --instance-url > active instance > glow_url config > default
fn resolve_glow_url(cli_url: Option<&str>, glow_sub_url: Option<&str>, cfg: &config::Config) -> String {
    cli_url
        .or(glow_sub_url)
        .or_else(|| cfg.active_instance_url())
        .or(cfg.glow_url.as_deref())
        .unwrap_or("http://localhost:8000")
        .to_string()
}

// ── Main ───────────────────────────────────────────────────────

fn main() -> Result<()> {
    // Hidden: dump CLI spec as JSON for docs generation
    if std::env::args().any(|a| a == "--dump-cli-spec") {
        let cmd = Cli::command();
        let spec = dump_command_spec(&cmd);
        println!("{}", serde_json::to_string_pretty(&spec)?);
        return Ok(());
    }

    let cli = Cli::parse();
    let cfg = config::Config::load()?;
    let mode = OutputMode::resolve(cli.json);

    // Resolve URLs: CLI flag/env > config file > default
    let api_url = config::resolve_option(
        cli.api_url.as_deref(),
        cfg.api_url.as_deref(),
        "https://api.learn-loop.org",
    );
    let license_key = cli.license_key.or(cfg.license_key.clone());
    let client_id = config::resolve_option(
        cli.client_id.as_deref(),
        cfg.client_id.as_deref(),
        "api-client",
    );

    use commands::learnloop as ll_cmd;
    use commands::glow as glow_cmd;

    match cli.command {
        // ── Top-level Glow instance commands ─────────────────
        Commands::Login => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            ll_cmd::auth::cmd_login(&glow_url, &client_id, mode)?
        }
        Commands::Logout => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            ll_cmd::auth::cmd_logout(&glow_url, mode)?
        }
        Commands::Health => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            glow_cmd::cmd_health(&client, mode)?
        }
        Commands::Context => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            glow_cmd::cmd_context(&client, mode)?
        }
        Commands::Emulate {
            target_profile_id,
            ttl,
        } => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            glow_cmd::cmd_emulate(&client, &target_profile_id, ttl, mode)?
        }
        Commands::Unemulate => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            glow_cmd::cmd_unemulate(&client, mode)?
        }
        Commands::Generate { group_id, body } => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            glow_cmd::cmd_generate(&client, &group_id, body.as_deref(), mode)?
        }
        Commands::Stream {
            artifact,
            operation,
            entity_id,
            cursor,
        } => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            glow_cmd::cmd_stream(
                &client,
                &artifact,
                &operation,
                entity_id.as_deref(),
                cursor.as_deref(),
                mode,
            )?
        }

        // ── Admin commands (LearnLoop management plane) ──────
        Commands::Admin { action } => {
            dispatch_admin(action, &api_url, &license_key, &client_id, cli.yes, mode)?
        }

        // ── Instance management ──────────────────────────────
        Commands::Instances { action } => {
            dispatch_instances(action, cfg, mode)?
        }
        Commands::Use { name } => {
            let mut cfg = cfg;
            if !cfg.instances.contains_key(&name) {
                anyhow::bail!(
                    "Unknown instance '{}'. Add it first with: glow instances add {} --url <url>",
                    name,
                    name,
                );
            }
            cfg.active_instance = Some(name.clone());
            cfg.save()?;
            if mode == OutputMode::Human {
                use colored::Colorize;
                println!(
                    "{} Now using instance '{}' ({})",
                    "OK".green().bold(),
                    name.bold(),
                    cfg.instances[&name].url.dimmed(),
                );
            }
        }

        // ── Ledger (bridges both) ───────────────────────────
        Commands::Ledger { action } => {
            let key = license_key
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("License key required for ledger operations. Use --license-key or set GLOW_LICENSE_KEY."))?;
            match action {
                LedgerCommands::Verify { path } => {
                    ll_cmd::ledger::cmd_ledger_verify(&path, key, mode)?
                }
                LedgerCommands::Status { path } => {
                    ll_cmd::ledger::cmd_ledger_status(&path, key, mode)?
                }
                LedgerCommands::Sync { path } => {
                    let ll_client = learnloop::LearnLoopClient::new(&api_url, Some(key));
                    ll_cmd::ledger::cmd_ledger_sync(&path, key, &ll_client, mode)?
                }
            }
        }

        // ── Generic resource dispatch ────────────────────────
        Commands::Resource(args) => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                None,
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            dispatch_resource(&client, &args, mode)?
        }

        // ── Legacy glow subcommand (backward compat) ────────
        Commands::Glow {
            instance_url,
            action,
        } => {
            let glow_url = resolve_glow_url(
                cli.instance_url.as_deref(),
                instance_url.as_deref(),
                &cfg,
            );
            let client = glow::GlowClient::new(&glow_url, license_key.as_deref());
            match action {
                GlowCommands::Login => ll_cmd::auth::cmd_login(&glow_url, &client_id, mode)?,
                GlowCommands::Logout => ll_cmd::auth::cmd_logout(&glow_url, mode)?,
                GlowCommands::Health => glow_cmd::cmd_health(&client, mode)?,
                GlowCommands::Personas { action } => match action {
                    PersonaCommands::List => glow_cmd::cmd_persona_list(&client, mode)?,
                    PersonaCommands::Get { id } => {
                        glow_cmd::cmd_persona_get(&client, &id, mode)?
                    }
                    PersonaCommands::Create { name, description } => {
                        glow_cmd::cmd_persona_create(&client, &name, description.as_deref(), mode)?
                    }
                    PersonaCommands::Delete { id } => {
                        glow_cmd::cmd_persona_delete(&client, &id, cli.yes, mode)?
                    }
                },
                GlowCommands::Artifacts {
                    artifact_type,
                    action,
                    body,
                } => {
                    glow_cmd::artifacts::cmd_artifact_action(
                        &client,
                        &artifact_type,
                        &action,
                        body.as_deref(),
                        mode,
                    )?
                }
                GlowCommands::Events { action } => match action {
                    EventCommands::Stream {
                        artifact_type,
                        artifact_id,
                        operation,
                    } => glow_cmd::events::cmd_events_stream(
                        &client,
                        artifact_type.as_deref(),
                        artifact_id.as_deref(),
                        operation.as_deref(),
                        mode,
                    )?,
                    EventCommands::Poll { body } => {
                        glow_cmd::events::cmd_events_poll(&client, body.as_deref(), mode)?
                    }
                    EventCommands::WebhooksDispatch { body } => {
                        glow_cmd::events::cmd_events_webhook_dispatch(
                            &client,
                            body.as_deref(),
                            mode,
                        )?
                    }
                },
                GlowCommands::Upload { action } => match action {
                    UploadCommands::Create {
                        filename,
                        content_type,
                        size,
                    } => glow_cmd::uploads::cmd_upload_create(
                        &client,
                        &filename,
                        content_type.as_deref(),
                        size,
                        mode,
                    )?,
                    UploadCommands::Send { id, file } => {
                        glow_cmd::uploads::cmd_upload_send(&client, &id, &file, mode)?
                    }
                    UploadCommands::Status { id } => {
                        glow_cmd::uploads::cmd_upload_status(&client, &id, mode)?
                    }
                    UploadCommands::Download { id, output } => {
                        glow_cmd::uploads::cmd_upload_download(
                            &client,
                            &id,
                            output.as_deref(),
                            mode,
                        )?
                    }
                    UploadCommands::Multipart { file } => {
                        glow_cmd::uploads::cmd_upload_multipart(&client, &file, mode)?
                    }
                    UploadCommands::Raw { file, content_type } => {
                        glow_cmd::uploads::cmd_upload_raw(
                            &client,
                            &file,
                            content_type.as_deref(),
                            mode,
                        )?
                    }
                    UploadCommands::CsvParse { file } => {
                        glow_cmd::uploads::cmd_upload_csv_parse(&client, &file, mode)?
                    }
                    UploadCommands::Preview => {
                        glow_cmd::uploads::cmd_upload_preview(&client, mode)?
                    }
                    UploadCommands::Template { output } => {
                        glow_cmd::uploads::cmd_upload_template(
                            &client,
                            output.as_deref(),
                            mode,
                        )?
                    }
                    UploadCommands::Discover => {
                        glow_cmd::uploads::cmd_upload_discover(&client, mode)?
                    }
                    UploadCommands::Finalize { body } => {
                        glow_cmd::uploads::cmd_upload_finalize(&client, body.as_deref(), mode)?
                    }
                },
            }
        }
    }

    Ok(())
}

/// Dispatch instance management subcommands
fn dispatch_instances(
    action: InstanceCommands,
    mut cfg: config::Config,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    match action {
        InstanceCommands::List => {
            if cfg.instances.is_empty() {
                if mode == OutputMode::Human {
                    println!("No instances configured. Add one with: glow instances add <name> --url <url>");
                } else {
                    println!("{}", serde_json::json!({ "instances": {}, "active": null }));
                }
            } else if mode == OutputMode::Json {
                println!(
                    "{}",
                    serde_json::json!({
                        "instances": &cfg.instances,
                        "active": &cfg.active_instance,
                    })
                );
            } else {
                println!("{}\n", "Configured Instances".bold());
                for (name, inst) in &cfg.instances {
                    let active = if cfg.active_instance.as_deref() == Some(name.as_str()) {
                        " (active)".green().to_string()
                    } else {
                        String::new()
                    };
                    println!("  {} → {}{}", name.bold(), inst.url.dimmed(), active);
                }
            }
        }
        InstanceCommands::Add { name, url } => {
            cfg.instances.insert(
                name.clone(),
                config::Instance {
                    url: url.trim_end_matches('/').to_string(),
                },
            );
            // If this is the first instance, auto-activate it
            if cfg.instances.len() == 1 {
                cfg.active_instance = Some(name.clone());
            }
            cfg.save()?;
            if mode == OutputMode::Human {
                println!("{} Added instance '{}' ({})", "OK".green().bold(), name.bold(), url.dimmed());
            }
        }
        InstanceCommands::Remove { name } => {
            if cfg.instances.remove(&name).is_none() {
                anyhow::bail!("Instance '{}' not found", name);
            }
            if cfg.active_instance.as_deref() == Some(name.as_str()) {
                cfg.active_instance = None;
            }
            cfg.save()?;
            if mode == OutputMode::Human {
                println!("{} Removed instance '{}'", "OK".green().bold(), name.bold());
            }
        }
    }
    Ok(())
}

/// Dispatch admin subcommands (LearnLoop management plane)
fn dispatch_admin(
    action: AdminCommands,
    api_url: &str,
    license_key: &Option<String>,
    client_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use commands::learnloop as ll_cmd;

    match action {
        AdminCommands::Login => ll_cmd::auth::cmd_login(api_url, client_id, mode)?,
        AdminCommands::Logout => ll_cmd::auth::cmd_logout(api_url, mode)?,
        AdminCommands::Sessions => ll_cmd::auth::cmd_sessions(mode)?,
        AdminCommands::Whoami => {
            let ll = learnloop::LearnLoopClient::new(api_url, license_key.as_deref());
            ll_cmd::auth::cmd_whoami(&ll, mode)?
        }
        AdminCommands::Network => ll_cmd::status::cmd_network(api_url, mode)?,
        AdminCommands::Status => ll_cmd::status::cmd_status(api_url, license_key, mode)?,

        AdminCommands::Licenses { action } => {
            let ll = learnloop::LearnLoopClient::new(api_url, license_key.as_deref());
            match action {
                LicenseCommands::List { active } => {
                    ll_cmd::licenses::cmd_license_list(&ll, active, mode)?
                }
                LicenseCommands::Validate => ll_cmd::licenses::cmd_license_validate(&ll, mode)?,
                LicenseCommands::Create { key, expiry } => {
                    ll_cmd::licenses::cmd_license_create(&ll, &key, &expiry, mode)?
                }
                LicenseCommands::Update {
                    id,
                    key,
                    expiry,
                    active,
                } => ll_cmd::licenses::cmd_license_update(
                    &ll,
                    &id,
                    key.as_deref(),
                    expiry.as_deref(),
                    active,
                    mode,
                )?,
                LicenseCommands::Delete { id } => {
                    ll_cmd::licenses::cmd_license_delete(&ll, &id, yes, mode)?
                }
            }
        }

        AdminCommands::Orgs { action } => {
            let ll = learnloop::LearnLoopClient::new(api_url, license_key.as_deref());
            match action {
                OrgCommands::List => ll_cmd::orgs::cmd_org_list(&ll, mode)?,
                OrgCommands::Create { name, description } => {
                    ll_cmd::orgs::cmd_org_create(&ll, &name, description.as_deref(), mode)?
                }
                OrgCommands::Get { id } => ll_cmd::orgs::cmd_org_get(&ll, &id, mode)?,
                OrgCommands::Update {
                    id,
                    name,
                    description,
                } => ll_cmd::orgs::cmd_org_update(
                    &ll,
                    &id,
                    name.as_deref(),
                    description.as_deref(),
                    mode,
                )?,
                OrgCommands::Delete { id } => {
                    ll_cmd::orgs::cmd_org_delete(&ll, &id, yes, mode)?
                }
                OrgCommands::Members { id, action } => match action {
                    OrgMemberCommands::List => ll_cmd::orgs::cmd_org_members(&ll, &id, mode)?,
                    OrgMemberCommands::Add { email } => {
                        ll_cmd::orgs::cmd_org_member_add(&ll, &id, &email, mode)?
                    }
                    OrgMemberCommands::Remove { user_id } => {
                        ll_cmd::orgs::cmd_org_member_remove(&ll, &id, &user_id, yes, mode)?
                    }
                },
                OrgCommands::License { id, action } => match action {
                    OrgLicenseCommands::Get => ll_cmd::orgs::cmd_org_license(&ll, &id, mode)?,
                    OrgLicenseCommands::Set { license_id } => {
                        ll_cmd::orgs::cmd_org_license_set(&ll, &id, &license_id, mode)?
                    }
                    OrgLicenseCommands::Remove => {
                        ll_cmd::orgs::cmd_org_license_remove(&ll, &id, yes, mode)?
                    }
                },
                OrgCommands::Deployments { id } => {
                    ll_cmd::orgs::cmd_org_deployments(&ll, &id, mode)?
                }
            }
        }

        AdminCommands::Usage { license_id } => {
            let ll = learnloop::LearnLoopClient::new(api_url, license_key.as_deref());
            ll_cmd::usage::cmd_usage(&ll, &license_id, mode)?
        }

        AdminCommands::Deploy { action } => {
            let ll = learnloop::LearnLoopClient::new(api_url, license_key.as_deref());
            match action {
                DeployCommands::Create {
                    license_id,
                    name,
                    subdomain,
                    base_domain,
                    public,
                } => ll_cmd::deploy::cmd_deploy_create(
                    &ll,
                    &license_id,
                    &name,
                    &subdomain,
                    base_domain.as_deref(),
                    !public,
                    mode,
                )?,
                DeployCommands::Stop { id } => {
                    ll_cmd::deploy::cmd_deploy_stop(&ll, &id, yes, mode)?
                }
                DeployCommands::Destroy { id } => {
                    ll_cmd::deploy::cmd_deploy_destroy(&ll, &id, yes, mode)?
                }
                DeployCommands::List { all } => {
                    ll_cmd::deploy::cmd_deploy_list(&ll, !all, mode)?
                }
            }
        }

        AdminCommands::Billing { action } => {
            let ll = learnloop::LearnLoopClient::new(api_url, license_key.as_deref());
            match action {
                BillingCommands::Plans => ll_cmd::billing::cmd_billing_plans(&ll, mode)?,
                BillingCommands::Status { org_id } => {
                    ll_cmd::billing::cmd_billing_status(&ll, &org_id, mode)?
                }
                BillingCommands::Checkout { org_id, plan } => {
                    ll_cmd::billing::cmd_billing_checkout(&ll, &org_id, &plan, mode)?
                }
                BillingCommands::Portal { org_id } => {
                    ll_cmd::billing::cmd_billing_portal(&ll, &org_id, mode)?
                }
            }
        }
    }

    Ok(())
}

/// Dispatch generic resource commands: glow <resource> <action> [--body JSON] [extra args]
/// Also handles media subcommands: glow <resource> <media_type> <action> [flags]
fn dispatch_resource(
    client: &glow::GlowClient,
    args: &[String],
    mode: OutputMode,
) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("Expected a resource name. Run 'glow --help' for usage.");
    }

    let resource_slug = &args[0];
    let resource = resource::Resource::from_slug(resource_slug)
        .ok_or_else(|| anyhow::anyhow!("{}", resource::unknown_resource_error(resource_slug)))?;

    if args.len() < 2 {
        anyhow::bail!(
            "Expected an action for resource '{}'. Example: glow {} search",
            resource.slug(),
            resource.slug(),
        );
    }

    // Check if second arg is a media type (file, image, text, audio, video)
    if let Some(media) = resource::MediaType::from_str(&args[1]) {
        return dispatch_media(client, resource.slug(), media, &args[2..], mode);
    }

    let action = &args[1];
    let body = parse_body_flag(&args[2..])?;

    commands::glow::cmd_resource_action(client, resource.slug(), action, body.as_deref(), mode)
}

/// Dispatch per-resource media operations: glow <resource> <media> <action> [flags]
fn dispatch_media(
    client: &glow::GlowClient,
    resource: &str,
    media: resource::MediaType,
    args: &[String],
    mode: OutputMode,
) -> Result<()> {
    use commands::glow as glow_cmd;

    if args.is_empty() {
        anyhow::bail!(
            "Expected a media action. Available: upload, download, create, chunk, status, finalize, discover, preview"
        );
    }

    let action = args[0].as_str();
    let rest = &args[1..];

    match action {
        "upload" => {
            let file = parse_flag(rest, "--file")?
                .ok_or_else(|| anyhow::anyhow!("--file <path> is required for upload"))?;
            glow_cmd::cmd_media_upload(client, resource, media.slug(), &file, mode)
        }
        "download" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for download"))?;
            let output = parse_flag(rest, "--output")?;
            glow_cmd::cmd_media_download(client, resource, media.slug(), &id, output.as_deref(), mode)
        }
        "create" => {
            let filename = parse_flag(rest, "--filename")?
                .ok_or_else(|| anyhow::anyhow!("--filename is required for TUS create"))?;
            let size = parse_flag(rest, "--size")?
                .map(|s| s.parse::<u64>())
                .transpose()
                .map_err(|_| anyhow::anyhow!("--size must be a number"))?;
            glow_cmd::cmd_media_create(client, resource, media.slug(), &filename, size, mode)
        }
        "chunk" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for chunk"))?;
            let file = parse_flag(rest, "--file")?
                .ok_or_else(|| anyhow::anyhow!("--file <path> is required for chunk"))?;
            let offset = parse_flag(rest, "--offset")?
                .map(|s| s.parse::<u64>())
                .transpose()
                .map_err(|_| anyhow::anyhow!("--offset must be a number"))?
                .unwrap_or(0);
            glow_cmd::cmd_media_chunk(client, resource, media.slug(), &id, &file, offset, mode)
        }
        "status" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for status"))?;
            glow_cmd::cmd_media_status(client, resource, media.slug(), &id, mode)
        }
        "finalize" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for finalize"))?;
            let body = parse_body_flag(rest)?;
            glow_cmd::cmd_media_finalize(client, resource, media.slug(), &id, body.as_deref(), mode)
        }
        "discover" => {
            let id = parse_flag(rest, "--id")?;
            glow_cmd::cmd_media_discover(client, resource, media.slug(), id.as_deref(), mode)
        }
        "preview" => {
            let id = parse_flag(rest, "--id")?
                .ok_or_else(|| anyhow::anyhow!("--id <upload_id> is required for preview"))?;
            glow_cmd::cmd_media_preview(client, resource, media.slug(), &id, mode)
        }
        other => anyhow::bail!(
            "Unknown media action '{}'. Available: upload, download, create, chunk, status, finalize, discover, preview",
            other,
        ),
    }
}

/// Parse --body <json> from a slice of extra args
fn parse_body_flag(args: &[String]) -> Result<Option<String>> {
    parse_flag(args, "--body")
}

/// Parse a --flag <value> pair from a slice of extra args
fn parse_flag(args: &[String], flag: &str) -> Result<Option<String>> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            if i + 1 < args.len() {
                return Ok(Some(args[i + 1].clone()));
            } else {
                anyhow::bail!("{} requires a value", flag);
            }
        }
        i += 1;
    }
    Ok(None)
}
