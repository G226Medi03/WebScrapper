use crate::_Import::*;
use _WeatherAPis::{EDay, EWeather};

#[derive(Debug)]
pub struct Weather {
    date: i8,
    day: EDay,
    weather: EWeather,
    lowTemp: Option<i8>,
    highTemp: Option<i8>,
    rainPercent: Option<i8>,
}

impl Weather {
    pub fn new(
        date: i8,
        day: EDay,
        weather: EWeather,
        lowTemp: Option<i8>,
        highTemp: Option<i8>,
        rainPercent: Option<i8>,
    ) -> Self {
        return Weather {
            date: date,
            day: day,
            weather: weather,
            lowTemp: lowTemp,
            highTemp: highTemp,
            rainPercent: rainPercent,
        };
    }

    pub fn Into_Json(&self) -> serde_json::Value {
        let json = serde_json::json!({
            "date" : self.date,
            "day" : self.day.To_SingleKorean(),
            "weather" : self.weather.ToKorean(),
            "lowTemp" : self.lowTemp,
            "highTemp" : self.highTemp,
            "rainPercent" : self.rainPercent,
        });

        return json;
    }
}
