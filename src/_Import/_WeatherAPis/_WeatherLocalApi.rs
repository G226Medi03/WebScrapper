use _WeatherAPis::{EArea, WeatherDb, WeatherGoKr_WebScraper};

use crate::_Import::*;

pub struct WeatherLocalApi {
    db: WeatherDb,
    weathers: Option<String>,
}

impl WeatherLocalApi {
    fn getTodayDate() -> String {
        let now = chrono::offset::Local::now();
        let todayDate = now.format("%Y%m%d").to_string();
        return todayDate;
    }

    pub fn new(dbName: &str) -> Self {
        return WeatherLocalApi {
            db: WeatherDb::new(dbName),
            weathers: None,
        };
    }
    pub async fn Get_Weathers(&mut self, area: EArea) -> String {
        match &self.weathers {
            Some(weathers) => {
                return weathers.clone();
            }
            _ => {}
        }
        let weathers = match self.db.Get_WeatherData(area).await {
            Some(weathers) => {
                self.weathers = Some(weathers.clone());
                weathers
            }

            None => {
                let weathers = WeatherGoKr_WebScraper::Get_Weathers_Json(area)
                    .await
                    .to_string();

                self.db.Insert(&Self::getTodayDate(), &weathers, area);
                self.weathers = Some(weathers.clone());
                weathers
            }
        };

        return weathers;
    }
}
