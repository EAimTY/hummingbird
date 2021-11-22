use crate::{data::Post, database::FileInfo, Config, Data};
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
    author_map: HashMap<String, (String, Vec<usize>)>,
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

        let author_map =
            data.iter()
                .enumerate()
                .fold(HashMap::new(), |mut author_map, (idx, post)| {
                    let posts = author_map
                        .entry(post.author().to_owned())
                        .or_insert_with(Vec::new);
                    posts.push(idx);
                    author_map
                });

        let author_map = author_map
            .into_iter()
            .map(|(author, posts)| {
                (
                    Config::read()
                        .url_patterns
                        .author_url
                        .replace("{author}", &author),
                    (author, posts),
                )
            })
            .collect();

        Ok(Self {
            data,
            url_map,
            author_map,
        })
    }

    pub fn get(&self, path: &str) -> Option<Data> {
        self.url_map.get(path).map(|id| Data::Post(&self.data[*id]))
    }

    pub fn get_index(&self) -> Option<Data> {
        if self.data.is_empty() {
            return None;
        }

        let data = if Config::read().settings.index_posts_from_old_to_new {
            self.data
                .iter()
                .take(Config::read().settings.index_posts_count)
                .collect()
        } else {
            self.data
                .iter()
                .rev()
                .take(Config::read().settings.index_posts_count)
                .collect()
        };

        Some(Data::Index { data })
    }

    pub fn get_author(&self, path: &str) -> Option<Data> {
        if self.author_map.is_empty() {
            return None;
        }
        self.author_map.get(path).map(|(author, posts)| {
            let data = posts.iter().map(|id| &self.data[*id]).collect();
            Data::Author {
                data,
                author: author.to_owned(),
            }
        })
    }
}
