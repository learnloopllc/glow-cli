pub mod auth;
pub mod billing;
pub mod deploy;
pub mod licenses;
pub mod orgs;
pub mod status;
pub mod usage;

use colored::Colorize;
use crate::admin::types::Deployment;

pub(crate) fn print_deployment(d: &Deployment) {
    let name = d.name.as_deref().unwrap_or("unnamed");
    let status = match d.status.as_deref() {
        Some("running") => "running".green().to_string(),
        Some("stopped") => "stopped".red().to_string(),
        Some("pending") => "pending".yellow().to_string(),
        Some(s) => s.dimmed().to_string(),
        None => "unknown".dimmed().to_string(),
    };
    let health = match d.health.as_deref() {
        Some("healthy") => "healthy".green().to_string(),
        Some("unhealthy") => "unhealthy".red().to_string(),
        Some(h) => h.dimmed().to_string(),
        None => "".to_string(),
    };
    println!("  {} {} [{}] {}", d.id.dimmed(), name.bold(), status, health);
    if let Some(domain) = &d.domain {
        println!("    https://{}", domain);
    }
}
