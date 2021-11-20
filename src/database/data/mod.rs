mod list;
mod page;
mod post;

pub use self::{list::List, page::Page, post::Post};

pub enum Data<'data> {
    Post(&'data Post),
    Page(&'data Page),
    List(List<'data>),
    // ...
}
