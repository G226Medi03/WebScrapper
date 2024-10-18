use EWeather::*;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum EWeather {
    NULL,
    SKC,
    FEW,
    BRK,
    OVC,
    RAIN,
    SNOW,
}

impl EWeather {
    pub fn ToKorean(&self) -> &str {
        match *self {
            NULL => return "-",
            SKC => return "맑음",
            FEW => return "구름조금",
            BRK => return "구름많음",
            OVC => return "흐림",
            RAIN => return "비",
            SNOW => {
                return "눈";
            }
        }
    }
    pub fn FromKorean(str: &str) -> Self {
        match str {
            "-" => return NULL,
            "맑음" => {
                return SKC;
            }
            "구름조금" => {
                return FEW;
            }
            "구름많음" => {
                return BRK;
            }
            "흐림" => {
                return OVC;
            }
            "비" | "한때 비" | "흐리고 비" | "비 조금" | "비 계속" | "구름많고 비"=> {
                return RAIN;
            }
            "눈" | "한때 눈" | "흐리고 눈" | "눈 조금" | "눈 계속" | "구름많고 눈" => return SNOW,
            _ => {
                panic!("err line 51, str : {:#?}", str);
            }
        }
    }
}
