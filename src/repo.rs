use crate::{config, db};
use anyhow::{anyhow, Result};
use git2::{build::RepoBuilder, Cred, ProxyOptions, RemoteCallbacks, Repository};
use std::{collections::HashMap, str};
use tempfile::TempDir;

pub struct Repo<'a> {
    builder: RepoBuilder<'a>,
    repo: Option<Repository>,
    remote_url: String,
    tempdir: TempDir,
}

impl<'a> Repo<'a> {
    pub fn init(git_config: &config::Git) -> Result<Self> {
        let mut fetch_options = git2::FetchOptions::new();

        if let Some(proxy) = git_config.proxy.clone() {
            let mut proxy_option = ProxyOptions::new();
            proxy_option.url(&proxy);

            fetch_options.proxy_options(proxy_option);
        }

        if let (Some(username), Some(password)) =
            (git_config.user.clone(), git_config.password.clone())
        {
            let mut remote_callbacks = RemoteCallbacks::new();
            remote_callbacks
                .credentials(move |_, _, _| Cred::userpass_plaintext(&username, &password));

            fetch_options.remote_callbacks(remote_callbacks);
        }

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);

        let remote_url = git_config
            .repository
            .clone()
            .ok_or_else(|| anyhow!("no remote git repo url specified"))?;
        let tempdir = TempDir::new()?;

        Ok(Self {
            builder,
            repo: None,
            remote_url,
            tempdir,
        })
    }

    pub fn fetch(&mut self) -> Result<()> {
        if let Some(_repository) = &self.repo {
            todo!();
        } else {
            let repo = self.builder.clone(&self.remote_url, self.tempdir.path())?;
            self.repo = Some(repo);
        }
        Ok(())
    }

    pub fn parse_posts(&self) -> Result<HashMap<String, db::Post>> {
        let repo = self.repo.as_ref().ok_or_else(|| anyhow!("no repository"))?;

        let mut posts = HashMap::new();

        let mut posts_dir_path = self.tempdir.path().to_path_buf();
        posts_dir_path.push("posts");
        let posts_dir = posts_dir_path.as_path().read_dir()?;

        for file in posts_dir {
            let file_path = file?.path();

            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = file_path
                        .file_stem()
                        .ok_or_else(|| anyhow!("no file name"))?
                        .to_string_lossy();

                    let oid = repo.blob_path(file_path.as_path())?;
                    let blob = repo.find_blob(oid)?;

                    let content = str::from_utf8(blob.content())?;

                    let post = db::Post::new(content.into());

                    posts.insert(title.into(), post);
                }
            }
        }

        Ok(posts)
    }

    pub fn parse_pages(&self) -> Result<HashMap<String, db::Page>> {
        let repo = self.repo.as_ref().ok_or_else(|| anyhow!("no repository"))?;

        let mut pages = HashMap::new();

        let mut pages_dir_path = self.tempdir.path().to_path_buf();
        pages_dir_path.push("pages");
        let pages_dir = pages_dir_path.as_path().read_dir()?;

        for file in pages_dir {
            let file_path = file?.path();

            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = file_path
                        .file_stem()
                        .ok_or_else(|| anyhow!("no file name"))?
                        .to_string_lossy()
                        .to_string();

                    let oid = repo.blob_path(file_path.as_path())?;
                    let blob = repo.find_blob(oid)?;

                    let content = str::from_utf8(blob.content())?.to_string();

                    let page = db::Page::new(content);

                    pages.insert(title, page);
                }
            }
        }

        Ok(pages)
    }
}
