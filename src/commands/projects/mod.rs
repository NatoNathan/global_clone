//! Project management commands.
//! 
//! This Module is for the project management commands.
//! these commands are used to manage the projects.
//! they include:
//! - listing the projects,
//! - adding a new project,
//! - removing a project,
//! - moving a project,

use log::{info, trace};

use crate::config::AppConfig;

mod add_project;
mod list_projects;
mod remove_project;
mod move_project;



/// Project management commands arguments.
/// 
/// This struct is used to hold the arguments for the project management commands.
#[derive(Debug, clap::Args)]
pub struct ProjectCommandArgs {
    #[clap(subcommand)]
    command: Option<ProjectCommand>,  

}

#[derive(Debug, clap::Subcommand)]
enum ProjectCommand {
    /// List the projects.
    /// 
    /// List the projects.
    List(list_projects::ListProjects),
    // /// Add a new project.
    // /// 
    // /// Add a new project.
    // Add(ProjectAddArgs),
    // /// Remove a project.
    // /// 
    // /// Remove a project.
    // Remove(ProjectRemoveArgs),
    // /// Move a project.
    // /// 
    // /// Move a project.
    // Move(ProjectMoveArgs),
}

impl ProjectCommandArgs {

  /// Run the project command.
  pub fn command(&self, config: AppConfig, _dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let args = self;

    trace!("running project command");

    match &args.command {
        Some(ProjectCommand::List(cmd)) => cmd.command(config),
        // Some(ProjectCommand::Add(args)) => {
        //     info!("Adding project");
        //     args.command()?
        // }
        // Some(ProjectCommand::Remove(args)) => {
        //     info!("Removing project");
        //     args.command()?
        // }
        // Some(ProjectCommand::Move(args)) => {
        //     info!("Moving project");
        //     args.command()?
        // }
        None => {
            info!("No project command specified");
            Ok(())
        }
    }
        
    }
  }


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cli_args() {
    pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Trace).init();
    let args = ProjectCommandArgs {
      command: Some(ProjectCommand::List(list_projects::ListProjects::new())),
    };
    let output = args.command( AppConfig::default(), false);
    assert!(output.is_ok());
  }

}