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
    // ...
}

pub enum Time {
    Year(i32),
    Month(u32),
}
