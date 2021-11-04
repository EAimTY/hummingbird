pub use self::{archive::ArchiveData, post::PostData};

mod archive;
mod post;

pub enum Query<'a> {
    Post(PostData<'a>),
    Archive(ArchiveData),
    // ...
}
