use crate::config;
use anyhow::{Context, Result};
use git2::{build::RepoBuilder, Cred, ProxyOptions, RemoteCallbacks, Repository};
use std::ffi::OsStr;
use std::fs;
use tempfile::TempDir;

pub struct Db<'a> {
    repo: Option<Repository>,
    url: String,
    builder: RepoBuilder<'a>,
    tempdir: TempDir,
    pages: Vec<Page>,
    posts: Vec<Post>,
}

impl<'a> Db<'a> {
    pub async fn new(config: &config::Config) -> Result<Db<'a>> {
        let url = config
            .git
            .repository
            .clone()
            .context("Failed to get repository from config")?;
        let mut fetch_options = git2::FetchOptions::new();
        if let Some(proxy) = config.git.proxy.clone() {
            let mut proxy_option = ProxyOptions::new();
            proxy_option.url(&proxy);
            fetch_options.proxy_options(proxy_option);
        }
        if let Some(user) = config.git.user.clone() {
            if let Some(password) = config.git.password.clone() {
                let mut callbacks = RemoteCallbacks::new();
                callbacks.credentials(move |_, _, _| Cred::userpass_plaintext(&user, &password));
                fetch_options.remote_callbacks(callbacks);
            }
        }
        let mut repo_builder = RepoBuilder::new();
        repo_builder.fetch_options(fetch_options);
        let tempdir = TempDir::new()?;
        Ok(Db {
            repo: None,
            url,
            builder: repo_builder,
            tempdir: tempdir,
            pages: Vec::new(),
            posts: Vec::new(),
        })
    }

    pub async fn fetch(&mut self) -> Result<()> {
        self.repo = Some(self.builder.clone(&self.url, self.tempdir.path())?);
        self.update_pages().await?;
        self.update_posts().await?;
        Ok(())
    }

    async fn update_pages(&mut self) -> Result<()> {
        let pages_dir_path = &mut self.tempdir.path().to_path_buf();
        pages_dir_path.push("pages");
        let mut pages: Vec<Page> = Vec::new();
        let pages_dir = pages_dir_path.as_path().read_dir()?;
        for file in pages_dir {
            let file_path = file?.path();
            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = String::from(
                        file_path
                            .file_stem()
                            .unwrap_or(OsStr::new(""))
                            .to_str()
                            .unwrap_or(""),
                    );
                    let content = fs::read_to_string(&file_path)
                        .with_context(|| format!("Failed to read page: {}", &title))?;
                    let page = Page {
                        title: title,
                        content: content,
                    };
                    pages.push(page);
                }
            }
        }
        self.pages = pages;
        Ok(())
    }

    async fn update_posts(&mut self) -> Result<()> {
        let posts_dir_path = &mut self.tempdir.path().to_path_buf();
        posts_dir_path.push("posts");
        let mut posts: Vec<Post> = Vec::new();
        let posts_dir = posts_dir_path.as_path().read_dir()?;
        for file in posts_dir {
            let file_path = file?.path();
            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = String::from(
                        file_path
                            .file_stem()
                            .unwrap_or(OsStr::new(""))
                            .to_str()
                            .unwrap_or(""),
                    );
                    let content = fs::read_to_string(&file_path)
                        .with_context(|| format!("Failed to read post: {}", &title))?;
                    let post = Post {
                        title: title,
                        content: content,
                    };
                    posts.push(post);
                }
            }
        }
        self.posts = posts;
        Ok(())
        /*let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
        for oid in revwalk {
            let oid = oid.unwrap();
            let commit = repo.find_commit(oid).unwrap();
            let parent = commit.parent(0).unwrap();
            let tree_commit = commit.tree().unwrap();
            let tree_parent = parent.tree().unwrap();
            let diff = repo
                .diff_tree_to_tree(Some(&tree_parent), Some(&tree_commit), None)
                .unwrap();
            diff.print(DiffFormat::NameOnly, |delta, _hunk, _line| {
                print!("{:?}", delta);
                true
            })
            .unwrap();
            break;
        }*/
    }

    pub fn get_pages(&self) -> Vec<Page> {
        self.pages.clone()
    }

    pub fn get_posts(&self) -> Vec<Post> {
        self.posts.clone()
    }
}

#[derive(Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
}

#[derive(Clone)]
pub struct Page {
    pub title: String,
    pub content: String,
}
