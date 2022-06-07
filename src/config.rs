use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
  pub version: String,
  pub default_template: String,
  pub templates: HashMap<String, String>
}

impl std::default::Default for AppConfig {
  fn default() -> Self {
    Self {
      version: "0.1.0".into(),
      default_template: "default".into(),
      templates: HashMap::from([("default".into(), get_default_template())]),
    }
  }
}

impl AppConfig {
  pub fn get_default_template(&self) -> String {
    self.templates.get(&self.default_template).unwrap().to_string()
  }

  pub fn get_template(&self, name: &str) -> String {
    // if template name match template syntax, return name as is
    if  (name.contains("{") && name.contains("}") )|| name.contains("/") {
      return name.to_string();
    }

    let template = self.templates.get(name);
    match template {
      Some(t) => t.to_string(),
      None => self.get_default_template(),
    }
  }

  pub fn set_default_template(&mut self, name: &str) {
    self.default_template = name.to_string();
  }

  pub fn add_template(&mut self, name: &str, template: &str) {
    self.templates.insert(name.to_string(), template.to_string());
  }

  pub fn remove_template(&mut self, name: &str) {
    self.templates.remove(name);
  }
    
}

#[cfg(target_family = "unix")]
fn get_default_template() -> String {
  "~/git/{provider}/{owner}/{repo}".into()
}

#[cfg(target_family = "windows")]
fn get_default_template() -> String {
  "C:\\git\\{provider}\\{owner}\\{repo}".into()
}

pub fn get_config() -> AppConfig {
  confy::load("global_clone").unwrap()
}

pub fn save_config(config: AppConfig){
  confy::store("global_clone", config).unwrap();
}