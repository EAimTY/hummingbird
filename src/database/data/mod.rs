pub use self::{archive::Archive, post::Post};

mod archive;
mod post;

pub enum Data<'data> {
    Post(&'data Post),
    Archive(Archive<'data>),
    // ...
}
