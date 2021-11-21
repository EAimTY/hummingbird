use crate::data::Post;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Authors<'data> {
    pub authors_map: HashMap<String, Vec<&'data Post>>,
}
