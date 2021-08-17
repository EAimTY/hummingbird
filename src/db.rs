use git2::{build::RepoBuilder, Cred, RemoteCallbacks, Repository};
use std::{fs, path};
use tempfile::TempDir;

pub struct Repo<'a> {
    builder: RepoBuilder<'a>,
    repo: Option<Repository>,
    path: path::PathBuf,
    posts: Vec<Post>,
}

impl<'a> Repo<'a> {
    pub async fn new(username: &'a str, token: &'a str) -> Repo<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_, _, _| Cred::userpass_plaintext(username, token));
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        let mut repo_builder = RepoBuilder::new();
        repo_builder.fetch_options(fetch_options);
        let tempdir = TempDir::new().expect("Failed to create temp directory");
        Repo {
            builder: repo_builder,
            repo: None,
            path: tempdir.into_path(),
            posts: Vec::new(),
        }
    }

    pub async fn fetch(&mut self, addr: &str) {
        self.repo = Some(
            self.builder
                .clone(addr, self.path.as_path())
                .expect("Failed to clone"),
        );
    }

    pub async fn get_posts(&mut self) {
        let posts_path = &mut self.path;
        posts_path.push("post");
        let posts_dir = posts_path.as_path().read_dir().unwrap();
        for file in posts_dir {
            let file_path = file.unwrap().path();
            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = String::from(file_path.file_name().unwrap().to_str().unwrap());
                    let content = fs::read_to_string(&file_path)
                        .expect("Something went wrong reading the file");
                    let post = Post {
                        title: title,
                        content: content,
                    };
                    self.posts.push(post);
                }
            }
        }
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
}

struct Post {
    title: String,
    content: String,
}
