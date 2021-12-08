use crate::Config;
use anyhow::{anyhow, Result};
use hyper::{Body, Request, Response};
use matchit::{InsertError, Node};
use once_cell::sync::OnceCell;
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::RwLock;

mod archive;
mod author;
mod index;
mod not_found;
mod page;
mod post;
mod update;

static ROUTE_TABLE: OnceCell<RouteTable> = OnceCell::new();

pub struct RouteTable {
    map: PathMap,
    tree: PathTree,
}

impl RouteTable {
    pub fn init() -> Result<()> {
        ROUTE_TABLE
            .set(Self {
                map: PathMap::init(),
                tree: PathTree::init()?,
            })
            .map_err(|_| anyhow!("Failed to initialize route table"))?;

        Ok(())
    }

    pub async fn route(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let route_table = ROUTE_TABLE.get().unwrap();

        if let Some(res) = route_table.map.match_pattern(&mut req).await {
            return Ok(res);
        }

        if let Some(res) = route_table.tree.match_pattern(&req).await {
            return Ok(res);
        }

        Ok(not_found::handle(&req).await)
    }

    pub async fn clear() {
        let route_table = ROUTE_TABLE.get().unwrap();
        route_table.map.clear().await;
    }

    pub async fn update_pages(pages: HashMap<String, usize>) {
        let route_table = ROUTE_TABLE.get().unwrap();
        route_table.map.update_pages(pages).await;
    }

    pub async fn update_posts(posts: HashMap<String, usize>) {
        let route_table = ROUTE_TABLE.get().unwrap();
        route_table.map.update_posts(posts).await;
    }
}

pub struct PathMap {
    pub map: RwLock<HashMap<String, RouteType>>,
}

impl PathMap {
    fn init() -> Self {
        let mut map = HashMap::new();

        map.insert(
            Config::read().url_patterns.index_url.to_owned(),
            RouteType::Index,
        );

        map.insert(
            Config::read().url_patterns.update_url.to_owned(),
            RouteType::Update,
        );

        Self {
            map: RwLock::new(map),
        }
    }

    async fn clear(&self) {
        let mut map = self.map.write().await;

        map.clear();

        map.insert(
            Config::read().url_patterns.index_url.to_owned(),
            RouteType::Index,
        );

        map.insert(
            Config::read().url_patterns.update_url.to_owned(),
            RouteType::Update,
        );
    }

    async fn update_pages(&self, pages: HashMap<String, usize>) {
        let mut map = self.map.write().await;

        map.extend(
            pages
                .into_iter()
                .map(|(path, id)| (path, RouteType::Page { id })),
        );
    }

    async fn update_posts(&self, posts: HashMap<String, usize>) {
        let mut map = self.map.write().await;

        map.extend(
            posts
                .into_iter()
                .map(|(path, id)| (path, RouteType::Post { id })),
        );
    }

    async fn match_pattern(&self, req: &mut Request<Body>) -> Option<Response<Body>> {
        let path = req.uri().path();
        let map = self.map.read().await;

        let matched = map
            .get(path)
            .map_or_else(|| map.get(&switch_trailing_slash(path)), Some);

        match matched {
            Some(RouteType::Index) => {
                if let Some(res) = index::handle(req).await {
                    return Some(res);
                }
            }
            Some(RouteType::Update) => {
                if let Some(res) = update::handle(req).await {
                    return Some(res);
                }
            }
            Some(RouteType::Page { id: page_id }) => {
                if let Some(res) = page::handle(req, *page_id).await {
                    return Some(res);
                }
            }
            Some(RouteType::Post { id: post_id }) => {
                if let Some(res) = post::handle(req, *post_id).await {
                    return Some(res);
                }
            }
            _ => {}
        }

        None
    }
}

pub struct PathTree {
    pub matcher: Node<RouteType>,
}

trait IgnoreConflict {
    fn ignore_conflict(self) -> Result<(), InsertError>;
}

impl IgnoreConflict for Result<(), InsertError> {
    fn ignore_conflict(self) -> Result<(), InsertError> {
        match self {
            Ok(_) => Ok(()),
            Err(InsertError::Conflict { .. }) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl PathTree {
    fn init() -> Result<Self> {
        let mut matcher = Node::new();

        let author_url = &Config::read().url_patterns.author_url;
        matcher.insert(author_url, RouteType::Author)?;
        matcher.insert(switch_trailing_slash(author_url), RouteType::Author)?;

        let archive_url = &Config::read().url_patterns.archive_url;
        matcher.insert(archive_url, RouteType::Archive)?;
        matcher.insert(switch_trailing_slash(archive_url), RouteType::Archive)?;
        let archive_url = Config::read()
            .url_patterns
            .archive_url
            .replace("/:month", "");
        matcher
            .insert(switch_trailing_slash(&archive_url), RouteType::Archive)
            .ignore_conflict()?;
        matcher
            .insert(archive_url, RouteType::Archive)
            .ignore_conflict()?;

        Ok(Self { matcher })
    }

    async fn match_pattern(&self, req: &Request<Body>) -> Option<Response<Body>> {
        if let Ok(matched) = self.matcher.at(req.uri().path()) {
            match matched.value {
                RouteType::Author => {
                    let author = matched.params.get("author").unwrap();

                    if let Some(res) = author::handle(req, author).await {
                        return Some(res);
                    }
                }
                RouteType::Archive => {
                    let year = matched.params.get("year").unwrap();
                    let month = matched.params.get("month");

                    if let Some(res) = archive::handle(req, year, month).await {
                        return Some(res);
                    }
                }
                _ => {}
            }
        }

        None
    }
}

fn switch_trailing_slash(path: &str) -> String {
    if path.ends_with('/') {
        path[..path.len() - 2].to_owned()
    } else {
        format!("{}/", path)
    }
}

#[derive(Clone)]
pub enum RouteType {
    Post { id: usize },
    Page { id: usize },
    Index,
    Update,
    Author,
    Archive,
}
