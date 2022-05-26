use log::{trace};

pub fn command(config:crate::config::AppConfig ) -> Result<(), Box<dyn std::error::Error>> {
  trace!("Templates:List");
  let default_template = &config.default_template.to_string();
  let mut table = prettytable::Table::new();
  table.add_row(row!["Name", "Template"]);
  for (key,val) in config.templates {
    if key == *default_template {
      table.add_row(row![format!("{}*", key), val]);
    } else {
      table.add_row(row![key, val]);
    }
  }
  table.printstd();

  Ok(())
}