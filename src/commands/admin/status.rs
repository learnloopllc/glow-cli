use crate::config;
use crate::output::{self, OutputMode};
use anyhow::Result;

pub(crate) fn cmd_network(api_url: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct NetworkStatus {
        api_url: String,
        api_status: String,
    }

    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let api_status = match http.get(format!("{}/health", api_url)).send() {
        Ok(resp) if resp.status().is_success() => "connected",
        _ => "unreachable",
    };

    output::print_result(
        mode,
        &NetworkStatus {
            api_url: api_url.to_string(),
            api_status: api_status.to_string(),
        },
        |s| {
            println!("{}", "Network Status".bold());
            match s.api_status.as_str() {
                "connected" => println!("  Glow API: {}", "connected".green()),
                _ => {
                    println!(
                        "  Glow API: {} (airgapped mode available)",
                        "unreachable".red()
                    );
                    println!("    Tip: check that {} is correct", api_url.dimmed());
                }
            }
        },
    );
    Ok(())
}

pub(crate) fn cmd_status(api_url: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct StatusReport {
        config_file: String,
        config_exists: bool,
        api_url: String,
        api_status: String,
    }

    let config_path = config::Config::config_path();
    let config_exists = config_path.exists();

    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()?;

    let api_status = match http.get(format!("{}/health", api_url)).send() {
        Ok(r) if r.status().is_success() => "connected",
        _ => "unreachable",
    };

    let report = StatusReport {
        config_file: config_path.display().to_string(),
        config_exists,
        api_url: api_url.to_string(),
        api_status: api_status.to_string(),
    };

    output::print_result(mode, &report, |r| {
        println!("{}", "Glow Status".bold());
        println!(
            "  Config:      {} {}",
            r.config_file,
            if r.config_exists {
                "(found)".green().to_string()
            } else {
                "(not found)".dimmed().to_string()
            }
        );
        println!("  API URL:     {}", r.api_url);
        println!(
            "  API Status:  {}",
            if r.api_status == "connected" {
                "connected".green().to_string()
            } else {
                "unreachable".red().to_string()
            }
        );
    });
    Ok(())
}
