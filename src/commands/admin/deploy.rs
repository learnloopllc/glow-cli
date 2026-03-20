use anyhow::Result;
use crate::admin::AdminClient;
use crate::output::{self, OutputMode};

pub(crate) fn cmd_deploy_create(
    client: &AdminClient,
    license_id: &str,
    name: &str,
    subdomain: &str,
    base_domain: Option<&str>,
    private: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy(license_id, name, subdomain, base_domain, private)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} Deployment created: {}",
            "OK".green().bold(),
            r.deployment.name.as_deref().unwrap_or("").bold()
        );
        super::print_deployment(&r.deployment);
        if let Some(repo) = &r.repo {
            if let Some(url) = repo.get("html_url").and_then(|v| v.as_str()) {
                println!("    Repo: {}", url);
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_deploy_stop(
    client: &AdminClient,
    deployment_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Stop deployment {}?", deployment_id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.deploy_stop(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} Deployment stopped: {}",
            "OK".green().bold(),
            r.name.as_deref().unwrap_or(deployment_id).dimmed()
        );
    });
    Ok(())
}

pub(crate) fn cmd_deploy_destroy(
    client: &AdminClient,
    deployment_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(
        &format!("Destroy deployment {}? This cannot be undone.", deployment_id),
        yes,
    ) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.deploy_destroy(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} Deployment destroyed: {}",
            "OK".green().bold(),
            r.name.as_deref().unwrap_or(deployment_id).dimmed()
        );
    });
    Ok(())
}

pub(crate) fn cmd_deploy_list(
    client: &AdminClient,
    active_only: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_list(active_only)?;
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
