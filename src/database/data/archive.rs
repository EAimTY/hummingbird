use super::Post;

pub struct Archive<'data> {
    list: Vec<&'data Post>,
}
