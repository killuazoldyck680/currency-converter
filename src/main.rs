use clap::{Parser,Subcommand};




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