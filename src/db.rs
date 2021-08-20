use git2::{build::RepoBuilder, Cred, ProxyOptions, RemoteCallbacks, Repository};
use std::{fs, path};
use tempfile::TempDir;

pub struct Db<'a> {
    repo: Option<Repository>,
    url: String,
    builder: RepoBuilder<'a>,
    path: path::PathBuf,
    pages: Vec<Page>,
    posts: Vec<Post>,
}

impl<'a> Db<'a> {
    pub async fn new(url: &'a str, username: &'a str, token: &'a str, proxy: &'a str) -> Db<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_, _, _| Cred::userpass_plaintext(username, token));
        let mut proxy_option = ProxyOptions::new();
        proxy_option.url(proxy);
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.proxy_options(proxy_option);
        fetch_options.remote_callbacks(callbacks);
        let mut repo_builder = RepoBuilder::new();
        repo_builder.fetch_options(fetch_options);
        let tempdir = TempDir::new().expect("Failed to create temp directory");
        Db {
            repo: None,
            url: String::from(url),
            builder: repo_builder,
            path: tempdir.into_path(),
            pages: Vec::new(),
            posts: Vec::new(),
        }
    }

    pub async fn fetch(&mut self) {
        self.repo = Some(
            self.builder
                .clone(&self.url, self.path.as_path())
                .expect("Failed to clone"),
        );
        self.update_pages().await;
        self.update_posts().await;
    }

    async fn update_pages(&mut self) {
        let pages_dir_path = &mut self.path;
        pages_dir_path.push("pages");
        let mut pages: Vec<Page> = Vec::new();
        let pages_dir = pages_dir_path.as_path().read_dir().unwrap();
        for file in pages_dir {
            let file_path = file.unwrap().path();
            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = String::from(file_path.file_stem().unwrap().to_str().unwrap());
                    let content = fs::read_to_string(&file_path)
                        .expect("Something went wrong reading the file");
                    let page = Page {
                        title: title,
                        content: content,
                    };
                    pages.push(page);
                }
            }
        }
        self.pages = pages;
    }

    async fn update_posts(&mut self) {
        let posts_dir_path = &mut self.path;
        posts_dir_path.push("posts");
        let mut posts: Vec<Post> = Vec::new();
        let posts_dir = posts_dir_path.as_path().read_dir().unwrap();
        for file in posts_dir {
            let file_path = file.unwrap().path();
            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = String::from(file_path.file_stem().unwrap().to_str().unwrap());
                    let content = fs::read_to_string(&file_path)
                        .expect("Something went wrong reading the file");
                    let post = Post {
                        title: title,
                        content: content,
                    };
                    posts.push(post);
                }
            }
        }
        self.posts = posts;
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
