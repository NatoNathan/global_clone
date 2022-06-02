use log::{trace};

#[derive(Debug, clap::Args)]
pub struct CliArgs {
  /// Name of the template
  #[clap(short, long, group ="temp")]
  name:Option<String>,

  /// the template string
  #[clap(short, long, requires ="temp")]
  template:Option<String>,
}

pub fn command(args:CliArgs, mut config:crate::config::AppConfig ) -> Result<(), Box<dyn std::error::Error>> {
  trace!("Templates:Add");
  let mut name: String = args.name.unwrap_or_default();
  let mut template:String = args.template.unwrap_or_default();
  if name == *"" {
    trace!("No name or template provided, asking user instead");
    name = dialoguer::Input::<String>::new()
      .with_prompt("Enter the Template Name")
      .interact_text()
      .unwrap();
    template = dialoguer::Input::<String>::new()
      .with_prompt("Enter Template")
      .with_initial_text("~/git/{provider}/{owner}/{repo}")
      .interact_text()
      .unwrap();
  }

  trace!("name:{}, template:{} provided", &name, &template);

  if dialoguer::Confirm::new()
      .with_prompt(format!("You are about to add a new Template: name:{}, template:{}", &name, &template))
      .interact()
      .unwrap_or(false) {
        trace!("Added new Template");
        config.templates.insert(name, template);
        crate::config::AppConfig::save_config(config);
  } else {
      trace!("Not adding new template");
  }
  Ok(())
}