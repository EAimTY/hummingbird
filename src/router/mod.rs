use crate::{config::Config, database::Database};
use hyper::{Body, Request, Response};
use once_cell::sync::OnceCell;
use regex::{Regex, RegexSet};
use std::convert::Infallible;

mod index;
mod page;
mod post;
mod update;

static ROUTER: OnceCell<Router> = OnceCell::new();

#[derive(Debug)]
pub struct Router {
    url_patterns: RegexSet,
}

impl Router {
    pub fn init() {
        let index_url = format!(
            "^{}$",
            regex::escape(&Config::read().url_patterns.index_url)
        );

        let update_url = format!(
            "^{}$",
            regex::escape(&Config::read().url_patterns.update_url)
        );

        let page_url = format!("^{}$", regex::escape(&Config::read().url_patterns.page_url));
        let page_placeholders = Regex::new("\\\\\\{slug\\\\\\}|\\\\\\{year\\\\\\}").unwrap();
        let page_url = page_placeholders
            .replace_all(&page_url, r"([A-Za-z\d._~!$&'()*+,;=:@%-])+")
            .to_string();

        let post_url = format!("^{}$", regex::escape(&Config::read().url_patterns.post_url));
        let post_placeholders = Regex::new("\\\\\\{slug\\\\\\}|\\\\\\{year\\\\\\}").unwrap();
        let post_url = post_placeholders
            .replace_all(&post_url, r"([A-Za-z\d._~!$&'()*+,;=:@%-])+")
            .to_string();

        let url_patterns = RegexSet::new(&[index_url, update_url, page_url, post_url]).unwrap();
        ROUTER.set(Self { url_patterns }).unwrap();
    }

    pub async fn route(
        mut database: Database,
        mut request: Request<Body>,
    ) -> Result<Response<Body>, Infallible> {
        let router = ROUTER.get().unwrap();

        for pattern in router.get_url_pattern(&request) {
            match pattern {
                0 => {
                    if let Some(response) = index::handle(&database, &request).await {
                        return Ok(response);
                    }
                }
                1 => {
                    if let Some(response) = update::handle(&mut database, &mut request).await {
                        return Ok(response);
                    }
                }
                2 => {
                    if let Some(response) = page::handle(&database, &request).await {
                        return Ok(response);
                    }
                }
                3 => {
                    if let Some(response) = post::handle(&database, &request).await {
                        return Ok(response);
                    }
                }
                _ => todo!(),
            }
        }

        Ok(Response::builder().status(404).body(Body::empty()).unwrap())
    }

    fn get_url_pattern(&self, request: &Request<Body>) -> impl Iterator<Item = usize> {
        let path = request.uri().path();
        self.url_patterns.matches(path).into_iter()
    }
}
