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

        // Outcome-based pricing details
        if let Some(meta) = &r.meta {
            println!("\n{}", "  Outcome-Based Pricing".bold());
            if let Some(outcomes) = meta.total_outcomes {
                println!("    Outcomes (passed): {}", outcomes);
            }
            if let Some(completed) = meta.attempts_completed {
                println!("    Attempts Completed: {}", completed);
            }
            if let Some(pct) = meta.discount_pct {
                let color = if pct > 0 {
                    format!("{}%", pct).green().to_string()
                } else {
                    format!("{}%", pct).dimmed().to_string()
                };
                println!("    Performance Discount: {}", color);
            }
            if let Some(gross) = &meta.gross {
                println!("    Gross:    {}", gross);
            }
            if let Some(discount) = &meta.discount {
                println!("    Discount: {}", discount.green());
            }
            if let Some(net) = &meta.net {
                println!("    Net:      {}", net.bold());
            }
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
            print!("  {} — {} sims", date.bold(), entry.simulation_count);
            if let Some(started) = entry.attempts_started {
                print!(", {} started", started);
            }
            if let Some(completed) = entry.attempts_completed {
                print!(", {} completed", completed);
            }
            if let Some(outcomes) = entry.outcomes {
                print!(", {} passed", outcomes.to_string().green());
            }
            println!();
        }
    });
    Ok(())
}
