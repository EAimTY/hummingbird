use crate::{
    data::{Page, Post},
    database::{Pages, Posts, Theme, Update},
    Config,
};
use anyhow::Result;
use git2::{
    build::RepoBuilder, Cred, DiffFindOptions, FetchOptions, ProxyOptions, RemoteCallbacks,
    Repository, ResetType,
};
use std::{
    collections::{BinaryHeap, HashMap},
    ffi::OsStr,
    path::PathBuf,
};
use tempfile::TempDir;

pub struct Repo {
    repo: Repository,
    tempdir: TempDir,
}

impl Repo {
    pub fn init() -> Result<Self> {
        let mut builder = RepoBuilder::new();
        builder.fetch_options(get_fetch_options());

        let tempdir = TempDir::new()?;

        let repo = builder.clone(&Config::read().git.repository, tempdir.path())?;

        Ok(Self { repo, tempdir })
    }

    pub async fn get_update(&mut self) -> Result<Update> {
        self.fetch()?;

        let mut posts = BinaryHeap::new();
        let mut pages = BinaryHeap::new();

        for (path, info) in self.get_file_info()?.into_iter() {
            if path.starts_with("posts/") {
                let abs_path = self.tempdir.path().join(&path);

                let title = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let content = tokio::fs::read_to_string(abs_path).await?;
                let Author {
                    name: author,
                    email: author_email,
                } = info.author.unwrap();
                let create_time = info.create_time.unwrap();
                let modify_time = info.modify_time;

                let post = Post {
                    title,
                    content,
                    author,
                    author_email,
                    create_time,
                    modify_time,
                };

                posts.push(post);
            } else if path.starts_with("pages/") {
                let abs_path = self.tempdir.path().join(&path);

                let title = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let content = tokio::fs::read_to_string(abs_path).await?;
                let Author {
                    name: author,
                    email: author_email,
                } = info.author.unwrap();
                let create_time = info.create_time.unwrap();
                let modify_time = info.modify_time;

                let page = Page {
                    title,
                    content,
                    author,
                    author_email,
                    create_time,
                    modify_time,
                };

                pages.push(page);
            }
        }

        let posts = posts.into_sorted_vec();

        let posts_url_map = posts
            .iter()
            .enumerate()
            .map(|(idx, post)| (post.get_url(), idx))
            .collect::<HashMap<String, usize>>();

        let pages = pages.into_sorted_vec();

        let pages_url_map = pages
            .iter()
            .enumerate()
            .map(|(idx, page)| (page.get_url(), idx))
            .collect::<HashMap<String, usize>>();

        let posts = Posts::new(posts, posts_url_map);

        let pages = Pages::new(pages, pages_url_map);

        let theme = Theme::new();

        Ok(Update {
            theme,
            posts,
            pages,
        })
    }

    fn fetch(&mut self) -> Result<()> {
        let mut origin_remote = self.repo.find_remote("origin")?;

        origin_remote.fetch(
            &[&Config::read().git.branch],
            Some(&mut get_fetch_options()),
            None,
        )?;
        let oid = self.repo.refname_to_id(&format!(
            "refs/remotes/origin/{}",
            &Config::read().git.branch
        ))?;

        let object = self.repo.find_object(oid, None)?;
        self.repo.reset(&object, ResetType::Hard, None)?;

        Ok(())
    }

    fn get_file_info(&self) -> Result<HashMap<PathBuf, FileInfo>> {
        let mut info_map = HashMap::new();
        let mut status_map = HashMap::new();

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        for step in revwalk {
            let oid = step?;
            let commit = self.repo.find_commit(oid)?;
            let a = if commit.parents().len() == 1 {
                let parent = commit.parent(0)?;
                Some(parent.tree()?)
            } else {
                None
            };
            let b = commit.tree()?;

            let mut diff = self.repo.diff_tree_to_tree(a.as_ref(), Some(&b), None)?;
            diff.find_similar(Some(DiffFindOptions::new().renames(true)))?;

            let deltas = diff.deltas();

            for delta in deltas {
                match (delta.old_file().exists(), delta.new_file().exists()) {
                    (true, true) if delta.old_file().path() == delta.new_file().path() => {
                        let path = delta.new_file().path().unwrap().to_path_buf();

                        match status_map.entry(path.clone()).or_insert_with(|| {
                            if (path.starts_with(PathBuf::from("posts/"))
                                || path.starts_with(PathBuf::from("pages/")))
                                && path.extension() == Some(OsStr::new("md"))
                            {
                                FileStatus::Created
                            } else {
                                FileStatus::Deleted
                            }
                        }) {
                            FileStatus::Created => {
                                info_map
                                    .entry(path)
                                    .or_insert_with(|| FileInfo::new(commit.time().seconds()));
                            }
                            _ => {}
                        }
                    }

                    (false, true) => {
                        let path = delta.new_file().path().unwrap().to_path_buf();

                        match status_map.entry(path.clone()).or_insert_with(|| {
                            if (path.starts_with(PathBuf::from("posts/"))
                                || path.starts_with(PathBuf::from("pages/")))
                                && path.extension() == Some(OsStr::new("md"))
                            {
                                FileStatus::Created
                            } else {
                                FileStatus::Deleted
                            }
                        }) {
                            FileStatus::Created => {
                                let info = info_map
                                    .entry(path)
                                    .or_insert_with(|| FileInfo::new(commit.time().seconds()));
                                info.set(
                                    commit.author().name().unwrap_or("Anonymous"),
                                    commit.author().email(),
                                    commit.time().seconds(),
                                );
                            }
                            FileStatus::Renamed(new_path) => {
                                if let Some(info) = info_map.get_mut(new_path) {
                                    info.set(
                                        commit.author().name().unwrap_or("Anonymous"),
                                        commit.author().email(),
                                        commit.time().seconds(),
                                    );
                                } else {
                                    unreachable!();
                                }
                            }
                            FileStatus::Deleted => {}
                        }
                    }

                    (true, true) => {
                        let new_path = delta.new_file().path().unwrap().to_path_buf();
                        let old_path = delta.old_file().path().unwrap().to_path_buf();

                        let status = status_map
                            .entry(new_path.clone())
                            .or_insert_with(|| {
                                if new_path.starts_with(PathBuf::from("posts/"))
                                    || new_path.starts_with(PathBuf::from("pages/"))
                                {
                                    FileStatus::Created
                                } else {
                                    FileStatus::Deleted
                                }
                            })
                            .clone();
                        match status {
                            FileStatus::Created => {
                                info_map
                                    .entry(new_path.clone())
                                    .or_insert_with(|| FileInfo::new(commit.time().seconds()));
                                status_map.insert(old_path, FileStatus::Renamed(new_path.clone()));
                            }
                            FileStatus::Renamed(new_new_path) => {
                                status_map
                                    .insert(old_path, FileStatus::Renamed(new_new_path.clone()));
                                status_map.remove(&new_path);
                            }
                            FileStatus::Deleted => {
                                status_map.insert(old_path, FileStatus::Deleted);
                                status_map.remove(&new_path);
                            }
                        }
                    }

                    (true, false) => {
                        let path = delta.old_file().path().unwrap().to_path_buf();
                        status_map.insert(path, FileStatus::Deleted);
                    }

                    (false, false) => unreachable!(),
                }
            }
        }

        Ok(info_map)
    }
}

#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for Repo {}
unsafe impl Sync for Repo {}

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

#[derive(Clone)]
struct FileInfo {
    author: Option<Author>,
    create_time: Option<i64>,
    modify_time: i64,
}

impl FileInfo {
    fn new(modify_time: i64) -> Self {
        Self {
            author: None,
            create_time: None,
            modify_time,
        }
    }

    fn set(&mut self, author_name: &str, author_email: Option<&str>, create_time: i64) {
        self.author = Some(Author {
            name: author_name.to_string(),
            email: author_email.map(|email| email.to_string()),
        });
        self.create_time = Some(create_time);
    }
}

#[derive(Clone)]
struct Author {
    name: String,
    email: Option<String>,
}

#[derive(Clone)]
enum FileStatus {
    Created,
    Renamed(PathBuf),
    Deleted,
}
