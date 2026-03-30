use crate::admin::AdminClient;
use crate::output::{self, OutputMode};
use anyhow::Result;

pub(crate) fn cmd_usage(client: &AdminClient, org_id: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.usage_summary(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{}", "Usage Summary".bold());
        if let Some(plan) = &r.plan {
            println!("  Plan:              {}", plan.bold());
        }
        println!("  Total Simulations: {}", r.total_simulations);
        if let Some(free) = r.free_tier {
            println!("  Free Tier:         {}", free);
        }
        if let Some(remaining) = r.free_remaining {
            println!(
                "  Free Remaining:    {}",
                if remaining > 0 {
                    remaining.to_string().green().to_string()
                } else {
                    "0".red().to_string()
                }
            );
        }
        if let Some(daily_limit) = r.daily_limit {
            println!("  Daily Limit:       {}", daily_limit);
        }
        if let Some(overage) = r.overage_simulations {
            if overage > 0 {
                println!("  Overage:           {}", overage.to_string().yellow());
            }
        }
        if let Some(cost) = &r.estimated_cost {
            println!("  Estimated Cost:    {}", cost);
        }
    });
    Ok(())
}

pub(crate) fn cmd_usage_daily(
    client: &AdminClient,
    org_id: &str,
    from: Option<&str>,
    to: Option<&str>,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.usage_daily(org_id, from, to)?;
    output::print_result(mode, &resp, |r| {
        println!(
            "{} ({} total simulations)\n",
            "Daily Usage".bold(),
            r.total_simulations
        );
        for entry in &r.daily {
            let date = match &entry.usage_date {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            println!("  {} — {} sims", date.bold(), entry.simulation_count);
        }
    });
    Ok(())
}
