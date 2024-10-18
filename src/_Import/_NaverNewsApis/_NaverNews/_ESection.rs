use ESection::*;

#[derive(Debug, Clone, Copy)]
pub enum ESection {
    Politic,
    Economic,
    Social,
    Culture,
    IT,
    World,
}

impl ESection {
    pub fn All() -> [ESection; 6] {
        return [Politic, Economic, Social, Culture, IT, World];
    }

    pub fn ToKorean(&self) -> &str {
        match self {
            Politic => return "정치",
            Economic => return "경제",
            Social => return "사회",
            Culture => return "생활/문화",
            IT => return "IT/과학",
            World => return "세계",
        }
    }
}

impl ESection {
    pub fn To_Url(&self) -> &str {
        match self {
            Politic => return "https://news.naver.com/section/100",
            Economic => return "https://news.naver.com/section/101",
            Social => return "https://news.naver.com/section/102",
            Culture => return "https://news.naver.com/section/103",
            World => return "https://news.naver.com/section/104",
            IT => return "https://news.naver.com/section/105",
        }
    }
}
