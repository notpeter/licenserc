use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

mod config;

const TEMPLATE: &str = include_str!("../assets/template.licenserc.yaml");

const CONFIG_CANDIDATES: &[&str] = &[
    ".licenserc.yaml",
    ".licenserc.yml",
    "licenserc.yaml",
    "licenserc.yml",
];

#[derive(Parser)]
#[command(name = "licenserc", about = "License header management tool")]
struct Cli {
    /// Config filename [default: .licenserc.yaml]
    #[arg(short, long, global = true)]
    config: Option<String>,

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

/// Find the first existing config file from the candidate list.
fn find_config() -> Option<&'static str> {
    CONFIG_CANDIDATES
        .iter()
        .find(|c| Path::new(c).exists())
        .copied()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Header => todo!("header"),
        Command::Dependency => todo!("dependency"),
        Command::Init => {
            if let Some(existing) = cli.config.as_deref().filter(|c| Path::new(c).exists()).or(find_config()) {
                eprintln!("Error: {existing} already exists");
                std::process::exit(1);
            }
            let path = cli.config.as_deref().unwrap_or(CONFIG_CANDIDATES[0]);
            let path = Path::new(path);
            std::fs::write(path, TEMPLATE).expect("failed to write config file");
            println!("Created {}", path.display());
        }
        Command::Schema { output } => {
            let schema = config::generate_schema();
            let json = serde_json::to_string_pretty(&schema).expect("failed to serialize schema");
            std::fs::write(&output, format!("{json}\n")).expect("failed to write schema file");
            println!("Schema written to {}", output.display());
        }
    }
}
