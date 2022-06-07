use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use std::{fs, path};


// Test the 'gclone' command with no arguments
// The command should print help
#[test]
fn global_clone_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("gclone [OPTIONS] <SUBCOMMAND>"));
    Ok(())
}

// Test the 'gclone' command with an invalid subcommand
// The command should error message and usage and offer the help flag
#[test]
fn global_clone_invalid_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("fun");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: Found argument"))
        .stderr(predicate::str::contains("gclone [OPTIONS] <SUBCOMMAND>"))
        .stderr(predicate::str::contains("For more information try --help"));
    Ok(())
}

// Test the 'gclone' command with the 'clone' subcommand with a valid SSH URL and --dry-run
// The command should print the URL and exit with success
#[test]
fn global_clone_clone_ssh_dry_run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("-d")
        .arg("clone")
        .arg("git@github.com:NatoNathan/global_clone.git");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("INFO  global_clone::commands::clone > dry run: cloning"));
    Ok(())
}

// Test the 'gclone' command with the 'clone' subcommand with a valid HTTPS URL and --dry-run
// The command should print the URL and exit with success
// clone https://github.com/NatoNathan/global_clone.git
#[test]
fn global_clone_clone_https_dry_run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("-d")
        .arg("clone")
        .arg("https://github.com/NatoNathan/global_clone.git");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("INFO  global_clone::commands::clone > dry run: cloning"));
    Ok(())
}

// test the 'gclone' command with the 'clone' subcommand with an adhoc template and --dry-run
// The command should print the URL and exit with success
// clone repo = "https://github.com/NatoNathan/global_clone.git"
// template = "ci/{provider}/{owner}/{repo}"
#[test]
fn global_clone_clone_adhoc_template_dry_run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("clone")
        .arg("https://github.com/NatoNathan/global_clone.git")
        .arg("-t ci/{provider}/{owner}/{repo}")
        .arg("--dry-run");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("INFO  global_clone::commands::clone > dry run: cloning"))
        .stderr(predicate::str::contains("ci/github/NatoNathan/global_clone"));
    Ok(())
}

// Test the 'gclone' command with the 'clone' subcommand with an adhoc template using HTTPS
// The command should clone the repo and exit with success
// clone repo = "https://github.com/NatoNathan/global_clone.git"
// template = "ci/{provider}/{owner}/{repo}"
#[test]
fn global_clone_clone_adhoc_template_https() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("clone")
        .arg("https://github.com/NatoNathan/global_clone.git")
        .arg("-t ci/{provider}/{owner}/{repo}");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("INFO  global_clone::commands::clone > cloning"));

    // TODO: Check that the repo was cloned
    // The repo should be in the current directory under 'ci/github/NatoNathan/global_clone'
    Ok(())
}

// Test the 'gclone' command with the 'templates' subcommand
// the command should list the templates
#[test]
fn global_clone_templates_list() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("templates")
        .arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("default"))
        .stdout(predicate::str::contains("~/git/{provider}/{owner}/{repo}"));
    Ok(())
}

// Test the 'gclone' command with the 'templates add' subcommand
// the command should add a template
// add template test_ci ci/{provider}/{owner}/{repo}
#[test]
fn global_clone_templates_add() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gclone")?;
    cmd.arg("templates")
        .arg("add")
        .arg("-n='test_ci'")
        .arg("-t='ci/{provider}/{owner}/{repo}'")
        .arg("--yes");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("INFO  global_clone::commands::templates::add > Added new Template"));
    Ok(())
}