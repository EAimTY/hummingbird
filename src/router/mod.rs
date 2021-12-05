use crate::Config;
use anyhow::{anyhow, Result};
use hyper::{Body, Request, Response};
use matchit::Node as PathTrie;
use once_cell::sync::OnceCell;
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::RwLock;

mod index;
mod not_found;
mod page;
mod post;
mod update;

static ROUTE_TABLE: OnceCell<RouteTable> = OnceCell::new();

pub struct RouteTable {
    path_map: RwLock<PathMap>,
    path_trie: PathTrie<DataType>,
}

pub struct PathMap {
    pub map: HashMap<String, DataType>,
}

impl PathMap {
    pub fn init() -> Result<Self> {
        let mut map = HashMap::new();

        map.insert(
            Config::read().url_patterns.index_url.to_owned(),
            DataType::Index,
        );

        map.insert(
            Config::read().url_patterns.update_url.to_owned(),
            DataType::Update,
        );

        Ok(Self { map })
    }

    pub fn match_pattern(&self, path: &str) -> Option<DataType> {
        self.map.get(path).cloned()
    }
}

impl RouteTable {
    pub fn init() -> Result<()> {
        ROUTE_TABLE
            .set(Self {
                path_trie: PathTrie::new(),
                path_map: RwLock::new(PathMap::init()?),
            })
            .map_err(|_| anyhow!("Failed to initialize route table"))?;

        Ok(())
    }

    pub async fn route(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let route_table = ROUTE_TABLE.get().unwrap();

        let path_map = route_table.path_map.read().await;

        match path_map.match_pattern(&req.uri().path()) {
            Some(DataType::Index) => {
                if let Some(res) = index::handle(&req).await {
                    return Ok(res);
                }
            }
            Some(DataType::Update) => {
                if let Some(res) = update::handle(&mut req).await {
                    return Ok(res);
                }
            }
            Some(DataType::Page { id: page_id }) => {
                if let Some(res) = page::handle(&req, page_id).await {
                    return Ok(res);
                }
            }
            Some(DataType::Post { id: post_id }) => {
                if let Some(res) = post::handle(&req, post_id).await {
                    return Ok(res);
                }
            }
            _ => {}
        }

        match route_table.path_trie.at(&req.uri().path()) {
            _ => {}
        }

        Ok(not_found::handle(&req).await)
    }

    pub async fn clear_path_map() -> Result<()> {
        let route_table = ROUTE_TABLE.get().unwrap();
        let mut path_map = route_table.path_map.write().await;

        *path_map = PathMap::init()?;

        Ok(())
    }

    pub async fn update_page_map(page_map: HashMap<String, usize>) {
        let route_table = ROUTE_TABLE.get().unwrap();
        let mut path_map = route_table.path_map.write().await;
        path_map.map.extend(
            page_map
                .into_iter()
                .map(|(path, id)| (path, DataType::Page { id })),
        );
    }

    pub async fn update_post_map(post_map: HashMap<String, usize>) {
        let route_table = ROUTE_TABLE.get().unwrap();
        let mut path_map = route_table.path_map.write().await;
        path_map.map.extend(
            post_map
                .into_iter()
                .map(|(path, id)| (path, DataType::Post { id })),
        );
    }
}

#[derive(Clone)]
pub enum DataType {
    Post { id: usize },
    Page { id: usize },
    Index,
    Update,
    NotFound,
}
