#[derive(Clone, Debug)]
pub enum SiteParameter {
    Name,
}

#[derive(Clone, Debug)]
pub enum DocumentParameter {
    Title,
}

#[derive(Clone, Debug)]
pub enum PageParameter {
    Title,
    Url,
    Content,
}

#[derive(Clone, Debug)]
pub enum PostParameter {
    Title,
    Url,
    Content,
}

#[derive(Clone, Debug)]
pub enum SummaryParameter {
    Title,
    Url,
    Content,
}