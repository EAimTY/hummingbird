use crate::{
    data::{List, Post},
    database::FileInfo,
};
use anyhow::Result;
use regex::Regex;
use std::{
    collections::{BinaryHeap, HashMap},
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::fs;

#[derive(Debug)]
pub struct Posts {
    data: Vec<Post>,
    url_map: HashMap<String, usize>,
    author_map: HashMap<String, Vec<usize>>,
}

impl Posts {
    pub async fn from_file_info(
        file_info: HashMap<PathBuf, FileInfo>,
        tempdir: &Path,
    ) -> Result<Self> {
        let mut data = BinaryHeap::new();
        let post_url_regex_args = Regex::new(r"(\{slug\}|\{year\}|\{month\})").unwrap();

        for (path, info) in file_info.into_iter() {
            if path.extension() == Some(OsStr::new("md")) {
                let abs_path = tempdir.join(&path);

                let title = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let content = fs::read_to_string(abs_path).await?;

                let post = Post::new(
                    title,
                    content,
                    info.author_name.unwrap(),
                    info.author_email,
                    info.create_time.unwrap(),
                    info.modify_time,
                    &post_url_regex_args,
                );
                data.push(post);
            }
        }

        let data = data.into_sorted_vec();

        let url_map = data
            .iter()
            .enumerate()
            .map(|(idx, post)| (post.url().to_owned(), idx))
            .collect::<HashMap<String, usize>>();

        let author_map = HashMap::new();

        Ok(Self {
            data,
            url_map,
            author_map,
        })
    }

    pub fn get(&self, path: &str) -> Option<&Post> {
        self.url_map.get(path).map(|id| &self.data[*id])
    }

    pub fn get_index(&self, count: usize, from_old_to_new: bool) -> Option<List> {
        if self.data.is_empty() {
            return None;
        }

        let data = if from_old_to_new {
            self.data.iter().take(count).collect()
        } else {
            self.data.iter().rev().take(count).collect()
        };

        Some(List::Index { data })
    }
}
