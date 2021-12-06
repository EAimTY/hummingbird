use crate::Config;
use anyhow::{anyhow, Result};
use hyper::{http::uri::PathAndQuery, Body, Request, Response, Uri};
use matchit::Node as PathTrie;
use once_cell::sync::OnceCell;
use std::{collections::HashMap, convert::Infallible};
use tokio::sync::RwLock;

mod author;
mod index;
mod not_found;
mod page;
mod post;
mod update;

static ROUTE_TABLE: OnceCell<RouteTable> = OnceCell::new();

pub struct RouteTable {
    path_map: RwLock<PathMap>,
    path_trie: PathTrie<RouteType>,
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

    pub fn match_pattern(&self, path: &str) -> Option<RouteType> {
        self.map.get(path).cloned()
    }
}

impl RouteTable {
    pub fn init() -> Result<()> {
        let mut path_trie = PathTrie::new();
        path_trie.insert(&Config::read().url_patterns.author_url, RouteType::Author)?;

        ROUTE_TABLE
            .set(Self {
                path_trie,
                path_map: RwLock::new(PathMap::init()?),
            })
            .map_err(|_| anyhow!("Failed to initialize route table"))?;

        Ok(())
    }

    pub async fn route(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let route_table = ROUTE_TABLE.get().unwrap();

        let path_map = route_table.path_map.read().await;

        match path_map.match_pattern(req.uri().path()) {
            Some(RouteType::Index) => {
                if let Some(res) = index::handle(&req).await {
                    return Ok(res);
                }
            }
            Some(RouteType::Update) => {
                if let Some(res) = update::handle(&mut req).await {
                    return Ok(res);
                }
            }
            Some(RouteType::Page { id: page_id }) => {
                if let Some(res) = page::handle(&req, page_id).await {
                    return Ok(res);
                }
            }
            Some(RouteType::Post { id: post_id }) => {
                if let Some(res) = post::handle(&req, post_id).await {
                    return Ok(res);
                }
            }
            _ => {}
        }

        match route_table.path_trie.at(req.uri().path()) {
            Ok(matched) => match matched.value {
                RouteType::Author => {
                    let author = matched.params.get("author").unwrap_or("");
                    if let Some(res) = author::handle(&req, author).await {
                        return Ok(res);
                    }
                }
                _ => {}
            },
            Err(err) => {
                if err.tsr() {
                    return Ok(Self::tsr_redirect(req.uri()));
                }
            }
        }

        Ok(not_found::handle(&req).await)
    }

    fn tsr_redirect(uri: &Uri) -> Response<Body> {
        let redirect_uri = {
            let uri = uri.clone();
            let mut parts = uri.into_parts();

            if let Some(p_and_q) = parts.path_and_query {
                let new_path = if p_and_q.path().ends_with('/') {
                    p_and_q.path()[..p_and_q.path().len() - 1].to_owned()
                } else {
                    format!("{}/", p_and_q.path())
                };

                let new_p_and_q = p_and_q
                    .as_str()
                    .replace(p_and_q.path(), &new_path)
                    .into_bytes();
                parts.path_and_query = PathAndQuery::from_maybe_shared(new_p_and_q).ok();
            }

            Uri::from_parts(parts).unwrap().to_string()
        };

        Response::builder()
            .status(301)
            .header("Location", redirect_uri)
            .body(Body::empty())
            .unwrap()
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

#[derive(Clone)]
pub enum RouteType {
    Post { id: usize },
    Page { id: usize },
    Index,
    Update,
    Author,
}
