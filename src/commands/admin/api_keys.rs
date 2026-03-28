use crate::admin::AdminClient;
use crate::output::{self, OutputMode};
use anyhow::Result;

pub(crate) fn cmd_api_keys_create(
    client: &AdminClient,
    org_id: &str,
    name: Option<&str>,
    scopes: Option<Vec<String>>,
    spend_limit_cents: Option<i64>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.api_keys_create(org_id, name, scopes, spend_limit_cents)?;
    output::print_result(mode, &resp, |r| {
        println!("{} API key created", "OK".green().bold());
        println!("  ID:     {}", r.id.dimmed());
        println!("  Name:   {}", r.name);
        println!("  Prefix: {}", r.key_prefix);
        if let Some(key) = &r.key {
            println!(
                "\n  {} {}",
                "Key:".bold(),
                key.yellow()
            );
            println!(
                "  {}",
                "Save this key now — it won't be shown again.".dimmed()
            );
        }
    });
    Ok(())
}

pub(crate) fn cmd_api_keys_list(
    client: &AdminClient,
    org_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.api_keys_list(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{} ({} total)\n", "API Keys".bold(), r.keys.len());
        for k in &r.keys {
            println!(
                "  {} {} [{}]",
                k.id.dimmed(),
                k.name.bold(),
                k.key_prefix.dimmed()
            );
            if let Some(scopes) = &k.scopes {
                println!("    Scopes: {}", scopes.join(", "));
            }
            if let Some(limit) = k.spend_limit_cents {
                println!("    Spend limit: ${:.2}", limit as f64 / 100.0);
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_api_keys_revoke(
    client: &AdminClient,
    org_id: &str,
    key_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Revoke API key {}?", key_id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.api_keys_revoke(org_id, key_id)?;
    output::print_result(mode, &resp, |_| {
        println!("{} API key revoked", "OK".green().bold());
    });
    Ok(())
}

pub(crate) fn cmd_api_keys_usage(
    client: &AdminClient,
    org_id: &str,
    days: u32,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.api_keys_usage(org_id, days)?;
    output::print_result(mode, &resp, |r| {
        println!("{} (last {} days)\n", "AI Usage".bold(), days);
        println!("  Total Requests: {}", r.total_requests);
        println!("  Total Tokens:   {}", r.total_tokens);
        println!(
            "  Total Cost:     ${:.2}",
            r.total_cost_cents as f64 / 100.0
        );
    });
    Ok(())
}
