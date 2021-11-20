use super::Post;
use std::ops::Range;

pub enum List<'list> {
    Index {
        data: Vec<&'list Post>,
    },
    TimeRange {
        data: Vec<&'list Post>,
        range: Range<i64>,
    },
}
