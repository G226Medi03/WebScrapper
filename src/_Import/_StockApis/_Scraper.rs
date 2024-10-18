use serde_json::Value;

use super::{EStock, StockPrice};

pub struct Scraper {}

impl Scraper {
    async fn get_sHtml(url: &str) -> String {
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();

        return html;
    }

    pub async fn Get_Prices(
        stock: EStock,
        sStartDate: String,
        sEndDate: String,
    ) -> Vec<StockPrice> {
        let resp = Self::get_sHtml(&stock.To_Url(&sStartDate, &sEndDate)).await;
        let mut jResp: Value = serde_json::from_str(&resp).unwrap();

        let refJson = jResp.as_array_mut().unwrap();

        let mut candles = Vec::with_capacity(64);
        for obj in refJson {
            let candle = StockPrice::From_Json(obj);
            candles.push(candle);
        }

        return candles;
    }
}
