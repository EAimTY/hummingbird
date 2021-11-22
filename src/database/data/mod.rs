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
    Time {
        data: Vec<&'data Post>,
        time: Time,
    },
    Update(UpdateResult),
    NotFound,
    // ...
}

pub enum Time {
    Year(i32),
    Month(u32),
}

pub enum UpdateResult {
    Success,
    PermissionDenied,
    Error(Error),
}
