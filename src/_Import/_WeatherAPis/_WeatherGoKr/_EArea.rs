
use EArea::*;
#[derive(Debug, Clone, Copy)]
pub enum EArea {
    서울시_강남구,
    인천시_미추홀구,
}

impl EArea {
    pub fn To_Url(&self) -> &str {
        match self {
            서울시_강남구 => {
                return "https://www.weather.go.kr/w/wnuri-fct2021/main/digital-forecast.do?code=1168060000&unit=m%2Fs&hr1=Y&ts=&lat=37.4870274262116&lon=127.059492941655";
            }
            인천시_미추홀구 => {
                return "https://www.weather.go.kr/w/wnuri-fct2021/main/digital-forecast.do?code=2817756000&unit=m%2Fs&hr1=Y&ts=&lat=37.44950951729936&lon=126.655756690903";
            }
        }
    }

    pub fn To_String(&self) -> String {
        return format!("{:#?}", self);
    }
}
