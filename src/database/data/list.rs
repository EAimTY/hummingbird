use super::Post;
use std::ops::Range;

pub struct ListData<'data> {
    data: Vec<&'data Post>,
}

pub enum List<'list> {
    Index {
        data: ListData<'list>,
    },
    TimeRange {
        data: ListData<'list>,
        range: Range<i64>,
    },
}
