
#[cfg(feature = "cli")]
#[derive(Debug, clap::Args)]
pub struct ListCommand {}

#[cfg(feature = "cli")]
impl  crate::commands::CliCommand for ListCommand {
  
    fn command(
        self,
        config: crate::config::AppConfig,
        _dry_run: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        crate::trace!("Templates:List");
        let default_template = &config.default_template.to_string();
        let mut table = prettytable::Table::new();
        table.add_row(row!["Name", "Template"]);
        for (key, val) in config.templates {
            if key == *default_template {
                table.add_row(row![format!("{}*", key), val]);
            } else {
                table.add_row(row![key, val]);
            }
        }
        table.printstd();

        Ok(())
    }
}
