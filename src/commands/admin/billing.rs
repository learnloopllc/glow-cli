use anyhow::Result;
use crate::admin::AdminClient;
use crate::output::{self, OutputMode};

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
