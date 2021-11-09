use super::Post;

pub struct Archive<'archive> {
    pub posts: Vec<&'archive Post>,
}
