use super::{post::Post, DatabaseUpdate};
use crate::config::Config;
use git2::{build::RepoBuilder, Cred, FetchOptions, ProxyOptions, RemoteCallbacks, Repository};
use std::{collections::HashMap, str};
use tempfile::TempDir;
use tokio::sync::{mpsc, oneshot};

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

    pub fn fetch(&mut self) {
        let mut origin_remote = self.repo.find_remote("origin").unwrap();
        origin_remote
            .fetch(&["master"], Some(&mut self.fetch_options), None)
            .unwrap();
        let oid = self
            .repo
            .refname_to_id("refs/remotes/origin/master")
            .unwrap();
        let object = self.repo.find_object(oid, None).unwrap();
        self.repo
            .reset(&object, git2::ResetType::Hard, None)
            .unwrap();
    }

    pub fn get_posts(&self) -> HashMap<String, Post> {
        let mut posts = HashMap::new();

        let mut posts_dir_path = self.tempdir.path().to_path_buf();
        posts_dir_path.push("posts");
        let posts_dir = posts_dir_path.as_path().read_dir().unwrap();

        for file in posts_dir {
            let file_path = file.unwrap().path();

            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = file_path.file_stem().unwrap().to_string_lossy();

                    let oid = self.repo.blob_path(file_path.as_path()).unwrap();
                    let blob = self.repo.find_blob(oid).unwrap();

                    let content = str::from_utf8(blob.content()).unwrap();
                    let post = Post {
                        content: content.into(),
                    };
                    posts.insert(title.into(), post);
                }
            }
        }

        posts
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
