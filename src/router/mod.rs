use crate::{Config, DatabaseManager};
use anyhow::{anyhow, Result};
use hyper::{Body, Request, Response, Uri};
use hyper_staticfile::ResponseBuilder as StaticFileResponseBuilder;
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
mod search;
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

    async fn match_pattern(&self, path: &str, req: &Request<Body>) -> Option<Response<Body>> {
        if let Some(res) = self.map.match_pattern(path, req).await {
            return Some(res);
        }

        if let Some(res) = self.tree.match_pattern(path, req).await {
            return Some(res);
        }

        None
    }

    pub async fn route(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let route_table = ROUTE_TABLE.get().unwrap();
        let path = req.uri().path();

        if let Some(res) = route_table.match_pattern(path, &req).await {
            return Ok(res);
        }

        if path == "/" {
            let path = &Config::read().site.homepage;

            if let Some(res) = route_table.match_pattern(path, &req).await {
                return Ok(res);
            }
        }

        if let Some(res) = Self::serve_static(&req).await {
            return Ok(res);
        }

        if path == Config::read().url_patterns.update {
            if let Some(res) = update::handle(&mut req).await {
                return Ok(res);
            }
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

    async fn serve_static(req: &Request<Body>) -> Option<Response<Body>> {
        let path = req.uri().path();

        if path.starts_with("/pages/")
            || path.starts_with("/posts/")
            || path == "/template/header.html"
            || path == "/template/footer.html"
            || path == "/template/page_nav.html"
            || path == "/template/page.html"
            || path == "/template/post.html"
            || path == "/template/summary.html"
            || path == "/template/not_found.html"
        {
            return None;
        }

        let db = DatabaseManager::read().await;
        let root = db.repo.tempdir.path();

        if let Ok(result) = hyper_staticfile::resolve(root, req).await {
            let res = StaticFileResponseBuilder::new()
                .request(req)
                .build(result)
                .unwrap();

            return Some(res);
        }

        None
    }
}

pub struct PathMap {
    pub map: RwLock<HashMap<String, RouteType>>,
}

impl PathMap {
    fn init() -> Self {
        let mut map = HashMap::new();

        map.insert(
            Config::read().url_patterns.index.to_owned(),
            RouteType::Index,
        );

        map.insert(
            Config::read().url_patterns.search.to_owned(),
            RouteType::Search,
        );

        Self {
            map: RwLock::new(map),
        }
    }

    async fn clear(&self) {
        let mut map = self.map.write().await;

        map.clear();

        map.insert(
            Config::read().url_patterns.index.to_owned(),
            RouteType::Index,
        );

        map.insert(
            Config::read().url_patterns.search.to_owned(),
            RouteType::Search,
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

    async fn match_pattern(&self, path: &str, req: &Request<Body>) -> Option<Response<Body>> {
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
            Some(RouteType::Search) => {
                if let Some(res) = search::handle(req).await {
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
            Ok(_) | Err(InsertError::Conflict { .. }) => Ok(()),
            err => err,
        }
    }
}

impl PathTree {
    fn init() -> Result<Self> {
        let mut matcher = Node::new();

        let author_url = &Config::read().url_patterns.author;
        matcher.insert(author_url, RouteType::Author)?;
        matcher.insert(switch_trailing_slash(author_url), RouteType::Author)?;

        let archive_url = &Config::read().url_patterns.archive;
        matcher.insert(archive_url, RouteType::Archive)?;
        matcher.insert(switch_trailing_slash(archive_url), RouteType::Archive)?;
        let archive_url = Config::read().url_patterns.archive.replace("/:month", "");
        matcher
            .insert(switch_trailing_slash(&archive_url), RouteType::Archive)
            .ignore_conflict()?;
        matcher
            .insert(archive_url, RouteType::Archive)
            .ignore_conflict()?;

        Ok(Self { matcher })
    }

    async fn match_pattern(&self, path: &str, req: &Request<Body>) -> Option<Response<Body>> {
        if let Ok(matched) = self.matcher.at(path) {
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

pub fn get_page_num_and_pos_in_url(uri: &Uri) -> (usize, (usize, usize), bool) {
    let scheme_len = uri.scheme_str().map_or(0, |scheme| scheme.len() + 3);
    let authority_len = uri
        .authority()
        .map_or(0, |authority| authority.as_str().len());
    let path_len = uri.path().len();

    let query_pos_idx = scheme_len + authority_len + path_len;

    let (query_iter, mut start_idx) = if let Some(query) = uri.query() {
        (query.split('&'), query_pos_idx)
    } else {
        return (1, (query_pos_idx, query_pos_idx), true);
    };

    for param in query_iter {
        if let Some(page_num) = param.strip_prefix("page=") {
            let end_idx = start_idx + param.len() + 1;
            let is_page_num_the_first_param_in_query = start_idx == query_pos_idx;

            return if let Ok(page_num) = page_num.parse() {
                (
                    page_num,
                    (start_idx, end_idx),
                    is_page_num_the_first_param_in_query,
                )
            } else {
                (
                    1,
                    (start_idx, end_idx),
                    is_page_num_the_first_param_in_query,
                )
            };
        } else {
            start_idx += param.len() + 1;
        }
    }

    (1, (start_idx, start_idx), false)
}

#[derive(Clone)]
pub enum RouteType {
    Post { id: usize },
    Page { id: usize },
    Index,
    Author,
    Archive,
    Search,
}
