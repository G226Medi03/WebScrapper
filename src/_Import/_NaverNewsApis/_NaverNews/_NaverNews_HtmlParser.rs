use scraper::{ElementRef, Selector};

use super::Article;


pub struct NnewsHtmlParser<'a> {
    elem: ElementRef<'a>,
    _selector: Option<Selector>,
}

impl<'a> NnewsHtmlParser<'a> {
    pub fn new(elem: ElementRef<'a>) -> Self {
        return NnewsHtmlParser {
            elem: elem,
            _selector: None,
        };
    }

    fn select(&mut self, selector: &str) -> scraper::element_ref::Select<'_, '_> {
        self._selector = Some(Selector::parse(selector).unwrap());
        return self.elem.select(&self._selector.as_ref().unwrap());
    }

    fn getTitle(&mut self) -> Option<String> {
        match self.select(".sa_text_strong").next() {
            Some(elem) => {
                return Some(elem.inner_html().replace("\"", "'"));
            }
            None => {
                return None;
            }
        }
    }

    fn getLink(&mut self) -> Option<String> {
        let elem = match self.select(".sa_thumb_inner>a").next() {
            Some(elem) => elem,
            None => return None,
        };

        let link = match elem.attr("href") {
            Some(imgSrc) => imgSrc.to_string(),
            None => {
                return None;
            }
        };

        return Some(link);
    }
    fn getImgSrc(&mut self) -> Option<String> {
        let elem = match self.select(".sa_thumb_link>img").next() {
            Some(elem) => elem,
            None => return None,
        };

        let imgSrc = match elem.attr("data-src") {
            Some(imgSrc) => imgSrc.to_string(),
            None => {
                return None;
            }
        };

        return Some(imgSrc);
    }

    pub fn GetArticle(&mut self) -> Option<Article> {
        let title = self.getTitle();
        let link = self.getLink();
        let imgSrc = self.getImgSrc();

        if title.is_none() | link.is_none() | imgSrc.is_none() {
            return None;
        }

        let article = Article::new(title.unwrap(), imgSrc.unwrap(), link.unwrap());
        return Some(article);
    }
}