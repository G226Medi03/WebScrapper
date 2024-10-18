use crate::_Import::*;
use rusqlite::Connection;

pub struct ArticleDb {
    con: Connection,
}

impl ArticleDb {
    pub fn new(dbName: &str) -> Self {
        return ArticleDb {
            con: Self::getCon(dbName),
        };
    }

    fn getTodayDate() -> String {
        let now = chrono::offset::Local::now();
        let todayDate = now.format("%Y%m%d-%H").to_string();
        return todayDate;
    }
    fn getCon(dbName: &str) -> Connection {
        let conn = Connection::open(dbName).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Article (
	                updatedDate	TEXT NOT NULL,
	                data	TEXT NOT NULL
                );",
            (),
        )
        .unwrap();

        return conn;
    }

    pub async fn Get_AllArticleData(&self) -> Option<String> {
        let updatedDate = Self::getTodayDate();

        let sql = format!(
            "SELECT * FROM Article WHERE updatedDate={:?};",
            &updatedDate
        );

        let mut stmt = self
            .con
            .prepare(&sql)
            .expect(&format!("err line 44, sql :{:?}", &sql));

        let row_iter = stmt
            .query_map([], |row| {
                let updatedDate: String = row.get(0).unwrap();
                let jsonData: String = row.get(1).unwrap();
                Ok((updatedDate, jsonData))
            })
            .unwrap();

        for row in row_iter {
            let (_, weatherData) = row.unwrap();
            return Some(weatherData);
        }

        return None;
    }

    pub fn Insert(&self, updatedDate: &str, articleData_sj: &str) {
        self.con
            .execute(
                "INSERT INTO Article (updatedDate, data) VALUES (?1, ?2)",
                (updatedDate, &articleData_sj),
            )
            .unwrap();
    }
}
