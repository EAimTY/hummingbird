use crate::Config;
use anyhow::{anyhow, Result};
use hyper::{Body, Request, Response};
use matchit::Node;
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
    path_map: RwLock<PathMap>,
    path_trie: PathTrie,
}

impl RouteTable {
    pub fn init() -> Result<()> {
        ROUTE_TABLE
            .set(Self {
                path_map: RwLock::new(PathMap::init()?),
                path_trie: PathTrie::init()?,
            })
            .map_err(|_| anyhow!("Failed to initialize route table"))?;

        Ok(())
    }

    pub async fn route(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let route_table = ROUTE_TABLE.get().unwrap();

        if let Some(res) = route_table
            .path_map
            .read()
            .await
            .match_pattern(&mut req)
            .await
        {
            return Ok(res);
        }

        if let Some(res) = route_table.path_trie.match_pattern(&req).await {
            return Ok(res);
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
                .map(|(path, id)| (path, RouteType::Page { id })),
        );
    }

    pub async fn update_post_map(post_map: HashMap<String, usize>) {
        let route_table = ROUTE_TABLE.get().unwrap();

        let mut path_map = route_table.path_map.write().await;

        path_map.map.extend(
            post_map
                .into_iter()
                .map(|(path, id)| (path, RouteType::Post { id })),
        );
    }
}

pub struct PathMap {
    pub map: HashMap<String, RouteType>,
}

impl PathMap {
    pub fn init() -> Result<Self> {
        let mut map = HashMap::new();

        map.insert(
            Config::read().url_patterns.index_url.to_owned(),
            RouteType::Index,
        );

        map.insert(
            Config::read().url_patterns.update_url.to_owned(),
            RouteType::Update,
        );

        Ok(Self { map })
    }

    pub async fn match_pattern(&self, req: &mut Request<Body>) -> Option<Response<Body>> {
        let path = req.uri().path();

        let matched = self
            .map
            .get(path)
            .map_or_else(|| self.map.get(&switch_trailing_slash(path)), |matched| Some(matched));

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

pub struct PathTrie {
    pub matcher: Node<RouteType>,
}

impl PathTrie {
    pub fn init() -> Result<Self> {
        let mut matcher = Node::new();

        let author_url = &Config::read().url_patterns.author_url;
        matcher.insert(author_url, RouteType::Author)?;
        matcher.insert(switch_trailing_slash(author_url), RouteType::Author)?;

        let archive_by_year_url = &Config::read().url_patterns.archive_by_year_url;
        matcher.insert(archive_by_year_url, RouteType::Archive)?;
        matcher.insert(switch_trailing_slash(archive_by_year_url), RouteType::Archive)?;

        let archive_by_year_and_month_url =
            &Config::read().url_patterns.archive_by_year_and_month_url;
        matcher.insert(archive_by_year_and_month_url, RouteType::Archive)?;
        matcher.insert(switch_trailing_slash(archive_by_year_and_month_url), RouteType::Archive)?;

        Ok(Self { matcher })
    }

    pub async fn match_pattern(&self, req: &Request<Body>) -> Option<Response<Body>> {
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
