use crate::admin::AdminClient;
use crate::output::{self, OutputMode};
use anyhow::Result;

pub(crate) fn cmd_oauth_clients_create(
    client: &AdminClient,
    org_id: &str,
    name: &str,
    redirect_uris: Vec<String>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.oauth_clients_create(org_id, name, redirect_uris)?;
    output::print_result(mode, &resp, |r| {
        println!("{} OAuth client created", "OK".green().bold());
        println!("  ID:        {}", r.id.dimmed());
        println!("  Client ID: {}", r.client_id);
        println!("  Name:      {}", r.name);
        if let Some(uris) = &r.redirect_uris {
            println!("  Redirects: {}", uris.join(", "));
        }
        if let Some(secret) = &r.client_secret {
            println!(
                "\n  {} {}",
                "Secret:".bold(),
                secret.yellow()
            );
            println!(
                "  {}",
                "Save this secret now — it won't be shown again.".dimmed()
            );
        }
    });
    Ok(())
}

pub(crate) fn cmd_oauth_clients_list(
    client: &AdminClient,
    org_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.oauth_clients_list(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} ({} total)\n",
            "OAuth Clients".bold(),
            r.clients.len()
        );
        for c in &r.clients {
            println!(
                "  {} {} [{}]",
                c.id.dimmed(),
                c.name.bold(),
                c.client_id.dimmed()
            );
            if let Some(uris) = &c.redirect_uris {
                println!("    Redirects: {}", uris.join(", "));
            }
            if let Some(scopes) = &c.scopes {
                println!("    Scopes:    {}", scopes.join(", "));
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_oauth_clients_update(
    client: &AdminClient,
    org_id: &str,
    client_id: &str,
    name: Option<&str>,
    redirect_uris: Option<Vec<String>>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.oauth_clients_update(org_id, client_id, name, redirect_uris)?;
    output::print_result(mode, &resp, |r| {
        println!("{} OAuth client updated", "OK".green().bold());
        println!("  Client ID: {}", r.client_id);
        println!("  Name:      {}", r.name);
    });
    Ok(())
}

pub(crate) fn cmd_oauth_clients_revoke(
    client: &AdminClient,
    org_id: &str,
    client_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Revoke OAuth client {}?", client_id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.oauth_clients_revoke(org_id, client_id)?;
    output::print_result(mode, &resp, |_| {
        println!("{} OAuth client revoked", "OK".green().bold());
    });
    Ok(())
}
