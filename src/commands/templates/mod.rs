#[cfg(feature = "cli")]
use super::CliCommand;

mod add;
mod list;
mod remove;

#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct TemplatesCommand {
    #[clap(subcommand)]
    command: Option<TemplatesCommands>,
}

#[cfg(feature = "cli")]
#[derive(Debug, clap::Subcommand)]
enum TemplatesCommands {
    /// Add a new template to the list of templates
    #[clap(alias = "a")]
    Add(add::AddCommand),
    /// Remove a template from the list of templates
    #[clap(alias = "r")]
    Remove(remove::RemoveCommand),

    /// List all templates
    #[clap(alias = "ls")]
    List(list::ListCommand),
}


#[cfg(feature = "cli")]
impl CliCommand for TemplatesCommand {
   fn command(
        self,
        config: crate::config::AppConfig,
        dry_run: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = self.command.unwrap_or(TemplatesCommands::List(list::ListCommand {}));

        crate::trace!("logging");

        match cmd {
            TemplatesCommands::List(a) => a.command(config, dry_run),
            TemplatesCommands::Add(a) => a.command(config, dry_run),
            TemplatesCommands::Remove(a) => a.command(config, dry_run),
        }
    }
}
