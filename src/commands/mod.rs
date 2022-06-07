pub mod templates;
pub mod clone;

#[cfg(feature = "cli")]
pub trait CliCommand {
  fn command(self, config: crate::config::AppConfig, dry_run: bool) -> Result<(), Box<dyn std::error::Error>>;

}
