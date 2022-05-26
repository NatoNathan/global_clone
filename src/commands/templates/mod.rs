use log::trace;

mod add;
mod list;
mod remove;

#[derive(Debug, clap::Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct CliArgs {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {

    /// Add a new template to the list of templates
    #[clap(alias = "a")]
    Add(add::CliArgs),
    /// Remove a template from the list of templates
    #[clap(alias = "r")]
    Remove(remove::CliArgs),

    /// List all templates
    #[clap(alias = "ls")]
    List,
}

pub fn command(
    args: CliArgs,
    config: crate::config::AppConfig,
    _dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = args.command.unwrap_or(Commands::List);
    trace!("Templates");
    match cmd {
        Commands::List => list::command(config),
        Commands::Add(add_args) => add::command(add_args, config),
        Commands::Remove(remove_args) => remove::command(remove_args, config),
    }
}
