#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
pub struct RemoveCommand {
    /// Name of the template
    name: Option<String>,
}
#[cfg(feature = "cli")]
impl crate::commands::CliCommand for RemoveCommand {
    fn command(
        self,
        mut config: crate::config::AppConfig,
        _dry_run: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let args = self;
        crate::trace!("Templates:Remove");
        let mut name: String = args.name.unwrap_or_default();
        if name == *"" {
            crate::trace!("name not provided asking user to pick instead");
            let options: Vec<String> = config.templates.clone().into_keys().collect();
            let selection =
                dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Pick the template you wish to remove")
                    .items(&options)
                    .default(0)
                    .interact_on_opt(&dialoguer::console::Term::stderr())
                    .unwrap()
                    .unwrap_or(0);

            name = options[selection].to_string();
        }

        crate::trace!("name: {} provided", &name);

        if dialoguer::Confirm::new()
            .with_prompt(format!("You are about to remove Template: name:{}", &name))
            .interact()
            .unwrap_or(false)
        {
            crate::trace!("Removed Template");
            config.remove_template(&name);
        } else {
            crate::trace!("Not removing template");
        }
        Ok(())
    }
}
