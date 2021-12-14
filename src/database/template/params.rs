use super::Part;
use crate::{
    database::{Page, Post},
    Config,
};

pub struct Params<'p> {
    data: [Option<&'p str>; 11],
}

impl<'p> Params<'p> {
    pub fn get(&self, param: &Part) -> &str {
        match param {
            Part::SiteName => self.data[0].unwrap(),
            Part::DocumentTitle => self.data[1].unwrap(),
            Part::PageTitle => self.data[2].unwrap(),
            Part::PageLink => self.data[3].unwrap(),
            Part::PageContent => self.data[4].unwrap(),
            Part::PostTitle => self.data[5].unwrap(),
            Part::PostLink => self.data[6].unwrap(),
            Part::PostContent => self.data[7].unwrap(),
            Part::SummaryTitle => self.data[8].unwrap(),
            Part::SummaryLink => self.data[9].unwrap(),
            Part::SummaryContent => self.data[10].unwrap(),
            _ => unreachable!(),
        }
    }

    pub fn from_page(page: &'p Page) -> Self {
        Self {
            data: [
                Some(&Config::read().site.name),
                Some(&page.title),
                Some(&page.path),
                Some(&page.title),
                Some(&page.content),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        }
    }

    pub fn from_post(post: &'p Post) -> Self {
        Self {
            data: [
                Some(&Config::read().site.name),
                Some(&post.title),
                None,
                None,
                None,
                Some(&post.path),
                Some(&post.title),
                Some(&post.content),
                None,
                None,
                None,
            ],
        }
    }
}
