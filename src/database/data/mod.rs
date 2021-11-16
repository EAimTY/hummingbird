pub use self::{archive::Archive, page::Page, post::Post};

mod archive;
mod page;
mod post;

pub enum Data<'data> {
    Post(&'data Post),
    Page(&'data Page),
    Archive(Archive<'data>),
    // ...
}
