use crate::admin::AdminClient;
use crate::output::{self, OutputMode};
use anyhow::Result;

#[allow(clippy::too_many_arguments)]
pub(crate) fn cmd_deploy_update(
    client: &AdminClient,
    deployment_id: &str,
    subdomain: Option<String>,
    base_domain: Option<String>,
    origin: Option<String>,
    client_origins: Option<String>,
    app_prefix: Option<String>,
    hosting_type: Option<String>,
    airgapped: Option<bool>,
    api_key_id: Option<String>,
    oauth_client_id: Option<String>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let mut body = serde_json::json!({});
    let obj = body.as_object_mut().unwrap();
    if let Some(v) = subdomain {
        obj.insert("subdomain".into(), v.into());
    }
    if let Some(v) = base_domain {
        obj.insert("base_domain".into(), v.into());
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
    if let Some(v) = hosting_type {
        obj.insert("hosting_type".into(), v.into());
    }
    if let Some(v) = airgapped {
        obj.insert("airgapped".into(), v.into());
    }
    if let Some(v) = api_key_id {
        obj.insert("api_key_id".into(), v.into());
    }
    if let Some(v) = oauth_client_id {
        obj.insert("oauth_client_id".into(), v.into());
    }

    if obj.is_empty() {
        anyhow::bail!("No fields to update. Pass at least one --flag.");
    }

    let resp = client.deploy_update(deployment_id, body)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} Updated deployment {}",
            "OK".green().bold(),
            r.name.as_deref().unwrap_or(deployment_id).bold()
        );
        super::print_deployment(r);
    });
    Ok(())
}

pub(crate) fn cmd_deploy_create_raw(
    client: &AdminClient,
    body: serde_json::Value,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_raw(body)?;
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

pub(crate) fn cmd_deploy_status(
    client: &AdminClient,
    deployment_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_status(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        super::print_deployment(&r.deployment);
        if let Some(wf) = &r.workflow_run {
            let status = wf.status.as_deref().unwrap_or("unknown");
            let conclusion = wf.conclusion.as_deref().unwrap_or("-");
            println!("    Workflow: {} ({})", status.bold(), conclusion.dimmed());
            if let Some(url) = &wf.html_url {
                println!("    {}", url.dimmed());
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_deploy_logs(
    client: &AdminClient,
    deployment_id: &str,
    service: &str,
    lines: u32,
    mode: OutputMode,
) -> Result<()> {
    let resp = client.deploy_logs(deployment_id, service, lines)?;
    output::print_result(mode, &resp, |r| {
        print!("{}", r.logs);
    });
    Ok(())
}

pub(crate) fn cmd_deploy_health(
    client: &AdminClient,
    deployment_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_health(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{}\n", "Container Health".bold());
        for c in &r.containers {
            let health = match c.health.as_deref() {
                Some("healthy") => "healthy".green().to_string(),
                Some("unhealthy") => "unhealthy".red().to_string(),
                Some(h) => h.dimmed().to_string(),
                None => "unknown".dimmed().to_string(),
            };
            println!("  {} [{}] {}", c.service.bold(), c.status, health);
        }
    });
    Ok(())
}

pub(crate) fn cmd_deploy_metrics(
    client: &AdminClient,
    deployment_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_metrics(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{}\n", "Container Metrics".bold());
        for c in &r.containers {
            println!("  {}", c.service.bold());
            if let Some(cpu) = &c.cpu_percent {
                println!("    CPU:    {}", cpu);
            }
            if let Some(mem) = &c.memory_usage {
                let limit = c.memory_limit.as_deref().unwrap_or("?");
                let pct = c.memory_percent.as_deref().unwrap_or("?");
                println!("    Memory: {} / {} ({})", mem, limit, pct);
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_deploy_events(
    client: &AdminClient,
    deployment_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_events(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{} ({} events)\n", "Events".bold(), r.events.len());
        for e in &r.events {
            let ts = e
                .created_at
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_default();
            println!(
                "  {} {} {}",
                ts.dimmed(),
                e.event_type.bold(),
                e.message.as_deref().unwrap_or("")
            );
        }
    });
    Ok(())
}

pub(crate) fn cmd_deploy_redeploy(
    client: &AdminClient,
    deployment_id: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Redeploy {}?", deployment_id), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.deploy_redeploy(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Redeploying", "OK".green().bold());
        super::print_deployment(r);
    });
    Ok(())
}

pub(crate) fn cmd_deploy_version(
    client: &AdminClient,
    deployment_id: &str,
    version: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(
        &format!("Update {} to version {}?", deployment_id, version),
        yes,
    ) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.deploy_version(deployment_id, version)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Version updated", "OK".green().bold());
        super::print_deployment(&r.deployment);
        if r.redeploying {
            println!("    Redeploying...");
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
        &format!(
            "Destroy deployment {}? This cannot be undone.",
            deployment_id
        ),
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
        println!("{} ({} total)\n", "Deployments".bold(), r.deployments.len());
        for d in &r.deployments {
            super::print_deployment(d);
        }
    });
    Ok(())
}

// ── Backups ──────────────────────────────────────────────────

pub(crate) fn cmd_deploy_backup_create(
    client: &AdminClient,
    deployment_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_backup_create(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Backup created: {}", "OK".green().bold(), r.name.bold());
        println!("    Size: {} bytes", r.size_bytes);
    });
    Ok(())
}

pub(crate) fn cmd_deploy_backup_list(
    client: &AdminClient,
    deployment_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_backup_list(deployment_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{} ({} backups)\n", "Backups".bold(), r.backups.len());
        for b in &r.backups {
            let template = if b.is_template == Some(true) {
                " (template)".dimmed().to_string()
            } else {
                String::new()
            };
            let ts = b.created_at.as_ref().and_then(|v| v.as_str()).unwrap_or("");
            println!(
                "  {} {} bytes  {}{}",
                b.name.bold(),
                b.size_bytes,
                ts.dimmed(),
                template
            );
        }
    });
    Ok(())
}

pub(crate) fn cmd_deploy_backup_restore(
    client: &AdminClient,
    deployment_id: &str,
    filename: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(
        &format!(
            "Restore backup '{}' to deployment {}? This will overwrite the current database.",
            filename, deployment_id
        ),
        yes,
    ) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.deploy_backup_restore(deployment_id, filename)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} Restoring {} to {}",
            "OK".green().bold(),
            r.backup.bold(),
            r.deployment_id.dimmed()
        );
    });
    Ok(())
}

pub(crate) fn cmd_deploy_backup_delete(
    client: &AdminClient,
    deployment_id: &str,
    filename: &str,
    yes: bool,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    if !output::confirm(&format!("Delete backup '{}'?", filename), yes) {
        println!("Aborted.");
        return Ok(());
    }

    let resp = client.deploy_backup_delete(deployment_id, filename)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Deleted backup {}", "OK".green().bold(), r.name.dimmed());
    });
    Ok(())
}

// ── Versions ─────────────────────────────────────────────────

pub(crate) fn cmd_deploy_versions(
    client: &AdminClient,
    component_type: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.deploy_versions(component_type)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} ({} versions)\n",
            format!("Available {} Versions", component_type).bold(),
            r.versions.len()
        );
        for v in &r.versions {
            let pre = if v.prerelease == Some(true) {
                " (pre-release)".yellow().to_string()
            } else {
                String::new()
            };
            let date = v.published_at.as_deref().unwrap_or("");
            println!(
                "  {} {} {}{}",
                v.name.bold(),
                v.sha[..7.min(v.sha.len())].dimmed(),
                date.dimmed(),
                pre
            );
        }
    });
    Ok(())
}
