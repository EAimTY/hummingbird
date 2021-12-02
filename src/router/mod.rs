use hyper::{Body, Request, Response};
use once_cell::sync::OnceCell;
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::RwLock;

mod index;
mod not_found;
mod page;
mod post;
mod update;

static ROUTE_TABLE: OnceCell<RouteTable> = OnceCell::new();

#[derive(Debug)]
pub struct RouteTable {
    path_map: RwLock<PathMap>,
    path_trie: PathTrie,
}

#[derive(Debug)]
pub struct PathMap {
    pub pages: HashMap<String, usize>,
    pub posts: HashMap<String, usize>,
}

impl PathMap {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            posts: HashMap::new(),
        }
    }

    pub fn match_pattern(&self, path: &str) -> Option<DataType> {
        self.pages.get(path).map_or_else(
            || self.posts.get(path).map(|&id| DataType::Post { id }),
            |&id| Some(DataType::Page { id }),
        )
    }
}

#[derive(Debug)]
pub struct PathTrie;

impl PathTrie {
    pub fn new() -> Self {
        Self
    }

    pub fn match_pattern(&self, path: &str) -> Option<DataType> {
        None
    }
}

impl RouteTable {
    pub fn init() {
        ROUTE_TABLE
            .set(Self {
                path_trie: PathTrie::new(),
                path_map: RwLock::new(PathMap::new()),
            })
            .unwrap();
    }

    pub async fn route(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let route_table = ROUTE_TABLE.get().unwrap();

        let path_map = route_table.path_map.read().await;

        match path_map.match_pattern(&req.uri().path()) {
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

        match route_table.path_trie.match_pattern(&req.uri().path()) {
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
            _ => {}
        }

        Ok(not_found::handle(&req).await)
    }

    pub async fn update_page_map(page_map: HashMap<String, usize>) {
        let route_table = ROUTE_TABLE.get().unwrap();
        let mut path_map = route_table.path_map.write().await;
        path_map.pages = page_map;
    }

    pub async fn update_post_map(post_map: HashMap<String, usize>) {
        let route_table = ROUTE_TABLE.get().unwrap();
        let mut path_map = route_table.path_map.write().await;
        path_map.posts = post_map;
    }
}

pub enum DataType {
    Post { id: usize },
    Page { id: usize },
    Index,
    Update,
    NotFound,
}
