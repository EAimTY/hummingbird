#[derive(Clone, Debug)]
pub enum SiteParameter {
    Url,
    Name,
    Description,
}

#[derive(Clone, Debug)]
pub enum DocumentParameter {
    Title,
    Url,
    PageNav,
    CurrentPage,
    TotalArticle,
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
