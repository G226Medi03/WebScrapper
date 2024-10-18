use super::{Article, ESection};

pub struct Section {
    section: ESection,
    articles: Vec<Article>,
}

impl Section {
    pub fn new(section: ESection, articles: Vec<Article>) -> Self {
        return Section {
            section: section,
            articles: articles,
        };
    }
}
