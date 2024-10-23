use _WeatherAPis::EDay;
use _WeatherAPis::Weather;
use chrono::Datelike;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use serde_json::Value;

use crate::_Import::_WeatherAPis;

use super::EArea;
use super::WeatherGoKrParser;

pub struct WeatherGoKr_WebScraper {}

impl WeatherGoKr_WebScraper {
    async fn get_sHtml(url: &str) -> String {
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();

        return html;
    }
    pub async fn Get_Weathers(area: EArea) -> Vec<Weather> {
        let url = area.To_Url();
        let html = Self::get_sHtml(url).await;

        let document = Html::parse_document(&html);
        let selector = Selector::parse(".dfs-daily-slide-box").unwrap();

        let mut weathers = Vec::with_capacity(8);
        for elem in document.select(&selector).into_iter() {
            let weather = WeatherGoKrParser::new(elem).Get_Weather();
            weathers.push(weather);
        }

        return weathers;
    }
    fn add_Json_Description(weather: &mut serde_json::Value) {
        let todayDay = chrono::offset::Local::now().weekday() as i32;
        match &weather["day"] {
            Value::String(sDay) => {
                let day = EDay::From_SingleKorean(&sDay);
                let uDay = day as i32;
                let diff = uDay - todayDay;
                match diff {
                    0 => {
                        weather["description"] = Value::String(String::from("오늘"));
                    }
                    1 => {
                        weather["description"] = Value::String(String::from("내일"));
                    }
                    2 => {
                        weather["description"] = Value::String(String::from("모레"));
                    }
                    _ => {
                        let singleKorean = day.To_SingleKorean().to_owned();
                        weather["description"] = Value::String(singleKorean);
                    }
                }
            }
            _ => {
                panic!("line 49")
            }
        }
    }
    pub async fn Get_Weathers_Json(area: EArea) -> serde_json::Value {
        let now = chrono::Local::now();
        let mut arr: Vec<Value> = Vec::with_capacity(16);
        for weather in Self::Get_Weathers(area).await {
            if now.day() != 1 && weather.Get_todayDate() < now.day() as i8 {
                continue;
            }
            let mut _json = weather.Into_Json();
            Self::add_Json_Description(&mut _json);
            arr.push(_json);
        }

        return json!(arr);
    }
}
