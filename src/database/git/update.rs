use super::super::{git, Repo, Theme, Update};
use crate::Config;
use anyhow::Result;
use git2::{DiffFindOptions, ResetType};
use std::{collections::HashMap, path::PathBuf};

impl Repo {
    pub async fn get_update(&mut self) -> Result<Update> {
        self.fetch()?;

        let (page_files_info_map, post_files_info_map) = self.get_page_and_post_files_info()?;

        Ok(Update {
            theme: Theme::new(),
            page_files_info_map,
            post_files_info_map,
        })
    }

    fn fetch(&mut self) -> Result<()> {
        let mut origin_remote = self.repo.find_remote("origin")?;

        origin_remote.fetch(
            &[&Config::read().git.branch],
            Some(&mut git::get_fetch_options()),
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

    fn get_page_and_post_files_info(
        &self,
    ) -> Result<(HashMap<PathBuf, GitFileInfo>, HashMap<PathBuf, GitFileInfo>)> {
        let mut page_files_info_map = HashMap::new();
        let mut post_files_info_map = HashMap::new();

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
                    // Update file
                    (true, true) if delta.old_file().path() == delta.new_file().path() => {
                        let path = delta.new_file().path().unwrap().to_path_buf();

                        if let FileStatus::Created(content_type) =
                            status_map.entry(path.clone()).or_insert_with(|| {
                                if path.starts_with(PathBuf::from("pages/")) {
                                    FileStatus::Created(ContentType::Page)
                                } else if path.starts_with(PathBuf::from("posts/")) {
                                    FileStatus::Created(ContentType::Post)
                                } else {
                                    FileStatus::Deleted
                                }
                            })
                        {
                            let entry = if let ContentType::Page = content_type {
                                page_files_info_map.entry(path)
                            } else {
                                post_files_info_map.entry(path)
                            };

                            entry.or_insert_with(|| {
                                GitFileInfo::new(
                                    commit.time().seconds()
                                        + commit.time().offset_minutes() as i64 * 60,
                                )
                            });
                        }
                    }

                    // Create file
                    (false, true) => {
                        let path = delta.new_file().path().unwrap().to_path_buf();

                        match status_map.entry(path.clone()).or_insert_with(|| {
                            if path.starts_with(PathBuf::from("pages/")) {
                                FileStatus::Created(ContentType::Page)
                            } else if path.starts_with(PathBuf::from("posts/")) {
                                FileStatus::Created(ContentType::Post)
                            } else {
                                FileStatus::Deleted
                            }
                        }) {
                            FileStatus::Created(content_type) => {
                                let entry = if let ContentType::Page = content_type {
                                    page_files_info_map.entry(path)
                                } else {
                                    post_files_info_map.entry(path)
                                };

                                let info = entry.or_insert_with(|| {
                                    GitFileInfo::new(
                                        commit.time().seconds()
                                            + commit.time().offset_minutes() as i64 * 60,
                                    )
                                });

                                info.set(
                                    commit.author().name(),
                                    commit.time().seconds()
                                        + commit.time().offset_minutes() as i64 * 60,
                                );
                            }
                            FileStatus::Renamed(new_path) => {
                                let info =
                                    page_files_info_map.get_mut(new_path).unwrap_or_else(|| {
                                        post_files_info_map.get_mut(new_path).unwrap()
                                    });
                                info.set(
                                    commit.author().name(),
                                    commit.time().seconds()
                                        + commit.time().offset_minutes() as i64 * 60,
                                );
                            }
                            FileStatus::Deleted => {}
                        }
                    }

                    // Rename file
                    (true, true) => {
                        let new_path = delta.new_file().path().unwrap().to_path_buf();
                        let old_path = delta.old_file().path().unwrap().to_path_buf();

                        match status_map
                            .entry(new_path.clone())
                            .or_insert_with(|| {
                                if new_path.starts_with(PathBuf::from("pages/")) {
                                    FileStatus::Created(ContentType::Page)
                                } else if new_path.starts_with(PathBuf::from("posts/")) {
                                    FileStatus::Created(ContentType::Post)
                                } else {
                                    FileStatus::Deleted
                                }
                            })
                            .clone()
                        {
                            FileStatus::Created(content_type) => {
                                let entry = if let ContentType::Page = content_type {
                                    page_files_info_map.entry(new_path.clone())
                                } else {
                                    post_files_info_map.entry(new_path.clone())
                                };

                                entry.or_insert_with(|| {
                                    GitFileInfo::new(
                                        commit.time().seconds()
                                            + commit.time().offset_minutes() as i64 * 60,
                                    )
                                });
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

                    // Delete file
                    (true, false) => {
                        let path = delta.old_file().path().unwrap().to_path_buf();
                        status_map.insert(path, FileStatus::Deleted);
                    }

                    (false, false) => unreachable!(),
                }
            }
        }

        Ok((page_files_info_map, post_files_info_map))
    }
}

#[derive(Debug, Clone)]
pub struct GitFileInfo {
    pub author: Option<String>,
    pub create_time: Option<i64>,
    pub modify_time: i64,
}

impl GitFileInfo {
    fn new(modify_time: i64) -> Self {
        Self {
            author: None,
            create_time: None,
            modify_time,
        }
    }

    fn set(&mut self, author: Option<&str>, create_time: i64) {
        self.author = author.map(|name| name.to_owned());
        self.create_time = Some(create_time);
    }
}

#[derive(Clone)]
enum ContentType {
    Page,
    Post,
}

#[derive(Clone)]
enum FileStatus {
    Created(ContentType),
    Renamed(PathBuf),
    Deleted,
}
