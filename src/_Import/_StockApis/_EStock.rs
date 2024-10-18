use actix_web::web::Bytes;
use EStock::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum EStock {
    BTC,
    NASDAQ,
    TMF,
    OILK,
    KT_G,
}

impl EStock {
    /// startDate and endDate should have format like "20230728"
    pub fn To_Url(&self, startDate: &str, endDate: &str) -> String {
        let url = format!(
            "https://api.stock.naver.com/chart/{}/item/{}/day?startDateTime={}&endDateTime={}",
            self.get_sNaverDomestic(),
            self.to_NaverStockTicker(),
            startDate,
            endDate
        );
        return url;
    }

    pub fn From_String(str: &str) -> Self {
        match str.to_uppercase().as_str() {
            "BTC" | "BITCOIN" => {
                return BTC;
            }
            "NASDAQ" => {
                return NASDAQ;
            }
            "TMF" => {
                return TMF;
            }
            "OILK" | "Oil" => {
                return OILK;
            }
            "KT&G" => return KT_G,

            _ => {
                panic!("err line 44, str : {:?}", str);
            }
        }
    }

    // pub fn From_UTF8(byte : &[u8]) -> Self {
    //     match byte {
    //         &[75, 84, 95, 71] => {
    //             return KT_G;
    //         }
    //     }
    // }

    pub fn To_String(&self) -> String {
        match self {
            BTC => {
                return String::from("BTC");
            }
            NASDAQ => {
                return String::from("Nasdaq");
            }
            TMF => {
                return String::from("TMF");
            }
            OILK => {
                return String::from("OIL");
            }
            KT_G => {
                return String::from("KT&G");
            }
        }
    }

    fn to_NaverStockTicker(&self) -> &str {
        match self {
            BTC => {
                return "IBIT.O";
            }
            NASDAQ => {
                return "QQQ.O";
            }
            TMF => {
                return "TMF";
            }
            OILK => {
                return "OILK.K";
            }
            KT_G => {
                return "033780";
            }
        }
    }

    fn get_sNaverDomestic(&self) -> &str {
        match self {
            KT_G => return "domestic",

            BTC | NASDAQ | TMF | OILK => {
                return "foreign";
            }
        }
    }
}
