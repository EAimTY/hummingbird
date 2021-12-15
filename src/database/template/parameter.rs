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
    Link,
    Content,
}

#[derive(Clone, Debug)]
pub enum PostParameter {
    Title,
    Link,
    Content,
}

#[derive(Clone, Debug)]
pub enum SummaryParameter {
    Title,
    Link,
    Content,
}
