mod parse;
mod render;

#[derive(Clone, Debug)]
pub struct Template;

impl Template {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}
