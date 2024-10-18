
use serde::{Deserialize, Serialize};
use EDay::*;
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EDay {
    Mon = 0,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl EDay {
    pub fn From_SingleStringNum(str: &str) -> Self {
        match str {
            "0" => return Sun,
            "1" => return Mon,
            "2" => return Tue,
            "3" => return Wed,
            "4" => return Thu,
            "5" => return Fri,
            "6" => return Sat,
            _ => {
                panic!("line 40 err str : {:#?}", str);
            }
        }
    }

    pub fn From_SingleNum(num: u64) -> Self {
        match num {
            0 => return Sun,
            1 => return Mon,
            2 => return Tue,
            3 => return Wed,
            4 => return Thu,
            5 => return Fri,
            6 => return Sat,
            _ => {
                panic!("line 42 err str : {:#?}", num);
            }
        }
    }
    pub fn From_SingleKorean(str: &str) -> Self {
        match str {
            "월" => {
                return Mon;
            }
            "화" => {
                return Tue;
            }
            "수" => {
                return Wed;
            }
            "목" => {
                return Thu;
            }
            "금" => {
                return Fri;
            }
            "토" => {
                return Sat;
            }
            "일" => {
                return Sun;
            }
            _ => {
                panic!("line 39 str : {:#?}", str);
            }
        }
    }

    pub fn To_SingleKorean(&self) -> &str {
        match *self {
            Mon => {
                return "월";
            }
            Tue => {
                return "화";
            }
            Wed => {
                return "수";
            }
            Thu => {
                return "목";
            }
            Fri => {
                return "금";
            }
            Sat => {
                return "토";
            }
            Sun => {
                return "일";
            }
        }
    }
}
