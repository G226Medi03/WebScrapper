#[derive(Debug)]
pub struct Article {
    title: String,
    imgSrc: String,
    link: String,
}

impl Article {
    pub fn new(title: String, imgSrc: String, link: String) -> Self {
        return Article {
            title: title,
            imgSrc: imgSrc,
            link: link,
        };
    }

    pub fn Into_Json(&self) -> serde_json::Value {
        let json = serde_json::json!({
            "title" : self.title,
            "imgSrc" : self.imgSrc,
            "link" : self.link,
        });

        return json;
    }
}
