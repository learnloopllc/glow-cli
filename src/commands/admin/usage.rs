use anyhow::Result;
use crate::admin::AdminClient;
use crate::output::{self, OutputMode};

pub(crate) fn cmd_usage(client: &AdminClient, license_id: &str, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.usage_summary(license_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{}", "Usage Summary".bold());
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
