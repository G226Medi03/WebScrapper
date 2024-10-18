use crate::_Import::*;
use rusqlite::Connection;

pub struct WeatherDb {
    con: Connection,
}

impl WeatherDb {
    pub fn new(dbName: &str) -> Self {
        return WeatherDb {
            con: Self::getCon(dbName),
        };
    }

    fn getTodayDate() -> String {
        let now = chrono::offset::Local::now();
        let todayDate = now.format("%Y%m%d").to_string();
        return todayDate;
    }
    fn getCon(dbName: &str) -> Connection {
        let conn = Connection::open(dbName).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Weather (
                updatedDate TEXT PRIMARY KEY UNIQUE,
                data TEXT NOT NULL,
                area TEXT NOT NULL
            )",
            (),
        )
        .unwrap();

        return conn;
    }

    pub async fn Get_WeatherData(&self, area: EArea) -> Option<String> {
        let updatedDate = Self::getTodayDate();

        let sql = format!(
            "SELECT * FROM Weather WHERE updatedDate={:?} AND area={:#?};",
            &updatedDate,
            &area.To_String()
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

    pub fn Insert(&self, updatedDate: &str, sjWeatherData: &str, area: EArea) {
        self.con
            .execute(
                "INSERT INTO Weather (updatedDate, data, area) VALUES (?1, ?2, ?3)",
                (updatedDate, &sjWeatherData, area.To_String()),
            )
            .unwrap();
    }
}
