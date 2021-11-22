use crate::data::Post;
use std::ops::Range;

pub enum List<'list> {
    Index {
        data: Vec<&'list Post>,
    },
    Author {
        data: Vec<&'list Post>,
        author: String,
    },
    TimeRange {
        data: Vec<&'list Post>,
        range: Range<i64>,
    },
}
