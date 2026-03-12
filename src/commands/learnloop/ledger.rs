use anyhow::Result;
use crate::ledger;
use crate::learnloop::LearnLoopClient;
use crate::output::{self, OutputMode};

pub(crate) fn cmd_ledger_verify(path: &str, license_key: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let chain = ledger::read_chain(std::path::Path::new(path))?;
    let status = ledger::verify_chain(&chain, license_key);

    output::print_result(mode, &status, |s| {
        if s.valid {
            println!(
                "{} Chain verified — {} entries, all valid",
                "OK".green().bold(),
                s.total_entries
            );
        } else {
            println!(
                "{} Chain verification failed — {} errors in {} entries",
                "FAIL".red().bold(),
                s.errors.len(),
                s.total_entries
            );
            for err in &s.errors {
                println!("  - {}", err);
            }
        }
    });

    if !status.valid {
        anyhow::bail!("Ledger verification failed");
    }
    Ok(())
}

pub(crate) fn cmd_ledger_status(path: &str, license_key: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct StatusReport {
        path: String,
        total_entries: usize,
        chain_valid: bool,
        latest_hash: String,
        latest_attempt: Option<String>,
    }

    let chain = ledger::read_chain(std::path::Path::new(path))?;
    let verification = ledger::verify_chain(&chain, license_key);
    let latest_attempt = chain.last().map(|e| e.attempt.attempt_id.clone());

    let report = StatusReport {
        path: path.to_string(),
        total_entries: verification.total_entries,
        chain_valid: verification.valid,
        latest_hash: verification.latest_hash.clone(),
        latest_attempt,
    };

    output::print_result(mode, &report, |r| {
        println!("{}", "Ledger Status".bold());
        println!("  Path:           {}", r.path);
        println!("  Total Entries:  {}", r.total_entries);
        println!(
            "  Chain Valid:    {}",
            if r.chain_valid {
                "yes".green().to_string()
            } else {
                "NO".red().to_string()
            }
        );
        if let Some(id) = &r.latest_attempt {
            println!("  Latest Attempt: {}", id.dimmed());
        }
        println!("  Latest Hash:    {}", r.latest_hash.dimmed());
    });
    Ok(())
}

pub(crate) fn cmd_ledger_sync(
    path: &str,
    license_key: &str,
    ll_client: &LearnLoopClient,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;
    use serde::Serialize;

    #[derive(Serialize)]
    struct SyncResult {
        total_local: usize,
        synced: usize,
        license_id: String,
    }

    let chain = ledger::read_chain(std::path::Path::new(path))?;
    let verification = ledger::verify_chain(&chain, license_key);
    if !verification.valid {
        anyhow::bail!(
            "Local chain is invalid — fix integrity issues before syncing. Run 'learnloop ledger verify --path {}'",
            path
        );
    }

    let license_resp = ll_client.validate_license()?;
    if !license_resp.valid {
        anyhow::bail!("License key is not valid. Check your LEARNLOOP_LICENSE_KEY.");
    }
    let license_id = license_resp
        .license
        .ok_or_else(|| anyhow::anyhow!("License validation returned no license info."))?
        .id;

    let mut synced = 0;
    for entry in &chain {
        ll_client.usage_report(&license_id, &entry.chain_hash, 1)?;
        synced += 1;
    }

    let result = SyncResult {
        total_local: chain.len(),
        synced,
        license_id: license_id.clone(),
    };

    output::print_result(mode, &result, |r| {
        println!(
            "{} Synced {} entries to LearnLoop API (license: {})",
            "OK".green().bold(),
            r.synced,
            r.license_id.dimmed()
        );
    });
    Ok(())
}
