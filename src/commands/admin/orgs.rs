use anyhow::Result;
use crate::admin::AdminClient;
use crate::output::{self, OutputMode};

pub(crate) fn cmd_org_list(client: &AdminClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_list()?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} ({} total)\n",
            "Organizations".bold(),
            r.organizations.len()
        );
        for org in &r.organizations {
            let status = match org.active {
                Some(true) => "active".green().to_string(),
                Some(false) => "inactive".red().to_string(),
                None => "".to_string(),
            };
            println!("  {} {} [{}]", org.id.dimmed(), org.name.bold(), status);
            if let Some(desc) = &org.description {
                println!("    {}", desc.dimmed());
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_org_create(
    client: &AdminClient,
    name: &str,
    description: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_create(name, description)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} Created organization {} ({})",
            "OK".green().bold(),
            r.name.bold(),
            r.id.dimmed()
        );
    });
    Ok(())
}

pub(crate) fn cmd_org_get(client: &AdminClient, id: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_get(id)?;
    output::print_result(mode, &resp, |r| {
        println!("{}", r.name.bold());
        println!("  ID:     {}", r.id.dimmed());
        if let Some(desc) = &r.description {
            println!("  Desc:   {}", desc);
        }
        if let Some(active) = r.active {
            println!(
                "  Active: {}",
                if active {
                    "yes".green().to_string()
                } else {
                    "no".red().to_string()
                }
            );
        }
    });
    Ok(())
}

pub(crate) fn cmd_org_update(
    client: &AdminClient,
    id: &str,
    name: Option<&str>,
    description: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_update(id, name, description, None)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Updated organization {}", "OK".green().bold(), r.name.bold());
    });
    Ok(())
}

pub(crate) fn cmd_org_delete(
    client: &AdminClient,
    id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Delete organization {}?", id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.org_delete(id)?;
    output::print_result(mode, &resp, |_| {
        println!("{} Deleted organization {}", "OK".green().bold(), id.dimmed());
    });
    Ok(())
}

pub(crate) fn cmd_org_members(client: &AdminClient, org_id: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_members(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{} ({} members)\n", "Members".bold(), r.members.len());
        for m in &r.members {
            let role_str = m.role.as_deref().unwrap_or("member");
            let role = match role_str {
                "admin" => "admin".yellow().to_string(),
                _ => role_str.dimmed().to_string(),
            };
            println!("  {} {} <{}> [{}]", m.id.dimmed(), m.name.bold(), m.email, role);
        }
    });
    Ok(())
}

pub(crate) fn cmd_org_member_add(
    client: &AdminClient,
    org_id: &str,
    email: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_member_add(org_id, email)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Added {} to organization", "OK".green().bold(), r.email);
    });
    Ok(())
}

pub(crate) fn cmd_org_member_remove(
    client: &AdminClient,
    org_id: &str,
    user_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Remove member {} from organization?", user_id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.org_member_remove(org_id, user_id)?;
    output::print_result(mode, &resp, |_| {
        println!(
            "{} Removed {} from organization",
            "OK".green().bold(),
            user_id.dimmed()
        );
    });
    Ok(())
}

pub(crate) fn cmd_org_license(client: &AdminClient, org_id: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_license(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{}", "Organization License".bold());
        println!("  Plan: {}", r.plan);
        match &r.license {
            Some(lic) => {
                println!("  License ID: {}", lic.id.dimmed());
                if let Some(exp) = &lic.expiry {
                    println!("  Expiry:     {}", exp);
                }
            }
            None => println!("  License:    {}", "none assigned".dimmed()),
        }
    });
    Ok(())
}

pub(crate) fn cmd_org_license_set(
    client: &AdminClient,
    org_id: &str,
    license_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_license_set(org_id, license_id)?;
    output::print_result(mode, &resp, |_| {
        println!(
            "{} Assigned license {} to organization",
            "OK".green().bold(),
            license_id.dimmed()
        );
    });
    Ok(())
}

pub(crate) fn cmd_org_license_remove(
    client: &AdminClient,
    org_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm("Remove license from organization?", yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.org_license_remove(org_id)?;
    output::print_result(mode, &resp, |_| {
        println!(
            "{} Removed license from organization",
            "OK".green().bold()
        );
    });
    Ok(())
}

pub(crate) fn cmd_org_deployments(
    client: &AdminClient,
    org_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.org_deployments(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} ({} total)\n",
            "Deployments".bold(),
            r.deployments.len()
        );
        for d in &r.deployments {
            super::print_deployment(d);
        }
    });
    Ok(())
}
