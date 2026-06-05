use clap::{Parser,Subcommand};
use serde::Deserialize;
use std::{collections::HashMap, mem::forget};




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
    base_code: String,
    rates: HashMap<String,f64>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();
    match cli.subcommand {
        Commands::Convert { amount, from, to } => {
            let u_to = to.to_uppercase();
            let u_from = from.to_uppercase();
            let url = format!("https://open.er-api.com/v6/latest/{}",u_from);
            let client = reqwest::Client::new();
            let response = client.get(&url)
            .send()
            .await?;

        let data = response.json::<ExchangeResponse>().await?;
        let new_rates = data.rates.get(&u_to);

        if let Some(rate) = new_rates {
            let converted_amount = amount * rate;
            println!("🎉 Conversion Successful!");
            println!("{:.2} {} = {:.2} {}", amount, u_from, converted_amount, u_to);
        } else {
            println!("❌ Error: Could not find currency code '{}'", u_to);
        }




        }
    }
    Ok(())
    
}