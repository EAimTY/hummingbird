use crate::{config::Config, database::DatabaseUpdate};
use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use tempfile::TempDir;
use tokio::sync::{mpsc, oneshot};

mod extract;
mod fetch;
mod parse;
mod theme;

pub struct Repo<'a> {
    repo: Repository,
    tempdir: TempDir,
    fetch_options: FetchOptions<'a>,
}

impl<'a> Repo<'a> {
    pub fn init(
        config: &'a Config,
        repo_update_listener: mpsc::Receiver<oneshot::Sender<DatabaseUpdate>>,
    ) -> RepoDaemon {
        let fetch_options = {
            let mut fetch_options = FetchOptions::new();

            if let Some(proxy_url) = config.git.proxy.as_ref() {
                let mut proxy_option = ProxyOptions::new();
                proxy_option.url(proxy_url);
                fetch_options.proxy_options(proxy_option);
            }

            if let (Some(username), Some(password)) =
                (config.git.user.as_ref(), config.git.password.as_ref())
            {
                let mut remote_callbacks = RemoteCallbacks::new();
                remote_callbacks
                    .credentials(move |_, _, _| Cred::userpass_plaintext(username, password));
                fetch_options.remote_callbacks(remote_callbacks);
            }

            fetch_options
        };

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);

        let tempdir = TempDir::new().unwrap();

        let repo = builder
            .clone(config.git.repository.as_ref().unwrap(), tempdir.path())
            .unwrap();

        RepoDaemon {
            repo: Self {
                repo,
                tempdir,
                fetch_options: {
                    let mut fetch_options = FetchOptions::new();

                    if let Some(proxy_url) = config.git.proxy.as_ref() {
                        let mut proxy_option = ProxyOptions::new();
                        proxy_option.url(proxy_url);
                        fetch_options.proxy_options(proxy_option);
                    }

                    if let (Some(username), Some(password)) =
                        (config.git.user.as_ref(), config.git.password.as_ref())
                    {
                        let mut remote_callbacks = RemoteCallbacks::new();
                        remote_callbacks.credentials(move |_, _, _| {
                            Cred::userpass_plaintext(username, password)
                        });

                        fetch_options.remote_callbacks(remote_callbacks);
                    }

                    fetch_options
                },
            },
            repo_update_listener,
        }
    }
}

pub struct RepoDaemon<'a> {
    repo: Repo<'a>,
    repo_update_listener: mpsc::Receiver<oneshot::Sender<DatabaseUpdate>>,
}

impl<'a> RepoDaemon<'a> {
    pub async fn listen(mut self) {
        while let Some(responder) = self.repo_update_listener.recv().await {
            self.repo.fetch();

            let posts = self.repo.get_posts();

            let update = DatabaseUpdate { posts };
            responder.send(update).unwrap();
        }
    }
}
