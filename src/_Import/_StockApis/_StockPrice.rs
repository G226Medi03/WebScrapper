use chrono::NaiveDate;

use crate::{Date2String, String2Date};

#[derive(Debug, Clone)]
pub struct StockPrice {
    pub date: NaiveDate,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
}

impl StockPrice {
    pub fn new(date: NaiveDate, open: f64, close: f64, high: f64, low: f64) -> Self {
        return StockPrice {
            date: date,
            open: open,
            close: close,
            high: high,
            low: low,
        };
    }

    pub fn Into_Json(&self) -> serde_json::Value {
        let json = serde_json::json!({
            "date" : Date2String(&self.date, "%Y%m%d"),
            "open" : self.open,
            "close" : self.close,
            "high" : self.high,
            "low" : self.low,
        });

        return json;
    }

    pub fn From_Json(json: &serde_json::Value) -> Self {
        let sDate = json["localDate"].as_str().unwrap().to_owned();

        let date = String2Date(&sDate);

        let open = json["openPrice"].as_f64().unwrap();
        let close = json["closePrice"].as_f64().unwrap();
        let high = json["highPrice"].as_f64().unwrap();
        let low = json["lowPrice"].as_f64().unwrap();

        return StockPrice::new(date, open, close, high, low);
    }
}
