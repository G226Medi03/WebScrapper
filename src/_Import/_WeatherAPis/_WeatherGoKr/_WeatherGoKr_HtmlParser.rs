use scraper::{ElementRef, Selector};

use crate::_Import::_WeatherAPis::{EDay, EWeather, Weather};
#[allow(non_camel_case_types)]
pub struct WeatherGoKrParser<'a> {
    elem: ElementRef<'a>,
    _selector: Option<Selector>,
}

impl<'a> WeatherGoKrParser<'a> {
    pub fn new(elem: ElementRef<'a>) -> Self {
        return WeatherGoKrParser {
            elem: elem,
            _selector: None,
        };
    }

    fn select(&mut self, selector: &str) -> scraper::element_ref::Select<'_, '_> {
        self._selector = Some(Selector::parse(selector).unwrap());

        let selected = self.elem.select(self._selector.as_ref().unwrap());
        return selected;
    }
    fn getDate(&mut self) -> i8 {
        let selected = self.select("h4>span");

        let mut text = selected.into_iter().next().unwrap().inner_html();
        text = text[..text.len() - 3].to_owned();
        return text.parse::<i8>().unwrap();
    }

    fn getDay(&mut self) -> EDay {
        let selected = self.select("h4");

        let parent = selected.into_iter().next().unwrap();

        let mut childrenIter = parent.child_elements();

        let _ = childrenIter.next().unwrap();

        let text = childrenIter.next().unwrap().inner_html();

        return EDay::From_SingleKorean(&text[1..text.len() - 1]);
    }

    fn isAllDayWeatherCard(&mut self) -> bool {
        let selected = self.select(".daily-pop-am");

        match selected.into_iter().next() {
            Some(_) => {
                return false;
            }
            None => {
                return true;
            }
        }
    }

    fn getDividedEWeather(&mut self) -> EWeather {
        let amWeather = self.getAmEWeather();
        let pmWeather = self.getPmEWeather();

        let weather = if amWeather >= pmWeather {
            amWeather
        } else {
            pmWeather
        };

        return weather;
    }

    fn getAmEWeather(&mut self) -> EWeather {
        let selector = ".daily-weather-am>div";
        return self.getEWeather(selector);
    }

    fn getPmEWeather(&mut self) -> EWeather {
        let selector = ".daily-weather-pm>div";
        return self.getEWeather(selector);
    }

    fn getEWeather(&mut self, selector: &str) -> EWeather {
        let mut iter = self.select(selector).into_iter();

        let _ = iter.next().unwrap();

        let elem = iter.next().unwrap();
        let mut text = elem.inner_html();

        if text == "-" {
            return EWeather::FromKorean(&text);
        }

        text = elem.child_elements().next().unwrap().inner_html();

        return EWeather::FromKorean(&text);
    }

    fn getDividedRainPercent(&mut self) -> Option<i8> {
        let amRainPercent = self.getAmRainPercent();
        let pmRainPercent = self.getPmRainPercent();

        if amRainPercent == None {
            return pmRainPercent;
        } else if pmRainPercent == None {
            return amRainPercent;
        }

        if amRainPercent.unwrap() >= pmRainPercent.unwrap() {
            return amRainPercent;
        } else {
            return pmRainPercent;
        }
    }

    fn getAmRainPercent(&mut self) -> Option<i8> {
        let selector = ".daily-pop-am>span";
        return self.getRainPercent(selector);
    }

    fn getPmRainPercent(&mut self) -> Option<i8> {
        let selector = ".daily-pop-pm>span";
        return self.getRainPercent(selector);
    }

    fn get_iRainPercentFromStr(text: &str) -> Option<i8> {
        if text == "-" {
            return None;
        }
        return Some((&text[..text.len() - 1]).parse::<i8>().unwrap());
    }
    fn getRainPercent(&mut self, selector: &str) -> Option<i8> {
        let mut selected = self.select(selector).into_iter();
        let elem = selected.next().unwrap();

        return Self::get_iRainPercentFromStr(&elem.inner_html());
    }

    /// It returns (lowTemp, highTemp)
    ///
    /// #Examples
    ///
    /// ```
    /// let (lowTemp, highTemp) = self.getTemp()
    /// ```
    fn getTemp(&mut self) -> (Option<i8>, Option<i8>) {
        let mut iter = self.select(".daily-minmax>div>span").into_iter();

        let sLowTemp = iter.next().unwrap().inner_html();
        let lowTemp = if sLowTemp == "-" {
            None
        } else {
            Some(sLowTemp[..sLowTemp.len() - 3].parse::<i8>().unwrap())
        };
        let sHighTemp = iter.next().unwrap().inner_html();

        let highTemp = if sHighTemp == "-" {
            None
        } else {
            Some(sHighTemp[..sHighTemp.len() - 3].parse::<i8>().unwrap())
        };

        return (lowTemp, highTemp);
    }

    pub fn getAllDayEWeather(&mut self) -> EWeather {
        let mut selected = self.select(".daily-weather-allday>div>span").into_iter();

        let text = selected.next().unwrap().inner_html();

        return EWeather::FromKorean(&text);
    }

    pub fn getAllDayRainPercent(&mut self) -> Option<i8> {
        let mut selected = self.select(".daily-pop-allday>span").into_iter();

        let text = selected.next().unwrap().inner_html();
        return Self::get_iRainPercentFromStr(&text);
    }

    pub fn Get_Weather(&mut self) -> Weather {
        let date = self.getDate();
        let day = self.getDay();

        let (eWeather, rainPercent) = if !self.isAllDayWeatherCard() {
            (self.getDividedEWeather(), self.getDividedRainPercent())
        } else {
            (self.getAllDayEWeather(), self.getAllDayRainPercent())
        };

        let (lowTemp, highTemp) = self.getTemp();

        let weather = Weather::new(date, day, eWeather, lowTemp, highTemp, rainPercent);
        return weather;
    }
}
