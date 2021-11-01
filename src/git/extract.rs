use super::Repo;
use crate::database::Post;
use std::{collections::HashMap, str};

impl Repo<'_> {
    pub fn get_posts(&self) -> HashMap<String, Post> {
        let mut posts = HashMap::new();

        let mut posts_dir_path = self.tempdir.path().to_path_buf();
        posts_dir_path.push("posts");
        let posts_dir = posts_dir_path.as_path().read_dir().unwrap();

        for file in posts_dir {
            let file_path = file.unwrap().path();

            if let Some(extension) = file_path.extension() {
                if extension == "md" {
                    let title = file_path.file_stem().unwrap().to_string_lossy();

                    let oid = self.repo.blob_path(file_path.as_path()).unwrap();
                    let blob = self.repo.find_blob(oid).unwrap();

                    let content = str::from_utf8(blob.content()).unwrap();
                    let post = Post {
                        content: content.into(),
                    };
                    posts.insert(title.into(), post);
                }
            }
        }

        posts
    }
}
