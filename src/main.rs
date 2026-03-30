// main.rs — Entry point for "glow" / "glw" CLI
//
// Two APIs:
//   - LearnLoop API (central): licensing, billing, OAuth — one instance
//   - Glow API (instance): personas, agents, etc. — many on customer machines
//
// The CLI talks to both.

mod admin;
mod api_common;
mod auth;
mod commands;
mod config;
mod glow;
mod output;
mod resource;


use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use output::OutputMode;

#[derive(Parser)]
#[command(name = "glow", about = "Glow CLI — manage your platform", version)]
struct Cli {
    /// LearnLoop API URL (central platform)
    #[arg(long, env = "GLOW_API_URL")]
    api_url: Option<String>,

    /// Glow instance URL
    #[arg(long, env = "GLOW_INSTANCE_URL")]
    instance_url: Option<String>,

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
        /// TTL in minutes (default: 120 = 2 hours)
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
        /// Event type filter (comma-separated)
        #[arg(long)]
        types: Option<String>,
        /// Max events per batch (1-200)
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Create a stream session (returns session ID)
    Connect,

    /// Destroy a stream session
    Disconnect {
        /// Session ID to destroy
        sid: String,
    },

    /// Report a problem
    Problem {
        /// Problem type
        #[arg(long, alias = "type")]
        kind: String,
        /// Problem description
        #[arg(long)]
        message: String,
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

    /// Platform administration (LearnLoop management plane)
    Admin {
        #[command(subcommand)]
        action: AdminCommands,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },

    /// Interact with a resource on the Glow instance (e.g. glow personas search)
    #[command(external_subcommand)]
    Resource(Vec<String>),
}

// ── Admin subcommands (LearnLoop management plane) ───────────

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
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

    /// Organization management
    #[command(alias = "org")]
    Orgs {
        #[command(subcommand)]
        action: OrgCommands,
    },

    /// View usage summary for an organization
    Usage {
        #[command(subcommand)]
        action: UsageCommands,
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

    /// Manage API keys for the AI gateway
    #[command(alias = "keys")]
    ApiKeys {
        #[command(subcommand)]
        action: ApiKeyCommands,
    },

    /// Manage OAuth clients ("Login with LearnLoop")
    #[command(alias = "oauth")]
    OauthClients {
        #[command(subcommand)]
        action: OAuthClientCommands,
    },

    /// AI gateway information
    Ai {
        #[command(subcommand)]
        action: AiCommands,
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
    /// List organization deployments
    Deployments {
        /// Organization ID
        id: String,
    },
    /// Manage invitations
    Invites {
        /// Organization ID
        id: String,
        #[command(subcommand)]
        action: OrgInviteCommands,
    },
}

#[derive(Subcommand)]
enum OrgInviteCommands {
    /// List invitations
    #[command(alias = "ls")]
    List {
        /// Show all invites (including claimed)
        #[arg(long)]
        all: bool,
    },
    /// Send an invitation
    Send {
        /// Email address to invite
        #[arg(long)]
        email: String,
        /// Role: admin or member
        #[arg(long, default_value = "member")]
        role: String,
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
    /// Update a member's role
    Role {
        /// User ID
        user_id: String,
        /// New role: admin or member
        #[arg(long)]
        set: String,
    },
}

#[derive(Subcommand)]
enum UsageCommands {
    /// Show usage summary for an organization
    Summary {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
    },
    /// Show daily usage breakdown
    Daily {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
}

#[derive(Subcommand)]
enum BillingCommands {
    /// List available plans
    Plans,
    /// Check billing status for an organization
    Status {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
    },
    /// Start checkout for a plan
    Checkout {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
        /// Plan ID (e.g. "pro")
        #[arg(long, default_value = "pro")]
        plan: String,
    },
    /// Open the billing portal
    Portal {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
    },
    /// List invoices for an organization
    Invoices {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
        /// Number of invoices to show
        #[arg(long, default_value = "10")]
        limit: u32,
    },
    /// Show pricing tiers and features
    Pricing,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
enum DeployCommands {
    /// Create a new Glow deployment
    #[command(alias = "new")]
    Create {
        /// Organization ID (defaults to your primary org)
        #[arg(long)]
        org_id: Option<String>,
        /// Deployment name
        #[arg(long)]
        name: String,
        /// Subdomain (e.g. "acme" for acme.learn-loop.org)
        #[arg(long)]
        subdomain: String,
        /// Version (semver, e.g. v1.0.0)
        #[arg(long)]
        version: String,
        /// Base domain (default: learn-loop.org)
        #[arg(long)]
        base_domain: Option<String>,
        /// Component type: api or client (default: api)
        #[arg(long)]
        component_type: Option<String>,
        /// Parent deployment ID (required for client deployments)
        #[arg(long)]
        parent: Option<String>,
        /// Origin URL (e.g. https://acme.learn-loop.org)
        #[arg(long)]
        origin: Option<String>,
        /// CORS client origins (comma-separated)
        #[arg(long)]
        client_origins: Option<String>,
        /// App path prefix
        #[arg(long)]
        app_prefix: Option<String>,
        /// Database backup filename to restore from
        #[arg(long)]
        db_backup: Option<String>,
        /// Hosting type: hosted or self_hosted (default: hosted)
        #[arg(long)]
        hosting_type: Option<String>,
        /// Deploy in airgapped mode (no external network)
        #[arg(long)]
        airgapped: bool,
    },
    /// Update deployment configuration
    Update {
        /// Deployment ID
        id: String,
        /// Subdomain
        #[arg(long)]
        subdomain: Option<String>,
        /// Base domain
        #[arg(long)]
        base_domain: Option<String>,
        /// Origin URL
        #[arg(long)]
        origin: Option<String>,
        /// CORS client origins (comma-separated)
        #[arg(long)]
        client_origins: Option<String>,
        /// App path prefix
        #[arg(long)]
        app_prefix: Option<String>,
        /// Hosting type: hosted or self_hosted
        #[arg(long)]
        hosting_type: Option<String>,
        /// Enable/disable airgapped mode
        #[arg(long)]
        airgapped: Option<bool>,
        /// API key ID for AI gateway
        #[arg(long)]
        api_key_id: Option<String>,
        /// OAuth client ID for authentication
        #[arg(long)]
        oauth_client_id: Option<String>,
    },
    /// Check deployment status and workflow progress
    Status {
        /// Deployment ID
        id: String,
    },
    /// View container logs
    Logs {
        /// Deployment ID
        id: String,
        /// Service name (e.g. api, database, redis)
        #[arg(long, default_value = "api")]
        service: String,
        /// Number of log lines
        #[arg(long, default_value = "100")]
        lines: u32,
    },
    /// Check container health
    Health {
        /// Deployment ID
        id: String,
    },
    /// View CPU/memory metrics
    Metrics {
        /// Deployment ID
        id: String,
    },
    /// View deployment event history
    Events {
        /// Deployment ID
        id: String,
    },
    /// Trigger a redeploy
    Redeploy {
        /// Deployment ID
        id: String,
    },
    /// Update deployment version
    #[command(alias = "upgrade")]
    Version {
        /// Deployment ID
        id: String,
        /// New version (semver, e.g. v2.1.0)
        #[arg(long)]
        set: String,
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
    /// Create a database backup
    BackupCreate {
        /// Deployment ID
        id: String,
    },
    /// List database backups
    #[command(alias = "backups")]
    BackupList {
        /// Deployment ID
        id: String,
    },
    /// Restore a database backup
    BackupRestore {
        /// Deployment ID
        id: String,
        /// Backup filename
        #[arg(long)]
        file: String,
    },
    /// Delete a database backup
    BackupDelete {
        /// Deployment ID
        id: String,
        /// Backup filename
        #[arg(long)]
        file: String,
    },
    /// List available versions for deployment
    Versions {
        /// Component type: api or client
        #[arg(long, default_value = "api")]
        component_type: String,
    },
}

#[derive(Subcommand)]
enum ApiKeyCommands {
    /// Create a new API key
    #[command(alias = "new")]
    Create {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
        /// Key name
        #[arg(long)]
        name: Option<String>,
        /// Scopes (comma-separated, e.g. "ai")
        #[arg(long)]
        scopes: Option<String>,
        /// Spend limit in cents
        #[arg(long)]
        spend_limit: Option<i64>,
    },
    /// List API keys for an organization
    #[command(alias = "ls")]
    List {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
    },
    /// Revoke an API key
    #[command(alias = "rm")]
    Revoke {
        /// Organization ID (defaults to your primary org)
        #[arg(long)]
        org_id: Option<String>,
        /// Key ID to revoke
        key_id: String,
    },
    /// View AI usage for API keys
    Usage {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
        /// Number of days to show (default: 30)
        #[arg(long, default_value = "30")]
        days: u32,
    },
}

#[derive(Subcommand)]
enum OAuthClientCommands {
    /// Register a new OAuth client
    #[command(alias = "new")]
    Create {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
        /// Client app name
        #[arg(long)]
        name: String,
        /// Redirect URIs (comma-separated)
        #[arg(long)]
        redirect_uris: String,
    },
    /// List OAuth clients for an organization
    #[command(alias = "ls")]
    List {
        /// Organization ID (defaults to your primary org)
        org_id: Option<String>,
    },
    /// Update an OAuth client
    Update {
        /// Organization ID (defaults to your primary org)
        #[arg(long)]
        org_id: Option<String>,
        /// Client ID to update
        client_id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New redirect URIs (comma-separated)
        #[arg(long)]
        redirect_uris: Option<String>,
    },
    /// Revoke an OAuth client
    #[command(alias = "rm")]
    Revoke {
        /// Organization ID (defaults to your primary org)
        #[arg(long)]
        org_id: Option<String>,
        /// Client ID to revoke
        client_id: String,
    },
}

#[derive(Subcommand)]
enum AiCommands {
    /// Show AI gateway pricing and model catalog
    Pricing,
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

/// Resolve org_id: explicit arg > config > error
fn require_org_id(explicit: Option<String>, cfg: &config::Config) -> Result<String> {
    explicit.or_else(|| cfg.org_id.clone()).ok_or_else(|| {
        anyhow::anyhow!(
            "Organization ID required. Pass it as an argument, or run 'glow admin login' to set a default."
        )
    })
}

/// Resolve glow instance URL: --instance-url > active instance > glow_url config > default
fn resolve_glow_url(cli_url: Option<&str>, cfg: &config::Config) -> String {
    cli_url
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
        let mut spec = dump_command_spec(&cmd);

        // Inject dynamic resource definitions (not captured by clap)
        if let Some(obj) = spec.as_object_mut() {
            use serde_json::json;

            let resources: Vec<serde_json::Value> = resource::Resource::all()
                .iter()
                .map(|r| json!({ "slug": r.slug(), "about": r.about() }))
                .collect();
            obj.insert("resources".into(), json!(resources));

            let media_types: Vec<&str> = resource::MediaType::all_slugs().to_vec();
            obj.insert("media_types".into(), json!(media_types));

            obj.insert("media_actions".into(), json!([
                { "name": "upload", "about": "Upload a file via multipart form", "args": [
                    { "name": "file", "long": "--file", "required": true, "help": "Path to file to upload" }
                ]},
                { "name": "download", "about": "Download a media file", "args": [
                    { "name": "id", "long": "--id", "required": true, "help": "Upload ID" },
                    { "name": "output", "long": "--output", "required": false, "help": "Output file path (stdout if omitted)" }
                ]},
                { "name": "create", "about": "Initiate a TUS resumable upload", "args": [
                    { "name": "filename", "long": "--filename", "required": true, "help": "Filename for the upload" },
                    { "name": "size", "long": "--size", "required": false, "help": "Total file size in bytes" }
                ]},
                { "name": "chunk", "about": "Upload a chunk for a TUS upload", "args": [
                    { "name": "id", "long": "--id", "required": true, "help": "Upload ID" },
                    { "name": "file", "long": "--file", "required": true, "help": "Path to chunk data" },
                    { "name": "offset", "long": "--offset", "required": false, "help": "Byte offset (default: 0)" }
                ]},
                { "name": "status", "about": "Check TUS upload status", "args": [
                    { "name": "id", "long": "--id", "required": true, "help": "Upload ID" }
                ]},
                { "name": "finalize", "about": "Finalize a TUS upload", "args": [
                    { "name": "id", "long": "--id", "required": true, "help": "Upload ID" },
                    { "name": "body", "long": "--body", "required": false, "help": "Optional JSON body" }
                ]},
                { "name": "discover", "about": "Discover available upload types", "args": [
                    { "name": "id", "long": "--id", "required": false, "help": "Optional upload ID" }
                ]},
                { "name": "preview", "about": "Preview a media file", "args": [
                    { "name": "id", "long": "--id", "required": true, "help": "Upload ID" }
                ]}
            ]));
        }

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
    let client_id = config::resolve_option(
        cli.client_id.as_deref(),
        cfg.client_id.as_deref(),
        "api-client",
    );

    use commands::admin as admin_cmd;
    use commands::glow as glow_cmd;

    match cli.command {
        // ── Top-level Glow instance commands ─────────────────
        Commands::Login => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            admin_cmd::auth::cmd_instance_login(&glow_url, &api_url, mode)?
        }
        Commands::Logout => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            admin_cmd::auth::cmd_logout(&glow_url, mode)?
        }
        Commands::Health => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_health(&client, mode)?
        }
        Commands::Context => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_context(&client, mode)?
        }
        Commands::Emulate {
            target_profile_id,
            ttl,
        } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_emulate(&client, &target_profile_id, ttl, mode)?
        }
        Commands::Unemulate => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_unemulate(&client, mode)?
        }
        Commands::Generate { group_id, body } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_generate(&client, &group_id, body.as_deref(), mode)?
        }
        Commands::Stream {
            artifact,
            operation,
            entity_id,
            cursor,
            types,
            limit,
        } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_stream(
                &client,
                &artifact,
                &operation,
                entity_id.as_deref(),
                cursor.as_deref(),
                types.as_deref(),
                limit,
                mode,
            )?
        }
        Commands::Connect => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_connect(&client, mode)?
        }
        Commands::Disconnect { sid } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_disconnect(&client, &sid, mode)?
        }
        Commands::Problem { kind, message } => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            glow_cmd::cmd_problem(&client, &kind, &message, mode)?
        }

        // ── Shell completions ────────────────────────────────
        Commands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "glow", &mut std::io::stdout());
        }

        // ── Admin commands (LearnLoop management plane) ──────
        Commands::Admin { action } => {
            dispatch_admin(action, &api_url, &client_id, cli.yes, mode, cfg)?
        }

        // ── Instance management ──────────────────────────────
        Commands::Instances { action } => dispatch_instances(action, cfg, mode)?,
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

        // ── Generic resource dispatch ────────────────────────
        Commands::Resource(args) => {
            let glow_url = resolve_glow_url(cli.instance_url.as_deref(), &cfg);
            let client = glow::GlowClient::new(&glow_url);
            dispatch_resource(&client, &args, mode)?
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
                println!(
                    "{} Added instance '{}' ({})",
                    "OK".green().bold(),
                    name.bold(),
                    url.dimmed()
                );
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
    client_id: &str,
    yes: bool,
    mode: OutputMode,
    mut cfg: config::Config,
) -> Result<()> {
    use commands::admin as admin_cmd;

    match action {
        AdminCommands::Login => admin_cmd::auth::cmd_login(api_url, client_id, &mut cfg, mode)?,
        AdminCommands::Logout => admin_cmd::auth::cmd_logout(api_url, mode)?,
        AdminCommands::Sessions => admin_cmd::auth::cmd_sessions(mode)?,
        AdminCommands::Whoami => {
            let ll = admin::AdminClient::new(api_url);
            admin_cmd::auth::cmd_whoami(&ll, mode)?
        }
        AdminCommands::Network => admin_cmd::status::cmd_network(api_url, mode)?,
        AdminCommands::Status => admin_cmd::status::cmd_status(api_url, mode)?,

        AdminCommands::Orgs { action } => {
            let ll = admin::AdminClient::new(api_url);
            match action {
                OrgCommands::List => admin_cmd::orgs::cmd_org_list(&ll, mode)?,
                OrgCommands::Create { name, description } => {
                    admin_cmd::orgs::cmd_org_create(&ll, &name, description.as_deref(), mode)?
                }
                OrgCommands::Get { id } => admin_cmd::orgs::cmd_org_get(&ll, &id, mode)?,
                OrgCommands::Update {
                    id,
                    name,
                    description,
                } => admin_cmd::orgs::cmd_org_update(
                    &ll,
                    &id,
                    name.as_deref(),
                    description.as_deref(),
                    mode,
                )?,
                OrgCommands::Delete { id } => admin_cmd::orgs::cmd_org_delete(&ll, &id, yes, mode)?,
                OrgCommands::Members { id, action } => match action {
                    OrgMemberCommands::List => admin_cmd::orgs::cmd_org_members(&ll, &id, mode)?,
                    OrgMemberCommands::Add { email } => {
                        admin_cmd::orgs::cmd_org_member_add(&ll, &id, &email, mode)?
                    }
                    OrgMemberCommands::Remove { user_id } => {
                        admin_cmd::orgs::cmd_org_member_remove(&ll, &id, &user_id, yes, mode)?
                    }
                    OrgMemberCommands::Role { user_id, set } => {
                        admin_cmd::orgs::cmd_org_member_update_role(&ll, &id, &user_id, &set, mode)?
                    }
                },
                OrgCommands::Deployments { id } => {
                    admin_cmd::orgs::cmd_org_deployments(&ll, &id, mode)?
                }
                OrgCommands::Invites { id, action } => match action {
                    OrgInviteCommands::List { all } => {
                        admin_cmd::orgs::cmd_org_invites_list(&ll, &id, !all, mode)?
                    }
                    OrgInviteCommands::Send { email, role } => {
                        admin_cmd::orgs::cmd_org_invite(&ll, &id, &email, &role, mode)?
                    }
                },
            }
        }

        AdminCommands::Usage { action } => {
            let ll = admin::AdminClient::new(api_url);
            match action {
                UsageCommands::Summary { org_id } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::usage::cmd_usage(&ll, &oid, mode)?
                }
                UsageCommands::Daily { org_id, from, to } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::usage::cmd_usage_daily(
                        &ll,
                        &oid,
                        from.as_deref(),
                        to.as_deref(),
                        mode,
                    )?
                }
            }
        }

        AdminCommands::Deploy { action } => {
            let ll = admin::AdminClient::new(api_url);
            match action {
                DeployCommands::Create {
                    org_id,
                    name,
                    subdomain,
                    version,
                    base_domain,
                    component_type,
                    parent,
                    origin,
                    client_origins,
                    app_prefix,
                    db_backup,
                    hosting_type,
                    airgapped,
                } => {
                    let mut body = serde_json::json!({
                        "organization_id": require_org_id(org_id, &cfg)?,
                        "deployment_name": name,
                        "subdomain": subdomain,
                        "version": version,
                    });
                    let obj = body.as_object_mut().unwrap();
                    if let Some(v) = base_domain {
                        obj.insert("base_domain".into(), v.into());
                    }
                    if let Some(v) = component_type {
                        obj.insert("component_type".into(), v.into());
                    }
                    if let Some(v) = parent {
                        obj.insert("parent_deployment_id".into(), v.into());
                    }
                    if let Some(v) = origin {
                        obj.insert("origin".into(), v.into());
                    }
                    if let Some(v) = client_origins {
                        let origins: Vec<&str> = v.split(',').map(|s| s.trim()).collect();
                        obj.insert("client_origins".into(), serde_json::json!(origins));
                    }
                    if let Some(v) = app_prefix {
                        obj.insert("app_prefix".into(), v.into());
                    }
                    if let Some(v) = db_backup {
                        obj.insert("db_backup".into(), v.into());
                    }
                    if let Some(v) = hosting_type {
                        obj.insert("hosting_type".into(), v.into());
                    }
                    if airgapped {
                        obj.insert("airgapped".into(), true.into());
                    }
                    admin_cmd::deploy::cmd_deploy_create_raw(&ll, body, mode)?
                }
                DeployCommands::Update {
                    id,
                    subdomain,
                    base_domain,
                    origin,
                    client_origins,
                    app_prefix,
                    hosting_type,
                    airgapped,
                    api_key_id,
                    oauth_client_id,
                } => admin_cmd::deploy::cmd_deploy_update(
                    &ll,
                    &id,
                    subdomain,
                    base_domain,
                    origin,
                    client_origins,
                    app_prefix,
                    hosting_type,
                    airgapped,
                    api_key_id,
                    oauth_client_id,
                    mode,
                )?,
                DeployCommands::Status { id } => {
                    admin_cmd::deploy::cmd_deploy_status(&ll, &id, mode)?
                }
                DeployCommands::Logs { id, service, lines } => {
                    admin_cmd::deploy::cmd_deploy_logs(&ll, &id, &service, lines, mode)?
                }
                DeployCommands::Health { id } => {
                    admin_cmd::deploy::cmd_deploy_health(&ll, &id, mode)?
                }
                DeployCommands::Metrics { id } => {
                    admin_cmd::deploy::cmd_deploy_metrics(&ll, &id, mode)?
                }
                DeployCommands::Events { id } => {
                    admin_cmd::deploy::cmd_deploy_events(&ll, &id, mode)?
                }
                DeployCommands::Redeploy { id } => {
                    admin_cmd::deploy::cmd_deploy_redeploy(&ll, &id, yes, mode)?
                }
                DeployCommands::Version { id, set } => {
                    admin_cmd::deploy::cmd_deploy_version(&ll, &id, &set, yes, mode)?
                }
                DeployCommands::Stop { id } => {
                    admin_cmd::deploy::cmd_deploy_stop(&ll, &id, yes, mode)?
                }
                DeployCommands::Destroy { id } => {
                    admin_cmd::deploy::cmd_deploy_destroy(&ll, &id, yes, mode)?
                }
                DeployCommands::List { all } => {
                    admin_cmd::deploy::cmd_deploy_list(&ll, !all, mode)?
                }
                DeployCommands::BackupCreate { id } => {
                    admin_cmd::deploy::cmd_deploy_backup_create(&ll, &id, mode)?
                }
                DeployCommands::BackupList { id } => {
                    admin_cmd::deploy::cmd_deploy_backup_list(&ll, &id, mode)?
                }
                DeployCommands::BackupRestore { id, file } => {
                    admin_cmd::deploy::cmd_deploy_backup_restore(&ll, &id, &file, yes, mode)?
                }
                DeployCommands::BackupDelete { id, file } => {
                    admin_cmd::deploy::cmd_deploy_backup_delete(&ll, &id, &file, yes, mode)?
                }
                DeployCommands::Versions { component_type } => {
                    admin_cmd::deploy::cmd_deploy_versions(&ll, &component_type, mode)?
                }
            }
        }

        AdminCommands::Billing { action } => {
            let ll = admin::AdminClient::new(api_url);
            match action {
                BillingCommands::Plans => admin_cmd::billing::cmd_billing_plans(&ll, mode)?,
                BillingCommands::Status { org_id } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::billing::cmd_billing_status(&ll, &oid, mode)?
                }
                BillingCommands::Checkout { org_id, plan } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::billing::cmd_billing_checkout(&ll, &oid, &plan, mode)?
                }
                BillingCommands::Portal { org_id } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::billing::cmd_billing_portal(&ll, &oid, mode)?
                }
                BillingCommands::Invoices { org_id, limit } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::billing::cmd_billing_invoices(&ll, &oid, limit, mode)?
                }
                BillingCommands::Pricing => admin_cmd::billing::cmd_billing_pricing(&ll, mode)?,
            }
        }

        AdminCommands::ApiKeys { action } => {
            let ll = admin::AdminClient::new(api_url);
            match action {
                ApiKeyCommands::Create {
                    org_id,
                    name,
                    scopes,
                    spend_limit,
                } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    let scope_vec =
                        scopes.map(|s| s.split(',').map(|v| v.trim().to_string()).collect());
                    admin_cmd::api_keys::cmd_api_keys_create(
                        &ll,
                        &oid,
                        name.as_deref(),
                        scope_vec,
                        spend_limit,
                        mode,
                    )?
                }
                ApiKeyCommands::List { org_id } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::api_keys::cmd_api_keys_list(&ll, &oid, mode)?
                }
                ApiKeyCommands::Revoke { org_id, key_id } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::api_keys::cmd_api_keys_revoke(&ll, &oid, &key_id, yes, mode)?
                }
                ApiKeyCommands::Usage { org_id, days } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::api_keys::cmd_api_keys_usage(&ll, &oid, days, mode)?
                }
            }
        }

        AdminCommands::OauthClients { action } => {
            let ll = admin::AdminClient::new(api_url);
            match action {
                OAuthClientCommands::Create {
                    org_id,
                    name,
                    redirect_uris,
                } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    let uris: Vec<String> = redirect_uris
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    admin_cmd::oauth_clients::cmd_oauth_clients_create(
                        &ll, &oid, &name, uris, mode,
                    )?
                }
                OAuthClientCommands::List { org_id } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::oauth_clients::cmd_oauth_clients_list(&ll, &oid, mode)?
                }
                OAuthClientCommands::Update {
                    org_id,
                    client_id,
                    name,
                    redirect_uris,
                } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    let uris =
                        redirect_uris.map(|u| u.split(',').map(|s| s.trim().to_string()).collect());
                    admin_cmd::oauth_clients::cmd_oauth_clients_update(
                        &ll,
                        &oid,
                        &client_id,
                        name.as_deref(),
                        uris,
                        mode,
                    )?
                }
                OAuthClientCommands::Revoke { org_id, client_id } => {
                    let oid = require_org_id(org_id, &cfg)?;
                    admin_cmd::oauth_clients::cmd_oauth_clients_revoke(
                        &ll, &oid, &client_id, yes, mode,
                    )?
                }
            }
        }

        AdminCommands::Ai { action } => {
            let ll = admin::AdminClient::new(api_url);
            match action {
                AiCommands::Pricing => {
                    use colored::Colorize;

                    let resp = ll.ai_pricing()?;
                    output::print_result(mode, &resp, |r| {
                        println!("{}\n", "AI Gateway Pricing".bold());
                        for tier in &r.tiers {
                            println!("  {} — {}", tier.name.bold(), tier.cost);
                            println!("    {}", tier.description.dimmed());
                            for model in &tier.models {
                                let provider = model
                                    .provider
                                    .as_deref()
                                    .map(|p| format!(" ({})", p))
                                    .unwrap_or_default();
                                println!(
                                    "      {} [{}]{} — {}",
                                    model.name.bold(),
                                    model.r#type.dimmed(),
                                    provider.dimmed(),
                                    model.description.as_deref().unwrap_or(""),
                                );
                            }
                            println!();
                        }
                    });
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

    // Show help for resource action
    if args[2..].iter().any(|a| a == "--help" || a == "-h") {
        use colored::Colorize;
        println!("{}\n", format!("glow {} {}", resource.slug(), action).bold());
        println!("  {} /{}/{}\n", "POST".dimmed(), resource.slug(), action);
        println!("  Pass --key value pairs as needed. The API will validate parameters.\n");
        println!("{}:", "Options".bold());
        println!("  {:<30} Raw JSON body (can combine with flags)", "--body <json>".green());
        println!("  {:<30} Output as JSON", "--json".green());
        return Ok(());
    }

    let body = build_resource_body(&args[2..])?;

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
            let body = parse_flag(rest, "--body")?;
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

/// Build a JSON body string from extra args.
///
/// Supports two styles (can be combined):
///   --body '{"key": "value"}'         explicit JSON (merged first)
///   --key value --flag true           individual params folded into body
///
/// Values are auto-coerced: "true"/"false" → bool, integers/floats → number,
/// "null" → null, everything else → string.
fn build_resource_body(args: &[String]) -> Result<Option<String>> {
    use serde_json::{json, Map, Value};

    let explicit = parse_flag(args, "--body")?;
    let params = parse_params(args)?;

    if explicit.is_none() && params.is_empty() {
        return Ok(None);
    }

    // Start with explicit --body JSON if provided
    let mut obj: Map<String, Value> = match &explicit {
        Some(s) => {
            let v: Value = serde_json::from_str(s)
                .map_err(|e| anyhow::anyhow!("Invalid JSON for --body: {}", e))?;
            match v {
                Value::Object(m) => m,
                _ => anyhow::bail!("--body must be a JSON object"),
            }
        }
        None => Map::new(),
    };

    // Merge --key value params (individual flags override --body keys)
    for (k, v) in params {
        obj.insert(k, coerce_value(&v));
    }

    Ok(Some(json!(obj).to_string()))
}

/// Parse --key value pairs from args, skipping --body and known media flags.
fn parse_params(args: &[String]) -> Result<Vec<(String, String)>> {
    let skip = [
        "--body",
        "--file",
        "--id",
        "--output",
        "--filename",
        "--size",
        "--offset",
    ];
    let mut pairs = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if let Some(key) = args[i].strip_prefix("--") {
            if skip.contains(&args[i].as_str()) {
                i += 2; // skip flag + value
                continue;
            }
            if i + 1 >= args.len() {
                anyhow::bail!("--{} requires a value", key);
            }
            pairs.push((key.replace('-', "_"), args[i + 1].clone()));
            i += 2;
        } else {
            i += 1;
        }
    }
    Ok(pairs)
}

/// Auto-coerce a string value to the most appropriate JSON type.
fn coerce_value(s: &str) -> serde_json::Value {
    use serde_json::Value;
    match s {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "null" => Value::Null,
        _ => {
            if let Ok(n) = s.parse::<i64>() {
                Value::Number(n.into())
            } else if let Ok(f) = s.parse::<f64>() {
                serde_json::Number::from_f64(f)
                    .map(Value::Number)
                    .unwrap_or_else(|| Value::String(s.to_string()))
            } else {
                Value::String(s.to_string())
            }
        }
    }
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
