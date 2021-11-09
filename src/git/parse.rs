use super::Repo;
use crate::database::{DatabaseUpdate, Post, Posts};
use git2::DiffFindOptions;
use std::{
    collections::{BinaryHeap, HashMap},
    ffi::OsStr,
    path::PathBuf,
};

#[derive(Clone)]
pub struct FileInfo {
    create_time: Option<i64>,
    modify_time: i64,
}

impl FileInfo {
    fn new(modify_time: i64) -> Self {
        Self {
            create_time: None,
            modify_time,
        }
    }

    fn set_create_time(&mut self, create_time: i64) {
        self.create_time = Some(create_time);
    }
}

#[derive(Clone)]
enum FileStatus {
    Created,
    Renamed(PathBuf),
    Deleted,
}

impl Repo<'_> {
    pub async fn get_database_update(&self) -> DatabaseUpdate {
        let mut posts = BinaryHeap::new();

        for (path, info) in self.get_file_info().into_iter() {
            let post = Post {
                title: path.file_name().unwrap().to_str().unwrap().to_owned(),
                content: tokio::fs::read_to_string(path).await.unwrap(),
                create_time: info.create_time.unwrap(),
                modify_time: info.modify_time,
            };

            posts.push(post);
        }

        let posts = posts.into_sorted_vec();

        let url_map = posts
            .iter()
            .enumerate()
            .map(|(idx, post)| (post.title.clone(), idx))
            .collect::<HashMap<String, usize>>();

        let posts = Posts {
            data: posts,
            url_map,
        };

        DatabaseUpdate { posts }
    }

    pub fn get_file_info(&self) -> HashMap<PathBuf, FileInfo> {
        let mut info_map = HashMap::<PathBuf, FileInfo>::new();
        let mut status_map = HashMap::<PathBuf, FileStatus>::new();

        let mut revwalk = self.repo.revwalk().unwrap();
        revwalk.push_head().unwrap();

        for step in revwalk {
            let oid = step.unwrap();
            let commit = self.repo.find_commit(oid).unwrap();
            let a = if commit.parents().len() == 1 {
                let parent = commit.parent(0).unwrap();
                Some(parent.tree().unwrap())
            } else {
                None
            };
            let b = commit.tree().unwrap();

            let mut diff = self
                .repo
                .diff_tree_to_tree(a.as_ref(), Some(&b), None)
                .unwrap();
            diff.find_similar(Some(DiffFindOptions::new().renames(true)))
                .unwrap();

            let deltas = diff.deltas();

            for delta in deltas {
                match (delta.old_file().exists(), delta.new_file().exists()) {
                    (true, true) if delta.old_file().path() == delta.new_file().path() => {
                        let path = delta.new_file().path().unwrap().to_path_buf();

                        match status_map.entry(path.clone()).or_insert_with(|| {
                            if path.starts_with(PathBuf::from("posts/"))
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
                                    .or_insert(FileInfo::new(commit.time().seconds()));
                            }
                            _ => {}
                        }
                    }

                    (false, true) => {
                        let path = delta.new_file().path().unwrap().to_path_buf();

                        match status_map.entry(path.clone()).or_insert_with(|| {
                            if path.starts_with(PathBuf::from("posts/"))
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
                                    .or_insert(FileInfo::new(commit.time().seconds()));
                                info.set_create_time(commit.time().seconds());
                            }
                            FileStatus::Renamed(new_path) => {
                                if let Some(info) = info_map.get_mut(new_path) {
                                    info.set_create_time(commit.time().seconds());
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
                                if new_path.starts_with(PathBuf::from("posts/")) {
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
                                    .or_insert(FileInfo::new(commit.time().seconds()));
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

        info_map
    }
}
