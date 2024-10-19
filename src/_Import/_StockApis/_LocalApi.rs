use _StockApis::_Scraper;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};

use crate::_Import::*;

use Db as StockDb;
use _Scraper::Scraper as Naver;
pub struct LocalApi {
    db: StockDb,
}

impl LocalApi {
    pub fn new(dbName: &str) -> Self {
        return LocalApi {
            db: Db::new(dbName),
        };
    }

    fn adjust_EndDate(endDate: &mut NaiveDateTime) {
        const KOREA_STOCK_MARKET_OPEN_HOUR: u32 = 9;
        if endDate.hour() < KOREA_STOCK_MARKET_OPEN_HOUR - 1 {
            *endDate -= Duration::days(1);
        }
        match endDate.weekday() {
            chrono::Weekday::Sat => {
                *endDate -= Duration::days(1);
            }
            chrono::Weekday::Sun => {
                *endDate -= Duration::days(2);
            }
            _ => {}
        }
    }
    async fn get_StockCandles(&mut self, stock: EStock, startDate: NaiveDate) -> Vec<StockPrice> {
        const TIME_FMT: &str = "%Y%m%d";

        let sStartDate = Date2String(&startDate, TIME_FMT);

        let mut endDate = chrono::Utc::now().naive_local();
        Self::adjust_EndDate(&mut endDate);
        let sEndDate = Date2String(&endDate.date(), TIME_FMT);

        //Get dbPrices
        let mut prices = self
            .db
            .Get_StocksPrices(stock, sStartDate.clone(), sEndDate.clone())
            .await;

        //Get LastDate in dbPrices
        let lastDate = match prices.last() {
            Some(lastDate) => lastDate.date,
            None => String2Date(&sStartDate),
        };

        if Date2String(&lastDate, TIME_FMT) == sEndDate {
            return prices;
        }

        let apiPrices = Naver::Get_Prices(stock, Date2String(&lastDate, TIME_FMT), sEndDate).await;

        let mut apiPricesIter = apiPrices.iter();
        let _ = apiPricesIter.next();

        apiPricesIter.for_each(|price| {
            self.db.Insert(price, stock);
            prices.push(price.clone());
        });

        return prices;
    }
    pub async fn Get_StockCandles_Json(&mut self, sStock: &str, startDate: NaiveDate) -> String {
        let stock = EStock::From_String(sStock);

        let mut json = serde_json::json!([]);
        let arr = json.as_array_mut().unwrap();
        self.get_StockCandles(stock, startDate)
            .await
            .iter()
            .for_each(|elem| {
                arr.push(elem.Into_Json());
            });

        return json.to_string();
    }
}
