use anyhow::{Context, Result};
use colored::Colorize;
use crate::glow::GlowClient;
use crate::output::{self, OutputMode};

pub(crate) fn cmd_upload_create(
    client: &GlowClient,
    filename: &str,
    content_type: Option<&str>,
    size: Option<u64>,
    mode: OutputMode,
) -> Result<()> {
    let response = client.upload_create(filename, content_type, size)?;
    output::print_result(mode, &response, |resp| {
        println!("{} Upload created", "OK".green().bold());
        println!("  Upload ID:  {}", resp.upload_id);
        println!("  Upload URL: {}", resp.upload_url.dimmed());
    });
    Ok(())
}

pub(crate) fn cmd_upload_send(
    client: &GlowClient,
    upload_id: &str,
    file_path: &str,
    mode: OutputMode,
) -> Result<()> {
    let data = std::fs::read(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;
    let total = data.len() as u64;

    if mode == OutputMode::Human {
        eprintln!(
            "{} Uploading {} ({} bytes)...",
            "▸".dimmed(),
            file_path,
            total
        );
    }

    let response = client.upload_patch(upload_id, data, 0)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{} Uploaded {} bytes (offset: {})",
            "OK".green().bold(),
            total,
            resp.offset
        );
    });
    Ok(())
}

pub(crate) fn cmd_upload_status(
    client: &GlowClient,
    upload_id: &str,
    mode: OutputMode,
) -> Result<()> {
    let response = client.upload_status(upload_id)?;
    output::print_result(mode, &response, |resp| {
        println!("{}", "Upload Status".bold());
        println!("  Offset: {}", resp.offset);
        if let Some(len) = resp.length {
            let pct = if len > 0 {
                (resp.offset as f64 / len as f64 * 100.0) as u64
            } else {
                0
            };
            println!("  Length: {}", len);
            println!("  Progress: {}%", pct);
        }
    });
    Ok(())
}

pub(crate) fn cmd_upload_download(
    client: &GlowClient,
    upload_id: &str,
    output_path: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let data = client.upload_download(upload_id)?;

    match output_path {
        Some(path) => {
            std::fs::write(path, &data)
                .with_context(|| format!("Failed to write to {}", path))?;
            if mode == OutputMode::Human {
                println!(
                    "{} Downloaded {} bytes to {}",
                    "OK".green().bold(),
                    data.len(),
                    path
                );
            }
        }
        None => {
            // Write raw to stdout
            use std::io::Write;
            std::io::stdout()
                .write_all(&data)
                .context("Failed to write to stdout")?;
        }
    }
    Ok(())
}

pub(crate) fn cmd_upload_multipart(
    client: &GlowClient,
    file_path: &str,
    mode: OutputMode,
) -> Result<()> {
    if mode == OutputMode::Human {
        eprintln!("{} Uploading {}...", "▸".dimmed(), file_path);
    }

    let response = client.upload_multipart(file_path)?;
    output::print_result(mode, &response, |resp| {
        println!("{} File uploaded", "OK".green().bold());
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}

pub(crate) fn cmd_upload_raw(
    client: &GlowClient,
    file_path: &str,
    content_type: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    if mode == OutputMode::Human {
        eprintln!("{} Uploading {}...", "▸".dimmed(), file_path);
    }

    let response = client.upload_raw(file_path, content_type)?;
    output::print_result(mode, &response, |resp| {
        println!("{} File uploaded", "OK".green().bold());
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}

pub(crate) fn cmd_upload_csv_parse(
    client: &GlowClient,
    file_path: &str,
    mode: OutputMode,
) -> Result<()> {
    if mode == OutputMode::Human {
        eprintln!("{} Parsing CSV {}...", "▸".dimmed(), file_path);
    }

    let response = client.upload_csv_parse(file_path)?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}

pub(crate) fn cmd_upload_preview(client: &GlowClient, mode: OutputMode) -> Result<()> {
    let response = client.upload_preview()?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}

pub(crate) fn cmd_upload_template(
    client: &GlowClient,
    output_path: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let data = client.upload_template()?;
    let path = output_path.unwrap_or("template.csv");

    std::fs::write(path, &data)
        .with_context(|| format!("Failed to write template to {}", path))?;

    if mode == OutputMode::Human {
        println!(
            "{} Template downloaded to {} ({} bytes)",
            "OK".green().bold(),
            path,
            data.len()
        );
    }
    Ok(())
}

pub(crate) fn cmd_upload_discover(client: &GlowClient, mode: OutputMode) -> Result<()> {
    let response = client.upload_discover()?;
    output::print_result(mode, &response, |resp| {
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}

pub(crate) fn cmd_upload_finalize(
    client: &GlowClient,
    body: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    let parsed = match body {
        Some(s) => serde_json::from_str(s)
            .map_err(|e| anyhow::anyhow!("Invalid JSON body: {}", e))?,
        None => serde_json::json!({}),
    };

    let response = client.upload_finalize(parsed)?;
    output::print_result(mode, &response, |resp| {
        println!("{} Upload finalized", "OK".green().bold());
        println!(
            "{}",
            serde_json::to_string_pretty(resp).unwrap()
        );
    });
    Ok(())
}
