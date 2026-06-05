use clap::{Parser,Subcommand};
use serde::Deserialize;
use std::{collections::HashMap, fs::OpenOptions};
use indicatif::ProgressBar;
use std::io::Write;





#[derive(Parser)]
#[command(name = "curr", version = "1.0", about = "Universal Currency Converter")]
struct CLI {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Subcommand,Debug)]
enum Commands {
    Convert {
        amount: f64,
        from: String,
        to: String,
    }
}

#[derive(Deserialize)]

struct ExchangeResponse {
    
    rates: HashMap<String,f64>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();
    match cli.subcommand {
        
        Commands::Convert { amount, from, to } => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_message("Fetching live exchange rates...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));

            let u_to = to.to_uppercase();
            let u_from = from.to_uppercase();
            let url = format!("https://open.er-api.com/v6/latest/{}",u_from);
            let client = reqwest::Client::new();
            let response = client.get(&url)
            .send()
            .await?;
            spinner.finish_and_clear();
        let data = response.json::<ExchangeResponse>().await?;
        let new_rates = data.rates.get(&u_to);

        if let Some(rate) = new_rates {
            let converted_amount = amount * rate;
            println!("🎉 Conversion Successful!");
            println!("{:.2} {} = {:.2} {}", amount, u_from, converted_amount, u_to);

            let log_line = format!("{:.2} {} = {:.2} {}\n", amount, u_from, converted_amount, u_to);

            let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("conversion_history.txt")?;

            file.write_all(log_line.as_bytes())?;
            println!("💾 Conversion recorded to conversion_history.txt");

        } else {
            println!("❌ Error: Could not find currency code '{}'", u_to);
        }





        }
    }
    Ok(())
    
}