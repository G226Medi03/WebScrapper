use crate::{Date2String, String2Date, _Import::*};
use chrono::{Duration, NaiveDate};
use rusqlite::Connection;

pub struct Db {
    con: Connection,
}

impl Db {
    pub fn new(dbName: &str) -> Self {
        return Db {
            con: Self::getCon(dbName),
        };
    }

    fn getCon(dbName: &str) -> Connection {
        let conn = Connection::open(dbName).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS StockChart (
                date TEXT NOT NULL,
                stock TEXT NOT NULL,
                open REAL NOT NULL,
                close REAL NOT NULL,
                high REAL NOT NULL,
                low REAL NOT NULL
            )",
            (),
        )
        .unwrap();

        return conn;
    }

    pub async fn Get_StocksPrices(
        &self,
        stock: EStock,
        sStartDate: String,
        sEndDate: String,
    ) -> Vec<StockPrice> {
        let startDate = String2Date(&sStartDate);
        let endDate = String2Date(&sEndDate);

        const TIME_FMT: &str = "%Y%m%d";

        let sql = format!(
            "SELECT * FROM StockChart WHERE date BETWEEN {:#?} AND {:#?} AND stock={:#?} ORDER BY date ASC;",
            &Date2String(&startDate, TIME_FMT),
            &Date2String(&endDate, TIME_FMT),
            &stock.To_String()
        );

        let mut stmt = self
            .con
            .prepare(&sql)
            .expect(&format!("err line 48, sql :{:?}", &sql));

        let row_iter = stmt
            .query_map([], |row| {
                let sDate: String = row.get(0).unwrap();
                let open: f64 = row.get(2).unwrap();
                let close: f64 = row.get(3).unwrap();
                let high: f64 = row.get(4).unwrap();
                let low: f64 = row.get(5).unwrap();

                let date = String2Date(&sDate);
                Ok(StockPrice::new(date, open, close, high, low))
            })
            .unwrap();

        let mut prices = Vec::with_capacity(64);
        for row in row_iter {
            match row {
                Ok(price) => {
                    prices.push(price);
                }
                _ => {
                    break;
                }
            }
        }

        return prices;
    }

    pub fn Insert(&self, candle: &StockPrice, stock: EStock) {
        self.con
            .execute(
                "INSERT INTO StockChart (stock, date, open, close, high, low) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    stock.To_String(),
                    &Date2String(&candle.date, "%Y%m%d"),
                    candle.open,
                    candle.close,
                    candle.high,
                    candle.low,
                ),
            )
            .unwrap();
    }
}
