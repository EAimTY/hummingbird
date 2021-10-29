pub use self::{archive::ArchiveData, post::PostData};

mod archive;
mod post;

pub enum Data {
    Post(PostData),
    Archive(ArchiveData),
    // ...
}
