use anyhow::Error;

mod page;
mod post;

pub use self::{page::Page, post::Post};

pub enum Data<'data> {
    Post(&'data Post),
    Page(&'data Page),
    Index {
        data: Vec<&'data Post>,
    },
    Author {
        data: Vec<&'data Post>,
        author: String,
    },
    Archive {
        data: Vec<&'data Post>,
        range: DateRange,
    },
    Update(UpdateResult),
    NotFound,
}

#[derive(Debug)]
pub enum DateRange {
    Year(i32),
    Month(i32, u32),
}

pub enum UpdateResult {
    Success,
    PermissionDenied,
    Error(Error),
}
