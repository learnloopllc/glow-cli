use crate::admin::AdminClient;
use crate::auth;
use crate::output::{self, OutputMode};
use anyhow::Result;

/// Simple login (for Glow instances — no config changes)
pub(crate) fn cmd_instance_login(
    server_url: &str,
    client_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct LoginResult {
        server: String,
        status: String,
    }

    let _token = auth::login(server_url, client_id)?;

    output::print_result(
        mode,
        &LoginResult {
            server: server_url.to_string(),
            status: "authenticated".into(),
        },
        |_| {
            println!(
                "{} Logged in to {}",
                "OK".green().bold(),
                server_url.dimmed()
            );
        },
    );
    Ok(())
}

/// Admin login — saves API URL and default org to config
pub(crate) fn cmd_login(
    server_url: &str,
    client_id: &str,
    cfg: &mut crate::config::Config,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct LoginResult {
        server: String,
        status: String,
        org_id: Option<String>,
    }

    let _token = auth::login(server_url, client_id)?;

    // Save API URL to config
    cfg.api_url = Some(server_url.to_string());

    // Fetch user info to get default org
    let client = crate::admin::AdminClient::new(server_url);
    let org_id = match client.whoami() {
        Ok(me) => {
            if let Some(ref oid) = me.org_id {
                cfg.org_id = Some(oid.clone());
            }
            me.org_id
        }
        Err(_) => None,
    };

    cfg.save()?;

    output::print_result(
        mode,
        &LoginResult {
            server: server_url.to_string(),
            status: "authenticated".into(),
            org_id: org_id.clone(),
        },
        |_| {
            println!(
                "{} Logged in to {}",
                "OK".green().bold(),
                server_url.dimmed()
            );
            if let Some(ref oid) = org_id {
                println!("  Default org: {}", oid.dimmed());
            }
        },
    );
    Ok(())
}

pub(crate) fn cmd_logout(server_url: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct LogoutResult {
        server: String,
        removed: bool,
    }

    let removed = auth::remove_token(server_url)?;

    output::print_result(
        mode,
        &LogoutResult {
            server: server_url.to_string(),
            removed,
        },
        |r| {
            if r.removed {
                println!(
                    "{} Logged out of {}",
                    "OK".green().bold(),
                    server_url.dimmed()
                );
            } else {
                println!("No session found for {}", server_url.dimmed());
            }
        },
    );
    Ok(())
}

pub(crate) fn cmd_sessions(mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct SessionList {
        sessions: Vec<SessionInfo>,
    }

    #[derive(Serialize)]
    struct SessionInfo {
        server: String,
        token_type: String,
        issued_at: u64,
    }

    let sessions_data: Vec<SessionInfo> = auth::list_sessions()?
        .into_iter()
        .map(|(url, tok)| SessionInfo {
            server: url,
            token_type: tok.token_type,
            issued_at: tok.issued_at,
        })
        .collect();

    let list = SessionList {
        sessions: sessions_data,
    };

    output::print_result(mode, &list, |l| {
        if l.sessions.is_empty() {
            println!("No active sessions. Run 'glow login' to authenticate.");
        } else {
            println!("{} ({} active)\n", "Sessions".bold(), l.sessions.len());
            for s in &l.sessions {
                println!("  {} [{}]", s.server, s.token_type.dimmed());
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_whoami(client: &AdminClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.whoami()?;
    output::print_result(mode, &resp, |r| {
        println!("{}", r.name.bold());
        println!("  Email:  {}", r.email);
        println!("  ID:     {}", r.user_id.dimmed());
        if r.is_superadmin {
            println!("  Role:   {}", "superadmin".yellow());
        }
        if let Some(org_id) = &r.org_id {
            let role = r.org_role.as_deref().unwrap_or("member");
            println!("  Org:    {} [{}]", org_id.dimmed(), role);
        }
        if let Some(idp) = &r.idp {
            println!("  IDP:    {}", idp);
        }
        if let Some(gh) = &r.github_username {
            println!("  GitHub: {}", gh);
        }
    });
    Ok(())
}
