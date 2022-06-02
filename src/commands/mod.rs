use crate::config::AppConfig;

pub mod templates;
pub mod clone;

//#[cfg(feature = "cli")]
pub(super) trait CliCommand {
  fn command(args: Self, config: AppConfig, dry_run: bool,) -> Result<(), Box<dyn std::error::Error>>;

}