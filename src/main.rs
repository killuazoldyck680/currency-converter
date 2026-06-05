use clap::{Parser,Subcommand};
use serde::Deserialize;
use std::collections::HashMap;




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
async fn main() {
    let cli = CLI::parse();
    match cli.subcommand {
        Commands::Convert { amount, from, to } => {
            println!("{amount} {from} {to}")
        }
    }
    
}