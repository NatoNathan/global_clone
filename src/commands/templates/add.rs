
#[cfg(feature = "cli")]
use crate::{commands::CliCommand};



#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
pub struct AddCommand {
    /// Name of the template
    #[clap(short, long, group = "temp")]
    name: Option<String>,

    /// the template string
    #[clap(short, long, requires = "temp")]
    template: Option<String>,
}

#[cfg(feature = "cli")]
impl CliCommand for AddCommand {
    fn command(
        self,
        mut config: crate::config::AppConfig,
        _dry_run: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let args = self;


        crate::trace!("Templates:Add");
        let mut name: String = args.name.unwrap_or_default();
        let mut template: String = args.template.unwrap_or_default();
        if name == *"" {
            crate::trace!("No name or template provided, asking user instead");
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

        crate::trace!("name:{}, template:{} provided", &name, &template);

        if dialoguer::Confirm::new()
            .with_prompt(format!(
                "You are about to add a new Template: name:{}, template:{}",
                &name, &template
            ))
            .interact()
            .unwrap_or(false)
        {
            crate::trace!("Added new Template");
            config.add_template(&name, &template);
        } else {
            crate::trace!("Not adding new template");
        }
        Ok(())
    }
}