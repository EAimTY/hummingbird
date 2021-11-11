pub use self::daemon::RepoDaemon;
use crate::{config::Config, database::DatabaseUpdate};
use anyhow::Result;
use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use tempfile::TempDir;
use tokio::sync::{mpsc, oneshot};

mod daemon;
mod fetch;
mod parse;
mod theme;

pub struct Repo<'repo> {
    repo: Repository,
    tempdir: TempDir,
    fetch_options: FetchOptions<'repo>,
}

impl<'repo> Repo<'repo> {
    pub fn init(
        repo_update_listener: mpsc::Receiver<oneshot::Sender<DatabaseUpdate>>,
    ) -> Result<RepoDaemon<'repo>> {
        let mut builder = RepoBuilder::new();
        builder.fetch_options(get_fetch_options());

        let tempdir = TempDir::new()?;

        let repo = builder.clone(&Config::read().git.repository, tempdir.path())?;

        Ok(RepoDaemon {
            repo: Self {
                repo,
                tempdir,
                fetch_options: get_fetch_options(),
            },
            repo_update_listener,
        })
    }
}

fn get_fetch_options<'repo>() -> FetchOptions<'repo> {
    let mut fetch_options = FetchOptions::new();

    if let Some(proxy_url) = Config::read().git.proxy.as_ref() {
        let mut proxy_option = ProxyOptions::new();
        proxy_option.url(proxy_url);
        fetch_options.proxy_options(proxy_option);
    }

    if let (Some(username), Some(password)) = (
        Config::read().git.user.as_ref(),
        Config::read().git.password.as_ref(),
    ) {
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(move |_, _, _| Cred::userpass_plaintext(username, password));
        fetch_options.remote_callbacks(remote_callbacks);
    }

    fetch_options
}
