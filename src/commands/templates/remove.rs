use log::{trace};

#[derive(Debug, clap::Args)]
pub struct CliArgs {
  /// Name of the template
  name:Option<String>,
}

pub fn command(args: CliArgs, mut config: crate::config::AppConfig) -> Result<(), Box<dyn std::error::Error>> {
  trace!("Templates:Remove");
  let mut name: String = args.name.unwrap_or_default();
  if name == *"" {
    trace!("name not provided asking user to pick instead");
    let options: Vec<String> = config.templates.clone().into_keys().collect();
    let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
      .with_prompt("Pick the template you wish to remove")
        .items(&options)
        .default(0)
        .interact_on_opt(&dialoguer::console::Term::stderr()).unwrap().unwrap_or(0);
    
        name = options[selection].to_string();
  }

  trace!("name: {} provided", &name);

  if dialoguer::Confirm::new()
      .with_prompt(format!("You are about to remove Template: name:{}", &name))
      .interact()
      .unwrap_or(false) {
        trace!("Removed Template");
        config.templates.remove(&name);
        crate::config::save_config(config);
  } else {
      trace!("Not removing template");
  }
  Ok(())
}