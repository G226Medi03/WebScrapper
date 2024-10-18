use scraper::{Html, Selector};
use serde_json::{json, Value};

use super::{Article, NnewsHtmlParser, _ESection::ESection};

pub struct NnewsScraper {}

impl NnewsScraper {
    async fn get_sHtml(url: &str) -> String {
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();

        return html;
    }

    pub async fn Get_Articles(section: ESection) -> Vec<Article> {
        let url = section.To_Url();
        let html = Self::get_sHtml(url).await;

        let mut articles = Vec::with_capacity(64);

        let document = Html::parse_document(&html);
        let selector = Selector::parse(".sa_item_flex").unwrap();

        for elem in document.select(&selector).into_iter() {
            let mut worker = NnewsHtmlParser::new(elem);
            let article = worker.GetArticle();

            if article.is_some() {
                articles.push(article.unwrap());
            }
        }
        return articles;
    }
    pub async fn Get_SectionArticle_Json(section: ESection) -> serde_json::Value {
        let mut arr: Vec<Value> = Vec::with_capacity(16);
        for article in Self::Get_Articles(section).await {
            let mut _json = article.Into_Json();
            arr.push(_json);
        }

        let json = json!({"section" : section.ToKorean(), "data" : arr});

        return json;
    }
    pub async fn Get_AllArticle_Json() -> serde_json::Value {
        let mut sectionDatas = Vec::with_capacity(8);

        let sections = ESection::All();

        for section in sections {
            let sectionJson = Self::Get_SectionArticle_Json(section).await;
            sectionDatas.push(sectionJson);
        }
        return json!(sectionDatas);
    }
}
