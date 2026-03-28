use crate::admin::AdminClient;
use crate::output::{self, OutputMode};
use anyhow::Result;

pub(crate) fn cmd_billing_plans(client: &AdminClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.billing_plans()?;
    output::print_result(mode, &resp, |r| {
        println!("{}\n", "Available Plans".bold());
        for plan in &r.plans {
            println!("  {} — {}", plan.id.bold(), plan.name);
            if let Some(price) = &plan.pricing {
                println!("    Price:    {}", price);
            }
            if let Some(included) = plan.included {
                println!("    Included: {} simulations", included);
            }
            if let Some(overage) = &plan.overage_unit_price {
                println!("    Overage:  {}/simulation", overage);
            }
            println!();
        }
    });
    Ok(())
}

pub(crate) fn cmd_billing_status(
    client: &AdminClient,
    org_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.billing_status(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{}", "Billing Status".bold());
        println!(
            "  Subscribed: {}",
            if r.subscribed {
                "yes".green().to_string()
            } else {
                "no".red().to_string()
            }
        );
        if let Some(status) = &r.status {
            println!("  Status:     {}", status);
        }
    });
    Ok(())
}

pub(crate) fn cmd_billing_checkout(
    client: &AdminClient,
    org_id: &str,
    plan: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.billing_checkout(org_id, plan)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Checkout session created", "OK".green().bold());
        println!("  Open this URL to complete payment:");
        println!("  {}", r.url);
    });

    if let Err(e) = open::that(&resp.url) {
        eprintln!("Could not open browser: {}", e);
    }
    Ok(())
}

pub(crate) fn cmd_billing_portal(
    client: &AdminClient,
    org_id: &str,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.billing_portal(org_id)?;
    output::print_result(mode, &resp, |r| {
        println!("{} Billing portal:", "OK".green().bold());
        println!("  {}", r.url);
    });

    if let Err(e) = open::that(&resp.url) {
        eprintln!("Could not open browser: {}", e);
    }
    Ok(())
}

pub(crate) fn cmd_billing_invoices(
    client: &AdminClient,
    org_id: &str,
    limit: u32,
    mode: OutputMode,
) -> Result<()> {
    use colored::Colorize;

    let resp = client.billing_invoices(org_id, limit)?;
    output::print_result(mode, &resp, |r| {
        println!("{} ({} invoices)\n", "Invoices".bold(), r.invoices.len());
        for inv in &r.invoices {
            let status = match inv.status.as_deref() {
                Some("paid") => "paid".green().to_string(),
                Some("open") => "open".yellow().to_string(),
                Some("draft") => "draft".dimmed().to_string(),
                Some(s) => s.to_string(),
                None => "unknown".dimmed().to_string(),
            };
            let number = inv.number.as_deref().unwrap_or(&inv.id);
            let amount = format!("${:.2}", inv.amount_due as f64 / 100.0);
            println!("  {} {} [{}]", number.bold(), amount, status);
            if let Some(start) = &inv.period_start {
                let end = inv.period_end.as_deref().unwrap_or("?");
                println!("    Period: {} — {}", start.dimmed(), end.dimmed());
            }
            if let Some(url) = &inv.hosted_invoice_url {
                println!("    {}", url.dimmed());
            }
        }
    });
    Ok(())
}

pub(crate) fn cmd_billing_pricing(client: &AdminClient, mode: OutputMode) -> Result<()> {
    use colored::Colorize;

    let resp = client.billing_pricing()?;
    output::print_result(mode, &resp, |r| {
        println!("{}\n", "Pricing".bold());
        for tier in &r.tiers {
            let highlight = if tier.highlighted == Some(true) {
                " *".yellow().to_string()
            } else {
                String::new()
            };
            println!("  {} — {}{}", tier.name.bold(), tier.price, highlight);
            if let Some(unit) = &tier.unit {
                print!("    per {}", unit);
                if let Some(period) = &tier.billing_period {
                    print!(" ({})", period);
                }
                println!();
            }
            println!("    {}", tier.description.dimmed());
            println!();
        }

        if !r.features.is_empty() {
            println!("{}", "Features:".bold());
            for feat in &r.features {
                println!("  {}", feat.name);
                for (tier_id, val) in &feat.values {
                    println!("    {}: {}", tier_id.dimmed(), val);
                }
            }
        }
    });
    Ok(())
}
