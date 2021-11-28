use crate::database::{Page, Post};
use anyhow::Error;

pub enum DataType<'data> {
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
