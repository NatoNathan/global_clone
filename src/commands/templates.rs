use clap::Args;
use clap::{Subcommand};
use log::{trace};

use prettytable::{Table};

use dialoguer::Input;
use dialoguer::Confirm;
use dialoguer::{
  Select,
  theme::ColorfulTheme
};
use dialoguer::console::Term;
use crate::config::{AppConfig, self};

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct CliArgs {
  #[clap(subcommand)]
  command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new template
    Add(AddArgs),
    /// Remove a template
    Remove(RemoveArgs),
    /// List templates
    List
}

#[derive(Debug, Args)]
struct AddArgs {
  /// Name of the template
  #[clap(short, long, group ="temp")]
  name:Option<String>,

  /// the template string
  #[clap(short, long, requires ="temp")]
  template:Option<String>,
}

#[derive(Debug, Args)]
struct RemoveArgs {
  /// Name of the template
  name:Option<String>,
}

pub fn command(args: CliArgs, config:AppConfig) {
  let cmd = args.command.unwrap_or(Commands::List);
  match cmd {
    Commands::List => list(config),
    Commands::Add(add_args)=> add(add_args, config),
    Commands::Remove(remove_args)=> remove(remove_args, config),
  }
}

fn list(config:AppConfig ) {
  trace!("Templates:List");
  let default_template = &config.default_template.to_string();
  let mut table = Table::new();
  table.add_row(row!["Name", "Template"]);
  for (key,val) in config.templates {
    if key == *default_template {
      table.add_row(row![format!("{}*", key), val]);
    } else {
      table.add_row(row![key, val]);
    }
  }
  table.printstd();
}

fn add(args:AddArgs,mut config:AppConfig ) {
  trace!("Templates:Add");
  let mut name: String = args.name.unwrap_or_default();
  let mut template:String = args.template.unwrap_or_default();
  if name == *"" {
    trace!("No name or template provided, asking user instead");
    name = Input::<String>::new()
      .with_prompt("Enter the Template Name")
      .interact_text()
      .unwrap();
    template = Input::<String>::new()
      .with_prompt("Enter Template")
      .with_initial_text("~/git/{provider}/{owner}/{repo}")
      .interact_text()
      .unwrap();
  }

  trace!("name:{}, template:{} provided", &name, &template);

  if Confirm::new()
      .with_prompt(format!("You are about to add a new Template: name:{}, template:{}", &name, &template))
      .interact()
      .unwrap_or(false) {
        trace!("Added new Template");
        config.templates.insert(name, template);
        config::save_config(config);
  } else {
      trace!("Not adding new template");
  }
}

fn remove(args: RemoveArgs, mut config: AppConfig) {
  trace!("Templates:Remove");
  let mut name: String = args.name.unwrap_or_default();
  if name == *"" {
    trace!("name not provided asking user to pick instead");
    let options: Vec<String> = config.templates.clone().into_keys().collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("Pick the template you wish to remove")
        .items(&options)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap().unwrap_or(0);
    
        name = options[selection].to_string();
  }

  trace!("name: {} provided", &name);

  if Confirm::new()
      .with_prompt(format!("You are about to remove Template: name:{}", &name))
      .interact()
      .unwrap_or(false) {
        trace!("Removed Template");
        config.templates.remove(&name);
        config::save_config(config);
  } else {
      trace!("Not removing template");
  }
}