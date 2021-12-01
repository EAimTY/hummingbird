use crate::Config;
use hyper::{Body, Request, Response};
use once_cell::sync::OnceCell;
use regex::{Regex, RegexSet};
use std::convert::Infallible;

mod index;
mod not_found;
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
        let page_args = Regex::new(r"\\\{slug\\\}").unwrap();
        let page_url = page_args.replace_all(&page_url, r"([A-Za-z\d._~!$&'()*+,;=:@%-])+");

        let post_url = format!("^{}$", regex::escape(&Config::read().url_patterns.post_url));
        let post_args = Regex::new(r"\\\{slug\\\}|\\\{year\\\}|\\\{month\\\}").unwrap();
        let post_url = post_args.replace_all(&post_url, r"([A-Za-z\d._~!$&'()*+,;=:@%-])+");

        let url_patterns = RegexSet::new(&[
            &index_url,
            &update_url,
            page_url.as_ref(),
            post_url.as_ref(),
        ])
        .unwrap();

        ROUTER.set(Self { url_patterns }).unwrap();
    }

    pub async fn route(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let router = ROUTER.get().unwrap();

        for pattern in router.get_url_pattern(&req) {
            match pattern {
                UrlPatternKind::Index => {
                    if let Some(res) = index::handle(&req).await {
                        return Ok(res);
                    }
                }
                UrlPatternKind::Update => {
                    if let Some(res) = update::handle(&mut req).await {
                        return Ok(res);
                    }
                }
                UrlPatternKind::Page => {
                    if let Some(res) = page::handle(&req).await {
                        return Ok(res);
                    }
                }
                UrlPatternKind::Post => {
                    if let Some(res) = post::handle(&req).await {
                        return Ok(res);
                    }
                }
            }
        }

        Ok(not_found::handle(&req).await)
    }

    fn get_url_pattern(&self, req: &Request<Body>) -> impl Iterator<Item = UrlPatternKind> {
        let path = req.uri().path();
        self.url_patterns
            .matches(path)
            .into_iter()
            .map(|n| match n {
                0 => UrlPatternKind::Index,
                1 => UrlPatternKind::Update,
                2 => UrlPatternKind::Page,
                3 => UrlPatternKind::Post,
                _ => unreachable!(),
            })
    }
}

enum UrlPatternKind {
    Index,
    Update,
    Page,
    Post,
}
