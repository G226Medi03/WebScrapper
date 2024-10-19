use actix_web::web::Bytes;
use rusqlite::types::Value;
use EStock::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EStock {
    BTC,
    NASDAQ,
    TMF,
    OILK,
    KT_G,
    Samsung,
}

impl EStock {
    /// startDate and endDate should have format like "20230728"
    pub fn Get_RequestUrl_NaverFinance(&self, startDate: &str, endDate: &str) -> String {
        let url = format!(
            "https://api.stock.naver.com/chart/{}/item/{}/day?startDateTime={}&endDateTime={}",
            self.get_sNaverFinanceDomestic(),
            self.Get_NaverStockTicker(),
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
            "OILK" | "OIL" => {
                return OILK;
            }
            "KT&G" => return KT_G,

            "SAMSUNG" => return Samsung,
            _ => {
                panic!("err line 44, str : {:?}", str);
            }
        }
    }

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
            Samsung => {
                return String::from("Samsung");
            }
        }
    }

    fn Get_NaverStockTicker(&self) -> &str {
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
            Samsung => {
                return "005930";
            }
        }
    }

    fn get_sNaverFinanceDomestic(&self) -> &str {
        match self {
            KT_G | Samsung => return "domestic",

            BTC | NASDAQ | TMF | OILK => {
                return "foreign";
            }
        }
    }

    pub fn Get_ChartUrl_TraidingView(&self) -> &str {
        match self {
            NASDAQ => return "https://kr.tradingview.com/chart/?symbol=NASDAQ%3AQQQ",
            TMF => {
                return "https://www.tradingview.com/chart/?symbol=AMEX%3ATMF";
            }
            BTC => {
                return "https://www.tradingview.com/chart/?symbol=NASDAQ%3AIBIT";
            }
            OILK => {
                return "https://www.tradingview.com/chart/?symbol=AMEX%3AOILK";
            }
            KT_G => {
                return "https://www.tradingview.com/chart/?symbol=KRX%3A033780";
            }
            Samsung => {
                return "https://www.tradingview.com/chart/?symbol=SAMSUNG";
            }
        }
    }

    pub fn Get_ChartUrl_NaverFinance(&self) -> &str {
        match self {
            NASDAQ => return "https://m.stock.naver.com/fchart/foreign/stock/QQQ.O",
            TMF => {
                return "https://m.stock.naver.com/fchart/foreign/stock/TMF";
            }
            BTC => {
                return "https://m.stock.naver.com/fchart/foreign/stock/IBIT.O";
            }
            OILK => {
                return "https://m.stock.naver.com/fchart/foreign/stock/OILK.K";
            }
            KT_G => {
                return "https://m.stock.naver.com/fchart/domestic/stock/033780";
            }
            Samsung => {
                return "https://m.stock.naver.com/fchart/domestic/stock/005930";
            }
        }
    }

    pub fn Get_ChartUrl_InvestingCom(&self) -> &str {
        match self {
            NASDAQ => return "https://kr.investing.com/etfs/powershares-qqqq-chart",
            TMF => {
                return "https://kr.investing.com/etfs/direxion-30-yr-tr.-bull-3x-shrs-chart";
            }
            BTC => {
                return "https://kr.investing.com/etfs/ibit-nasdaq-chart";
            }
            OILK => {
                return "https://kr.investing.com/etfs/proshares-k1-free-crd-oil-strat-chart";
            }
            KT_G => {
                return "https://kr.investing.com/equities/kt-g-corp-chart";
            }
            Samsung => {
                return "https://kr.investing.com/equities/samsung-electronics-co-ltd-chart";
            }
        }
    }

    pub fn Get_ChartUrl_All_Json(&self) -> serde_json::Value {
        let json = serde_json::json!({
            "naverfinance" : self.Get_ChartUrl_NaverFinance(),
            "investing" : self.Get_ChartUrl_InvestingCom(),
            "traidingview" : self.Get_ChartUrl_TraidingView(),
        });

        return json;
    }
}
