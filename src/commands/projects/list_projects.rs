use std::path::{PathBuf};

use log::trace;

use crate::config::AppConfig;
use crate::config;


#[derive(Debug, clap::Args)]
pub struct ListProjects {

    #[clap(short, long, default_value_t = config::AppConfig::get_config().default_template.to_string())]
    template: String,
}

impl ListProjects {
    #[cfg(test)]
    pub fn new() -> Self {
        ListProjects {
            template: config::AppConfig::get_config().default_template.to_string(),
        }
    }

    /// List projects in the picked template.
    pub fn command(&self, _config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        //!
        //! the algorithm for listing projects is:
        //! 1. build path until the first variable is found (e.g. {provider} in a template of the form {provider}/{owner}/{repo})
        //! 2. list all directories in the path
        //! 3. filter out directories that match other templates, for example: <br>
        //!    given  list_template: ~/git/{provider}/{owner}/{repo}<br>
        //!    and other_template: ~/git/apps/{provider}/{owner}/{repo} <br>
        //!    the directory ~/git/apps will be filtered out because it matches the other_template
        //! 4. recursively repeat 1-3 until no more variables are found
        //! 5. return the tree of directories
        //! 6. print the tree
        //! 7. return success
        log::info!("Listing projects");
        log::info!("Template: {}", self.template);


        Ok(())
    }


    /// list all directories in a given path
    /// 
    /// # Arguments
    /// * `path` - the path to list directories in
    /// 
    /// # Returns
    /// * `Vec<PathBuf>` - a vector of paths to directories
    /// 
    /// # Errors
    /// * `std::io::Error` - if the path does not exist
    fn list_dirs(path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        trace!("listing directories in {:?}", path);
        let mut dirs = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            trace!("{:?}", entry);
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }
        Ok(dirs)
    }

    /// Build a path from a template.
    /// 
    /// A function that takes a template and returns a path.
    /// 
    /// # Arguments
    /// * `path` - the path to build
    /// * `template` - the template to build the path from
    /// 
    /// # Returns
    /// * `PathBuf` - the built pate
    /// * `template` - the remaining template
    /// # Errors
    /// * `std::io::Error` - if the path cannot be built
    /// 
    pub fn build_path(mut path :PathBuf, template: &String) -> Result<PathBuf, std::io::Error> {
        trace!("Building path from template: {}", &template);
        // the algorithm for building a path is:
        // 1. split the template into parts (e.g. {provider}/{owner}/{repo}) using the '/' character as the delimiter
        let parts = (&template).split("/");
        // 2. for each part: check if its already in the path

        // 3. append each part to the path unless it is a variable (e.g. {provider} in the example)
        for part in parts {
            if part.contains('{') && part.contains('}') {
                // 3.1. if the part is a variable, return the path
                trace!("Built path: {}", path.display());
                return Ok(path);
            } else {
                // 3.2. if the part is not a variable, append it to the path (e.g. git -> git) and continue
                trace!("Appending part: {}", part);
                path.push(part);
            }
        }

        // TODO: add a custom error type for this
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not build path"))

    }


}


#[cfg(test)]
mod tests {
    use super::*;

  #[test]
  fn test_build_path() {
    let path = PathBuf::from("/tmp/");
    let template = "git/*/{owner}/{repo}".to_string();
    let result = ListProjects::build_path(path, &template);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PathBuf::from("/tmp/git/*/*"));
  }

    #[test]
    fn test_list_dirs() {
        let path = PathBuf::from("/test_dir/");
        let result = ListProjects::list_dirs(&path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::new());
    }
}