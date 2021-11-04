use crate::database::Post;

pub struct PostData<'a> {
    pub data: &'a Post,
}
