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