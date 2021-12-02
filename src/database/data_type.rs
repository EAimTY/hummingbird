use super::{DatabaseUpdateResult, Page, Post};

pub enum DataType<'data> {
    Post(&'data Post),
    Page(&'data Page),
    Index { data: Vec<&'data Post> },
    Update(DatabaseUpdateResult),
    NotFound,
}
