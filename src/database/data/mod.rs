pub use self::{list::List, page::Page, post::Post};

mod list;
mod page;
mod post;

pub enum Data<'data> {
    Post(&'data Post),
    Page(&'data Page),
    List(List<'data>),
    // ...
}
