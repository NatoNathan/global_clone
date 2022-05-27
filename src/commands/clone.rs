use crate::config;
use clap::Args;
use config::AppConfig;
use indicatif::ProgressBar;
use log::info;
use log::trace;

use git2::{Cred, RemoteCallbacks};
use std::env;
use std::path::Path;

use regex::Regex;

#[derive(Debug, Args)]
pub struct CliArgs {
    /// The Git repository to be cloned e.g.
    /// "https://{provider}/{owner}/{repo}",
    /// "git@{provider}:{owner}/{repo}"
    repo: String,

    /// clone using ssh
    #[clap(long, group = "ssh_clone")]
    ssh: bool,
    /// ssh key path, (requires --ssh)
    #[clap(short='k', long,requires = "ssh_clone" ,default_value_t = ssh_key_scan() )]
    ssh_key: String,
    /// ssh username, (requires --ssh)
    #[clap(short = 'u', long, requires = "ssh_clone")]
    ssh_username: Option<String>,
    /// ssh password, (requires --ssh)
    #[clap(short = 'p', long, requires = "ssh_clone")]
    ssh_password: Option<String>,

    /// branch to checkout after clone
    #[clap(long, short)]
    branch: Option<String>,

    /// The template path the be used
    #[clap(long, short, default_value_t = config::get_config().default_template)]
    template: String,
}

enum RepoType {
    Http,
    Ssh,
    Github,
}

struct RepoMeta {
    repo: String,
    owner: String,
    provider: String,
    host: String,
}

pub fn command(
    args: CliArgs,
    config: AppConfig,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_type = get_repo_type(&args.repo);
    let repo_meta = get_repo_meta(&args.repo, &repo_type);
    trace!(
        "repo: {}, owner:{}, provider:{}",
        repo_meta.repo,
        repo_meta.owner,
        repo_meta.provider
    );

    let repo_path = build_repo_path(&args.repo, &repo_type, &args.ssh, &repo_meta, args.ssh_username);

    trace!("getting target path");
    let target_template = config.templates.get(&args.template).unwrap();
    let target_path = build_target_path(target_template, &repo_meta);
    trace!("target_path: {}", target_path);

    if dry_run {
        info!("dry run, not cloning");
        info!("dry run: cloning {} to {}, using {}", repo_path, target_path, &args.template);
        return Ok(());
    }

    let mut callbacks = RemoteCallbacks::new();


    // set up credentials for private repos
    callbacks.credentials(|_, username_from_url, _| {
        get_credentials_callback(
            &repo_type,
            username_from_url.unwrap_or("git"),
            args.ssh,
            args.ssh_key.clone(),
            args.ssh_password.clone(),
        )
    });

    // progress callback
    let progress_spinner: ProgressBar = ProgressBar::new_spinner();
    callbacks.transfer_progress(|progress| {
        // progress_spinner.set_message(format!("{}/{}", progress.received_objects(), progress.total_objects()));
        log::debug!("{}/{}", progress.received_objects(), progress.total_objects());
        true
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    if args.branch.is_some() {
        builder.branch(args.branch.unwrap().as_str());
    }
    builder.clone(repo_path.as_str(), Path::new(target_path.as_str()))?;
    
    progress_spinner.finish_with_message("Finished cloning");
    Ok(())
}

fn build_target_path(template_str: &str, repo_meta: &RepoMeta) -> String {
    let mut target_path = String::from(template_str);
    if cfg!(target_family = "unix") {
        target_path = target_path.replace('~', env::var("HOME").unwrap().as_str());
    } 
    let re = Regex::new(r"\{(.*?)\}").unwrap();
    let captures = re.captures_iter(template_str).collect::<Vec<_>>();
    for cap in captures {
        let key = cap.get(1).unwrap().as_str();
        let value = match key {
            "repo" => repo_meta.repo.clone(),
            "owner" => repo_meta.owner.clone(),
            "provider" => repo_meta.provider.clone(),
            _ => "".to_string(),
        };
        target_path = target_path.replace(&cap.get(0).unwrap().as_str(), &value)
    }
    target_path
}

fn build_repo_path(repo: &String ,repo_type: &RepoType, ssh: &bool, repo_meta: &RepoMeta, username:Option<String>) -> String {
    match (&repo_type, ssh) {
        (RepoType::Github, true) => format!("git@github.com:{}.git", &repo),
        (RepoType::Github, false) => format!("https://github.com/{}", &repo),
        (_, false) => repo.clone(),
        (_, true) => {
            let ssh_url = format!(
                "{}@{}:{}/{}.git",
                &username.unwrap_or_else(|| String::from("git")),
                &repo_meta.host,
                &repo_meta.owner,
                &repo_meta.repo.replace(".git", "")
            );
            trace!("ssh_url: {}", ssh_url);
            ssh_url
        }
    }
}

/// get the repo meta data from the repo string
fn get_repo_meta(repo_path: &str, repo_type: &RepoType) -> RepoMeta {
    let re =
        Regex::new(r"([\da-z](?:[\da-z-]{0,61}[\da-z])?)\.+[\da-z][\da-z-]{0,61}[\da-z]").unwrap();
    match &repo_type {
        RepoType::Github => {
            trace!("RepoType::Github");
            let repo_path_split: Vec<&str> = repo_path.split('/').collect();
            RepoMeta {
                repo: repo_path_split[1].to_string(),
                owner: repo_path_split[0].to_string(),
                provider: "github".to_string(),
                host: "github.com".to_string(),
            }
        }
        RepoType::Http => {
            trace!("RepoType::Http");
            let path = repo_path.replace("https://", "");
            let repo_path_split: Vec<&str> = path.split('/').collect();
            let domain = re.captures(repo_path_split[0]).unwrap();
            RepoMeta {
                repo: repo_path_split[2].to_string().replace(".git", ""),
                owner: repo_path_split[1].to_string(),
                provider: domain[1].to_string(),
                host: domain[0].to_string(),
            }
        }
        RepoType::Ssh => {
            trace!("RepoType::Ssh");
            let repo_path_split: Vec<&str> = repo_path.split(':').collect();
            let repo_split: Vec<&str> = repo_path_split[1].split('/').collect();
            let domain = re.captures(repo_path_split[0]).unwrap();
            RepoMeta {
                repo: repo_split[1].to_string().replace(".git", ""),
                owner: repo_split[0].to_string(),
                provider: domain[1].to_string(),
                host: domain[0].to_string(),
            }
        }
    }
}

/// get the repo type from the repo path
fn get_repo_type(repo_path: &str) -> RepoType {
    if repo_path.contains("http") {
        RepoType::Http
    } else if repo_path.contains('@') {
        RepoType::Ssh
    } else {
        RepoType::Github
    }
}

fn get_credentials_callback(
    repo_type: &RepoType,
    username: &str,
    ssh: bool,
    ssh_key: String,
    ssh_password: Option<String>,
) -> Result<Cred, git2::Error> {
    match (&repo_type, &ssh) {
        (RepoType::Ssh, true) => {
            trace!("using ssh");
            Cred::ssh_key(
                username, // username
                None,
                Path::new(&ssh_key),
                ssh_password.as_ref().map(|p| p.as_ref()),
            )
        }
        (RepoType::Ssh, false) => {
            trace!("using ssh from agent");
            Cred::ssh_key_from_agent(username)
        }
        _ => {
            trace!("using http");
            Cred::default()
        }
    }
}


/// get default ssh key path on unix systems
#[cfg(target_family = "unix")]
fn get_default_ssh_key_path() -> String {
    let mut ssh_dir = env::var("HOME").unwrap();
    ssh_dir.push_str("/.ssh/");
    ssh_dir
}
/// get default ssh key path on windows systems
#[cfg(target_family = "windows")]
fn get_default_ssh_key_path() -> String {
    let mut ssh_dir = env::var("HOMEDRIVE").unwrap();
    ssh_dir.push_str(&env::var("HOMEPATH").unwrap());
    ssh_dir.push_str("\\.ssh\\");
    ssh_dir
}

/// Scan for ssh keys in the default ssh directory
/// and return the first one found
fn ssh_key_scan() -> String {
    let ssh_dir = get_default_ssh_key_path();
    let mut keys = Vec::new();
    let re = Regex::new(r"(.*)\.pub").unwrap();
    for entry in std::fs::read_dir(&ssh_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_str = path.to_str().unwrap();
            if re.is_match(path_str) {
                keys.push(path_str.to_string());
            }
        }
    }
    if keys.len() > 0 {
        keys[0].clone().replace(".pub", "")
    } else {
        panic!("No ssh keys found in {}", ssh_dir);
    }
}