use crate::config;
use crate::{info, warn, trace};

#[cfg(feature = "cli")] 
use {
    clap::Args,
    indicatif::ProgressBar,
    crate::{config::AppConfig, commands::CliCommand},
};

use git2::{Cred, RemoteCallbacks};
use std::{env, path::Path};
use regex::Regex;

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

#[cfg(feature = "cli")]
#[derive(Debug, Args)]
pub struct CloneCommand {
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

#[cfg(feature = "cli")]
impl CliCommand for CloneCommand {
    fn command(self, _config: AppConfig, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
        let args = self;

        let clone_options = CloneOptions::new(args.repo, &args.template, args.branch, args.ssh, Some(args.ssh_key), args.ssh_username, args.ssh_password);
        if dry_run {
            #[cfg(feature = "logging")]
            info!("dry run: cloning {} to {}, using {}", &clone_options.repo_path, &clone_options.target_path, &args.template);
            return Ok(());
        }

        #[cfg(feature = "logging")]
        info!("cloning {} to {}, using {}", &clone_options.repo_path, &clone_options.target_path, &args.template);

        clone_options.git_clone()?;
        Ok(())
    }
}

struct CloneOptions {
    repo_path: String,
    repo_type: RepoType,
    target_path: String,
    branch: Option<String>,
    ssh: bool,
    ssh_key: String,
    _ssh_username: Option<String>,
    ssh_password: Option<String>,
}

impl CloneOptions {
    pub fn new(
        repo_path: String,
        template: &str,
        branch: Option<String>,
        ssh: bool,
        ssh_key: Option<String>,
        ssh_username: Option<String>,
        ssh_password: Option<String>,
    ) -> Self {
        let config = config::get_config();
        let repo_type = get_repo_type(&repo_path);
        let repo_meta = get_repo_meta(&repo_path, &repo_type);
        let template_path = config.get_template(&template);
        let target_path = build_target_path(template_path.as_str(), &repo_meta);
        let ssh_key = ssh_key.unwrap_or_else(|| {
            warn!("no ssh key provided, using default");
            get_default_ssh_key_path()
        });
        let repo = build_repo_path(&repo_path, &repo_type, &ssh, &repo_meta, ssh_username.clone());
        Self {
            repo_path: repo,
            repo_type,
            target_path,
            branch,
            ssh,
            ssh_key,
            _ssh_username: ssh_username,
            ssh_password,
        }
    }

    pub fn git_clone(&self) -> Result<(), Box<dyn std::error::Error>> {
        let options = self;
        if !check_sh_availability() {
            #[cfg(feature = "logging")]
            warn!("sh not available, aborting");
            return Ok(());
        }

        let mut callbacks = RemoteCallbacks::new();
    
    
        // set up credentials for private repos
        callbacks.credentials(|url, username_from_url, _| {
            get_credentials_callback(
                &options.repo_type,
                username_from_url.unwrap_or("git"),
                options.ssh,
                options.ssh_key.clone(),
                options.ssh_password.clone(),
                url,
            )
        });

        // progress callback
        #[cfg(feature = "cli")]
        let progress_spinner: ProgressBar = ProgressBar::new_spinner();
        #[cfg(feature = "cli")]
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
    
        if options.branch.is_some() {
            builder.branch(options.branch.as_ref().unwrap().as_str());
        }
        builder.clone(options.repo_path.as_str(), Path::new(options.target_path.as_str()))?;
        
        #[cfg(feature = "cli")]
        progress_spinner.finish_with_message("Finished cloning");


        Ok(())

    }
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
            #[cfg(feature = "logging")]
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
            #[cfg(feature = "logging")]
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
            #[cfg(feature = "logging")]
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
            #[cfg(feature = "logging")]
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
    url: &str,
) -> Result<Cred, git2::Error> {

    #[cfg(feature = "logging")]
    trace!("get_credentials_callback");
    match (&repo_type, &ssh) {
        (RepoType::Ssh, true) => {
            #[cfg(feature = "logging")]
            trace!("using ssh");
            #[cfg(feature = "logging")]
            trace!("ssh_key: {}", ssh_key);

            // Warning: On windows, the key must be in the RSA format.
            // looks to be a bug in libssh2
            // See: https://github.com/rust-lang/git2-rs/issues/659#issuecomment-757527900
            // warn on windows
            #[cfg(feature = "logging")]
            if cfg!(target_family = "windows") {
                warn!("On windows, the key must be in the RSA format.");
            }

            Cred::ssh_key(
                username, // username
                None,
                Path::new(&ssh_key),
                ssh_password.as_ref().map(|p| p.as_ref()),
            )
        }
        (RepoType::Ssh, false) => {
            #[cfg(feature = "logging")]
            trace!("using ssh from agent");
            Cred::ssh_key_from_agent(username)
        }
        _ => {
            #[cfg(feature = "logging")]
            trace!("using http");
            let local_git_config = git2::Config::open_default()?;
            Cred::credential_helper(
                &local_git_config,
                url,
                Some(username),
            )
        }
    }
}


/// get default ssh key path on unix systems
#[cfg(target_family = "unix")]
fn get_default_ssh_key_path() -> String {
    #[cfg(feature = "logging")]
    trace!("get_default_ssh_key_path (unix)");
    let mut ssh_dir = env::var("HOME").unwrap();
    ssh_dir.push_str("/.ssh/");
    ssh_dir
}
/// get default ssh key path on windows systems
#[cfg(target_family = "windows")]
fn get_default_ssh_key_path() -> String {
    #[cfg(feature = "logging")]
    trace!("get_default_ssh_key_path (windows)");
    let mut ssh_dir = env::var("HOMEDRIVE").unwrap();
    ssh_dir.push_str(&env::var("HOMEPATH").unwrap());
    ssh_dir.push_str("\\.ssh\\");
    ssh_dir
}

/// Scan for ssh keys in the default ssh directory
/// and return the first one found
fn ssh_key_scan() -> String {
    #[cfg(feature = "logging")]
    trace!("ssh_key_scan");
    let ssh_dir = get_default_ssh_key_path();
    let mut keys = Vec::new();
    let re = Regex::new(r"(.*)\.pub").unwrap();
    for entry in std::fs::read_dir(&ssh_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_str = path.to_str().unwrap();
            if re.is_match(path_str) {
                #[cfg(feature = "logging")]
                trace!("found key: {}", path_str);
                keys.push(path_str.to_string());
            }
        }
    }
    if !keys.is_empty() {
        #[cfg(feature = "logging")]
        trace!("found keys: {:?}", keys);
        #[cfg(feature = "logging")]
        trace!("using key: {}", keys[0]);
        keys[0].clone().replace(".pub", "")
    } else {
        panic!("No ssh keys found in {}", ssh_dir);
    }
}

/// On Windows, check if `sh` is available. ie on the PATH.
/// 
/// If not, warn the user and return false.
#[cfg(target_family = "windows")]
fn check_sh_availability() -> bool {
    #[cfg(feature = "logging")]
    trace!("check_sh_availability");
    let output = std::process::Command::new("sh").output();
    match output {
        Ok(output) if output.status.success() => true,
        _ => {
            warn!("sh is not available on your system.");
            false
        }
    }
}
/// On Unix, return true.
#[cfg(target_family = "unix")]
fn check_sh_availability() -> bool {
    #[cfg(feature = "logging")]
    trace!("check_sh_availability");
    true
}