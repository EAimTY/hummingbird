#[derive(Clone, Debug)]
pub enum SiteParameter {
    Url,
    Name,
    Description,
    PageList,
    RecentPosts,
}

#[derive(Clone, Debug)]
pub enum DocumentParameter {
    Title,
    Url,
    Breadcrumb,
    PageNav,
    CurrentPageNumInList,
    TotalNumOfArticleInList,
}

#[derive(Clone, Debug)]
pub enum PageParameter {
    Title,
    Url,
    Content,
    Author,
    CreateTime,
    ModifyTime,
}

#[derive(Clone, Debug)]
pub enum PostParameter {
    Title,
    Url,
    Content,
    Author,
    CreateTime,
    ModifyTime,
}

#[derive(Clone, Debug)]
pub enum SummaryParameter {
    Title,
    Url,
    Summary,
    Author,
    CreateTime,
    ModifyTime,
}
