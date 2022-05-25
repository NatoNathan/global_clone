use crate::config;
use clap::Args;
use config::AppConfig;
use log::{debug, trace, warn, info};

use git2::{Cred, RemoteCallbacks, CredentialHelper};
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
    #[clap(short='k', long,requires = "ssh_clone" ,default_value_t = format!("{}/.ssh/id_rsa", env::var("HOME").unwrap()) )]
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

    /// Dry run, do not clone
    #[clap(long, short)]
    dry_run: bool,
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
}

pub fn command(args: CliArgs, config: AppConfig) {
    let repo_type = get_repo_type(&args.repo);
    let repo_meta = get_repo_meta(&args.repo, &repo_type);
    trace!(
        "repo: {}, owner:{}, provider:{}",
        repo_meta.repo,
        repo_meta.owner,
        repo_meta.provider
    );
    let repo_path = match (&repo_type, &args.ssh) {
        (RepoType::Github, true )=> format!("git@github.com:{}.git", &args.repo),
        (RepoType::Github, false)=> format!("https://github.com/{}", &args.repo),
        (_, false) => args.repo,
        (_, true) => {
            let mut ssh_url = format!("git@{}:{}/{}.git", &repo_meta.provider, &repo_meta.owner, &repo_meta.repo);
            if let Some(username) = &args.ssh_username {
                ssh_url = format!("{}:{}@{}", username, &args.ssh_password.unwrap(), ssh_url);
            }
            trace!("ssh_url: {}", ssh_url);
            ssh_url
        }
    };

    trace!("getting target path");
    let target_template = config.templates.get(&args.template).unwrap();
    let target_path = build_target_path(&target_template, &repo_meta);
    trace!("target_path: {}", target_path);

    let mut callbacks = RemoteCallbacks::new();
    if matches!(repo_type, RepoType::Ssh) || args.ssh {
        warn!("ssh clone is currently not supported");
        trace!("using ssh");
        trace!("ssh_key: {}", args.ssh_key);
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key_from_agent(username_from_url.unwrap())
        });
    } else {
        trace!("using http");
        // callbacks.credentials(|_url, username_from_url, _allowed_types| {
        //     CredentialHelper::
        // });

    }
    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    if args.branch.is_some() {
        builder.branch(args.branch.unwrap().as_str());
    }
    if args.dry_run {
        info!("dry run, not cloning");
        trace!("target_path: {}", target_path);
        trace!("repo_path: {}", repo_path);
        return;
    }
    builder
    .clone(repo_path.as_str(), Path::new(target_path.as_str()))
    .unwrap();
}

fn build_target_path(template_str: &String, repo_meta: &RepoMeta) -> String {
    let mut target_path = template_str.clone();
    target_path = target_path.replace("~", env::var("HOME").unwrap().as_str());
    let re = Regex::new(r"\{(.*?)\}").unwrap();
    let captures = re.captures_iter(&template_str).collect::<Vec<_>>();
    for cap in captures {
        let key = cap.get(1).unwrap().as_str();
        let value = match key {
            "repo" => repo_meta.repo.clone(),
            "owner" => repo_meta.owner.clone(),
            "provider" => repo_meta.provider.clone(),
            _ => "".to_string(),
        };
        target_path = target_path.replace(&cap.get(0).unwrap().as_str(), &value);
    }
    target_path
}

/// get the repo meta data from the repo string
/// e.g. "https://github.com/NatoNathan/global-clone"
/// returns RepoMeta {
///   repo: "global-clone",
///   owner: "NatoNathan",
///   provider: "github"
/// }
/// or "git@github:NatoNathan/global-clone"
/// returns RepoMeta {
///  repo: "global-clone",
/// owner: "NatoNathan",
/// provider: "github"
/// }
/// or "nato-nathan/global-clone"
/// returns RepoMeta {
///  repo: "global-clone",
/// owner: "nato-nathan",
/// provider: "github"
/// }
fn get_repo_meta(repo_path: &String, repo_type: &RepoType) -> RepoMeta {
    let re =
        Regex::new(r"([\da-z](?:[\da-z-]{0,61}[\da-z])?)\.+[\da-z][\da-z-]{0,61}[\da-z]").unwrap();
    match &repo_type {
        RepoType::Github => {
            trace!("RepoType::Github");
            let repo_path_split: Vec<&str> = repo_path.split("/").collect();
            RepoMeta {
                repo: repo_path_split[1].to_string(),
                owner: repo_path_split[0].to_string(),
                provider: "github".to_string(),
            }
        }
        RepoType::Http => {
            trace!("RepoType::Http");
            let path = repo_path.replace("https://", "");
            let repo_path_split: Vec<&str> = path.split("/").collect();
            let domain = re.captures(repo_path_split[0]).unwrap();
            RepoMeta {
                repo: repo_path_split[2].to_string(),
                owner: repo_path_split[1].to_string(),
                provider: domain[1].to_string(),
            }
        }
        RepoType::Ssh => {
            trace!("RepoType::Ssh");
            let repo_path_split: Vec<&str> = repo_path.split(":").collect();
            let repo_split: Vec<&str> = repo_path_split[1].split("/").collect();
            let domain = re.captures(repo_path_split[0]).unwrap();
            RepoMeta {
                repo: repo_split[1].to_string(),
                owner: repo_split[0].to_string(),
                provider: domain[1].to_string(),
            }
        }
    }
}

/// get the repo type from the repo path
fn get_repo_type(repo_path: &String) -> RepoType {
    if repo_path.contains("http") {
        RepoType::Http
    } else if repo_path.contains("@") {
        RepoType::Ssh
    } else {
        RepoType::Github
    }
}
