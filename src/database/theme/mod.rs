mod author;
mod index;
mod not_found;
mod page;
mod post;
mod update;

#[derive(Clone, Debug)]
pub struct Theme;

impl Theme {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}
