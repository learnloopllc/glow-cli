// main.rs — Entry point for "learnloop" / "ll" CLI
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

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
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

    /// OAuth client ID
    #[arg(long, env = "LEARNLOOP_CLIENT_ID")]
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
    /// Authenticate with LearnLoop API (OAuth)
    Login,

    /// Remove stored authentication token
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

// Glow subcommands
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
        .map(|s| dump_command_spec(s))
        .collect();
    if !subs.is_empty() {
        obj.insert("subcommands".into(), json!(subs));
    }

    serde_json::Value::Object(obj)
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
    let license_key = cli.license_key.or(cfg.license_key);
    let client_id = config::resolve_option(
        cli.client_id.as_deref(),
        cfg.client_id.as_deref(),
        "api-client",
    );

    use commands::learnloop as ll_cmd;
    use commands::glow as glow_cmd;

    match cli.command {
        Commands::Login => ll_cmd::auth::cmd_login(&api_url, &client_id, mode)?,
        Commands::Logout => ll_cmd::auth::cmd_logout(&api_url, mode)?,
        Commands::Sessions => ll_cmd::auth::cmd_sessions(mode)?,
        Commands::Whoami => {
            let ll = learnloop::LearnLoopClient::new(&api_url, license_key.as_deref());
            ll_cmd::auth::cmd_whoami(&ll, mode)?
        }
        Commands::Network => ll_cmd::status::cmd_network(&api_url, mode)?,
        Commands::Status => ll_cmd::status::cmd_status(&api_url, &license_key, mode)?,

        Commands::Licenses { action } => {
            let ll = learnloop::LearnLoopClient::new(&api_url, license_key.as_deref());
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
                    ll_cmd::licenses::cmd_license_delete(&ll, &id, cli.yes, mode)?
                }
            }
        }

        Commands::Orgs { action } => {
            let ll = learnloop::LearnLoopClient::new(&api_url, license_key.as_deref());
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
                    ll_cmd::orgs::cmd_org_delete(&ll, &id, cli.yes, mode)?
                }
                OrgCommands::Members { id, action } => match action {
                    OrgMemberCommands::List => ll_cmd::orgs::cmd_org_members(&ll, &id, mode)?,
                    OrgMemberCommands::Add { email } => {
                        ll_cmd::orgs::cmd_org_member_add(&ll, &id, &email, mode)?
                    }
                    OrgMemberCommands::Remove { user_id } => {
                        ll_cmd::orgs::cmd_org_member_remove(&ll, &id, &user_id, cli.yes, mode)?
                    }
                },
                OrgCommands::License { id, action } => match action {
                    OrgLicenseCommands::Get => ll_cmd::orgs::cmd_org_license(&ll, &id, mode)?,
                    OrgLicenseCommands::Set { license_id } => {
                        ll_cmd::orgs::cmd_org_license_set(&ll, &id, &license_id, mode)?
                    }
                    OrgLicenseCommands::Remove => {
                        ll_cmd::orgs::cmd_org_license_remove(&ll, &id, cli.yes, mode)?
                    }
                },
                OrgCommands::Deployments { id } => {
                    ll_cmd::orgs::cmd_org_deployments(&ll, &id, mode)?
                }
            }
        }

        Commands::Usage { license_id } => {
            let ll = learnloop::LearnLoopClient::new(&api_url, license_key.as_deref());
            ll_cmd::usage::cmd_usage(&ll, &license_id, mode)?
        }

        Commands::Deploy { action } => {
            let ll = learnloop::LearnLoopClient::new(&api_url, license_key.as_deref());
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
                    ll_cmd::deploy::cmd_deploy_stop(&ll, &id, cli.yes, mode)?
                }
                DeployCommands::Destroy { id } => {
                    ll_cmd::deploy::cmd_deploy_destroy(&ll, &id, cli.yes, mode)?
                }
                DeployCommands::List { all } => {
                    ll_cmd::deploy::cmd_deploy_list(&ll, !all, mode)?
                }
            }
        }

        Commands::Billing { action } => {
            let ll = learnloop::LearnLoopClient::new(&api_url, license_key.as_deref());
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

        Commands::Ledger { action } => {
            let key = license_key
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("License key required for ledger operations. Use --license-key or set LEARNLOOP_LICENSE_KEY."))?;
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

        Commands::Glow {
            instance_url,
            action,
        } => {
            let glow_url = config::resolve_option(
                instance_url.as_deref(),
                cfg.glow_url.as_deref(),
                "http://localhost:8000",
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
