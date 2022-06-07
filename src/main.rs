use clap::{Parser, Subcommand, Command, IntoApp};
use clap_verbosity_flag::{Verbosity, InfoLevel};

use global_clone::{
    commands::{templates, clone, CliCommand},
    config::{AppConfig, self},
};

use log::{trace, info};

/// A CLI Project to help keep Git Repos Organized
/// 
/// This is a Simple CLI Project to help keep Git Repos Organized
/// easily. The idea is to replace the `git clone` command,
/// with a simple command line interface, that clones repositories to defined locations.
#[derive(Parser, Debug)]
#[clap(author, version, about, trailing_var_arg = true)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[clap(short, long, global = true)]
    dry_run: bool,

    // Flag to skip prompting for confirmation
    #[clap(short, long, global = true)]
    yes: bool,
    
    #[clap(subcommand)]
    command: Commands,
}


#[derive(Subcommand, Debug)]
enum Commands {
    /// Templates Commands - manage templates
    ///
    /// Templates are used to generate the path to a local Git Repo
    /// based on the provided template string.
    /// Templates are stored in the config file.
    /// 
    /// The following templates keys are supported:
    /// 
    /// - `{provider}` - The Git Provider (github, bitbucket, etc)
    /// 
    /// - `{owner}` - The owner of the repo (ex: github.com/owner)
    /// 
    /// - `{repo}` - The name of the repo (ex: repo)
    #[clap(alias = "t", about)]
    Templates(templates::TemplatesCommand),
    
    /// Clone a Git Repo
    /// 
    /// Clone a Git Repo into a local directory. The directory will be determined by the template.
    /// Both ssh and https are supported.
    /// 
    /// see: `templates` command for more information.
    #[clap(alias = "c", about)]
    Clone(clone::CloneCommand),

    /// Generate Shell completion Scripts
    /// 
    /// Generate shell completion scripts for the CLI.
    #[clap(alias = "completion", about)]
    ShellCompletion(CompletionCliArgs),

    /// Project Management Commands
    /// 
    /// Project management commands.
    /// these commands are used to manage the projects.
    /// they include:
    /// - listing the projects,
    /// - adding a new project,
    /// - removing a project,
    /// - moving a project,
    #[clap(alias = "p", about)]
    Projects(projects::ProjectCommandArgs),
}

#[derive(Debug, clap::Args, PartialEq)]
struct CompletionCliArgs {
  #[clap(arg_enum )]
  shell: clap_complete::Shell,

}

fn print_completion_script<G: clap_complete::Generator>(shell: G, cmd: &mut Command) {
    clap_complete::generate(shell, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn completion(args: CompletionCliArgs, _config: AppConfig, _dry_run:bool) -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = crate::Cli::command();
  info!("Generating completion script for {}", args.shell);
  print_completion_script(args.shell, &mut cmd);

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = Cli::parse();
    pretty_env_logger::formatted_builder().filter_level(cli_args.verbose.log_level_filter()).init();

    trace!("loading config");
    let cfg: AppConfig = AppConfig::get_config();

    trace!("running command");
    
    match cli_args.command {
        Commands::Templates(a) => a.command(cfg, cli_args.dry_run),
        Commands::Clone(a) => a.command(cfg, cli_args.dry_run),
        Commands::ShellCompletion(a) => completion(a, cfg, cli_args.dry_run),
        Commands::Projects(a) => a.command(cfg, cli_args.dry_run),
    }
}
