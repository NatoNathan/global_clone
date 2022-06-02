use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub default_template: String,
    pub templates: HashMap<String, String>,
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            version: "0.1.0".into(),
            default_template: "default".into(),
            templates: HashMap::from([("default".into(), Self::get_default_template())]),
        }
    }
}

impl AppConfig {
    #[cfg(not(test))]
    pub fn get_config() -> AppConfig {
        confy::load("global_clone").unwrap()
    }
    
    #[cfg(not(test))]
    pub fn save_config(config: AppConfig) {
        confy::store("global_clone", config).unwrap();
    }

    pub fn get_template(&self, template: &str) -> String {
        self.templates.get(template).unwrap().clone().into()
    }
}

#[cfg(target_family = "unix")]
impl AppConfig {
    fn get_default_template() -> String {
        "~/git/{provider}/{owner}/{repo}".into()
    }
}

#[cfg(target_family = "windows")]
impl AppConfig {
    fn get_default_template() -> String {
        "C:\\git\\{provider}\\{owner}\\{repo}".into()
    }
}

#[cfg(test)]
impl AppConfig {
    pub fn get_config() -> Self {
        confy::load_path(std::path::Path::new("test_config.toml")).unwrap()
    }

    pub fn save_config(config: Self) {
        confy::store_path(std::path::Path::new("test_config.toml"), config).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_config() {
        let config = AppConfig::get_config();
        assert_eq!(config.version, "0.1.0");
    }

    #[test]
    fn test_save_config() {
        let mut config = AppConfig::get_config();
        config.version = "0.1.1".into();
        AppConfig::save_config(config);
        let config = AppConfig::get_config();
        assert_eq!(config.version, "0.1.1");
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_get_template() {
        let config = AppConfig::get_config();
        let template = config.get_template("default");
        assert_eq!(template, "~/git/{provider}/{owner}/{repo}");
    }

    #[test]
    #[cfg(target_family = "windows")]
    fn test_get_template() {
        let config = AppConfig::get_config();
        let template = config.get_template("default");
        assert_eq!(template, "C:\\git\\{provider}\\{owner}\\{repo}");
    }
}