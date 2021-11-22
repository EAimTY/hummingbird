use crate::{data::Page, database::FileInfo};
use anyhow::Result;
use regex::Regex;
use std::{
    collections::{BinaryHeap, HashMap},
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::fs;

#[derive(Debug)]
pub struct Pages {
    data: Vec<Page>,
    url_map: HashMap<String, usize>,
    author_map: HashMap<String, Vec<usize>>,
}

impl Pages {
    pub async fn from_file_info(
        file_info: HashMap<PathBuf, FileInfo>,
        tempdir: &Path,
    ) -> Result<Self> {
        let mut data = BinaryHeap::new();
        let page_url_regex_args = Regex::new("(\\{slug\\})").unwrap();

        for (path, info) in file_info.into_iter() {
            if path.extension() == Some(OsStr::new("md")) {
                let abs_path = tempdir.join(&path);

                let title = path.file_stem().unwrap().to_str().unwrap().to_owned();
                let content = fs::read_to_string(abs_path).await?;

                let page = Page::new(
                    title,
                    content,
                    info.author_name.unwrap(),
                    info.author_email,
                    info.create_time.unwrap(),
                    info.modify_time,
                    &page_url_regex_args,
                );
                data.push(page);
            }
        }

        let data = data.into_sorted_vec();

        let url_map = data
            .iter()
            .enumerate()
            .map(|(idx, page)| (page.url().to_owned(), idx))
            .collect::<HashMap<String, usize>>();

        let author_map = HashMap::new();

        Ok(Self {
            data,
            url_map,
            author_map,
        })
    }

    pub fn get(&self, path: &str) -> Option<&Page> {
        self.url_map.get(path).map(|id| &self.data[*id])
    }
}
