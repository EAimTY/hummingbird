use crate::database::{Page, Post};
use anyhow::Error;

pub enum DataType<'data> {
    Post(&'data Post),
    Page(&'data Page),
    Index { data: Vec<&'data Post> },
    Update(UpdateResult),
    NotFound,
}

pub enum UpdateResult {
    Success,
    PermissionDenied,
    Error(Error),
}
