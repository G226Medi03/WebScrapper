use crate::_Import::*;

pub struct ArticleLocalApi {
    db: ArticleDb,
    articles: Option<String>,
}

impl ArticleLocalApi {
    fn getTodayDate() -> String {
        let now = chrono::offset::Local::now();
        let todayDate = now.format("%Y%m%d-%H").to_string();
        return todayDate;
    }

    pub fn new(dbName: &str) -> Self {
        return ArticleLocalApi {
            db: ArticleDb::new(dbName),
            articles: None,
        };
    }
    pub async fn Get_Response(&mut self) -> String {
        match &self.articles {
            Some(article) => {
                return article.clone();
            }
            _ => {}
        }
        let articles = match self.db.Get_AllArticleData().await {
            Some(_articles) => {
                self.articles = Some(_articles.clone());
                _articles
            }

            None => {
                let articles = NnewsScraper::Get_AllArticle_Json().await.to_string();

                self.db.Insert(&Self::getTodayDate(), &articles);
                self.articles = Some(articles.clone());
                articles
            }
        };

        return articles;
    }
}
