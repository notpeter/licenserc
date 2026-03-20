use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;

#[derive(Parser)]
#[command(name = "licenserc", about = "License header management tool")]
struct Cli {
    /// Config filename
    #[arg(short, long, default_value = ".licenserc.yaml", global = true)]
    config: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check and fix license headers
    Header,
    /// Check license compatibility of dependencies
    Dependency,
    /// Initialize a licenserc config file
    Init,
    /// Regenerate the JSON schema file
    Schema {
        /// Output path for the schema file
        #[arg(default_value = "licenserc.schema.json")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Header => todo!("header"),
        Command::Dependency => todo!("dependency"),
        Command::Init => todo!("init"),
        Command::Schema { output } => {
            let schema = config::generate_schema();
            let json = serde_json::to_string_pretty(&schema).expect("failed to serialize schema");
            std::fs::write(&output, format!("{json}\n")).expect("failed to write schema file");
            println!("Schema written to {}", output.display());
        }
    }
}
