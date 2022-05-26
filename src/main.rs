use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, InfoLevel};
#[macro_use] extern crate prettytable;

use log::{trace};
mod config;
mod commands;

use config::AppConfig;
use commands::{templates, clone};

/// A CLI Project to help keep Git Repos Organized
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Templates Commands
    Templates(templates::CliArgs),
    Clone(clone::CliArgs),
}



fn main() {
    let cli_args = Cli::parse();
    pretty_env_logger::formatted_builder().filter_level(cli_args.verbose.log_level_filter()).init();

    trace!("loading config");
    let cfg: AppConfig = config::get_config();
    
    match cli_args.command {
        Commands::Templates(a) => templates::command(a, cfg),
        Commands::Clone(a) => clone::command(a, cfg),
    }
}
