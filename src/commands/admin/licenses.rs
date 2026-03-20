use anyhow::Result;
use crate::admin::AdminClient;
use crate::output::{self, OutputMode};

pub(crate) fn cmd_license_list(
    client: &AdminClient,
    active_only: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.license_list(active_only)?;
    output::print_result(mode, &resp, |r| {
        println!("{} ({} total)\n", "Licenses".bold(), r.licenses.len());
        for lic in &r.licenses {
            let status = match lic.active {
                Some(true) => "active".green().to_string(),
                Some(false) => "inactive".red().to_string(),
                None => "unknown".dimmed().to_string(),
            };
            println!(
                "  {} [{}]",
                lic.id.dimmed(),
                status
            );
            if let Some(key) = &lic.key {
                println!("    Key:    {}", key);
            }
            if let Some(exp) = &lic.expiry {
                println!("    Expiry: {}", exp);
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_license_validate(client: &AdminClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.validate_license()?;
    output::print_result(mode, &resp, |r| {
        if r.valid {
            println!("{} License is valid", "OK".green().bold());
            if let Some(lic) = &r.license {
                println!("  License ID: {}", lic.id.dimmed());
            }
        } else {
            println!("{} License is invalid", "FAIL".red().bold());
            if let Some(msg) = &r.message {
                println!("  {}", msg);
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_license_create(
    client: &AdminClient,
    key: &str,
    expiry: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.license_create(key, expiry)?;
    output::print_result(mode, &resp, |r| {
        println!("{} License created: {}", "OK".green().bold(), r.id.dimmed());
    });
    Ok(())
}

pub(crate) fn cmd_license_update(
    client: &AdminClient,
    id: &str,
    key: Option<&str>,
    expiry: Option<&str>,
    active: Option<bool>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.license_update(id, key, expiry, active)?;
    output::print_result(mode, &resp, |r| {
        println!("{} License updated: {}", "OK".green().bold(), r.id.dimmed());
    });
    Ok(())
}

pub(crate) fn cmd_license_delete(
    client: &AdminClient,
    id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Delete license {}?", id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.license_delete(id)?;
    output::print_result(mode, &resp, |_| {
        println!("{} Deleted license {}", "OK".green().bold(), id.dimmed());
    });
    Ok(())
}
