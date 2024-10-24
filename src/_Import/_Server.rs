#![allow(non_snake_case)]

use std::{
    io::{Read, Write},
    process::Child,
};

use crate::_Import::*;

use actix_files::{Files, NamedFile};
use actix_web::{get, http::header::map::Iter, App, HttpResponse, HttpServer, Responder};

use _StockApis::LocalApi as StockApi;
use chrono::{format, DateTime, Local, NaiveDate, NaiveDateTime};
use reqwest::header::HeaderValue;
use std::process::Command;

#[derive(serde::Deserialize)]
struct StockQuery {
    stock: String,
}

#[get("/api/stock/link")]
async fn api_stockLink(query: actix_web::web::Query<StockQuery>) -> impl Responder {
    let sStock = &query.stock;
    let stock = EStock::From_String(sStock);
    return stock.Get_ChartUrl_All_Json().to_string();
}

#[get("/api/stock/price")]
async fn api_stockPrice(query: actix_web::web::Query<StockQuery>) -> impl Responder {
    fn getDate_20WeeksAgo_Today() -> NaiveDate {
        let now = chrono::offset::Local::now();
        let past = now - chrono::Duration::weeks(20);
        return past.naive_utc().date();
    }

    let startDate = getDate_20WeeksAgo_Today();
    let mut api = StockApi::new("database.db");
    let sStock = &query.stock;
    return api.Get_StockCandles_Json(sStock, startDate).await;
}

#[get("/api/weathers")]
async fn api_weathers() -> impl Responder {
    let mut api = WeatherLocalApi::new("database.db");
    return api.Get_Weathers(EArea::서울시_강남구).await;
}

#[get("/api/news")]
async fn api_news() -> impl Responder {
    let mut api = ArticleLocalApi::new("database.db");
    return api.Get_Response().await;
}

#[derive(serde::Deserialize)]
struct YoutubeAudioQuery {
    url: String,
}

#[get("/youtube/audio")]
async fn api_yotubeAudio(
    req: actix_web::HttpRequest,
    query: actix_web::web::Query<YoutubeAudioQuery>,
) -> impl Responder {
    async fn getTitle() -> String {
        let url = format!("http://127.0.0.1:40001/filename");
        let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();
        if resp == "\"\"" {
            return String::from("Unknown.mp3");
        }
        return resp.replace("\"", "");
    }

    fn removeUnnecessaryInfo(inputUrl: &mut String) {
        if inputUrl.contains("&list=") {
            let splited: &Vec<&str> = &inputUrl.split("&list=").collect();
            *inputUrl = splited[0].to_string();
        }
        if inputUrl.contains("&t=") {
            let splited: &Vec<&str> = &inputUrl.split("&t=").collect();
            *inputUrl = splited[0].to_string();
        }
        if inputUrl.contains("&start_radio=") {
            let splited: &Vec<&str> = &inputUrl.split("&start_radio=").collect();
            *inputUrl = splited[0].to_string();
        }
    }

    let mut inputUrl = query.url.clone();

    removeUnnecessaryInfo(&mut inputUrl);
    let url = format!("http://127.0.0.1:40001/download?url={}", inputUrl);
    let resp = reqwest::get(&url).await.unwrap();

    let title = getTitle().await;
    let path = format!("mp3/{}", title);
    let bytes = resp.bytes().await.unwrap();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&path)
        .expect(&format!("path:{:#?}", path));
    file.write_all(&bytes).unwrap();
    file.sync_all().unwrap();

    let namedFile = NamedFile::from_file(file, &title).unwrap();
    let ret = namedFile
        .use_last_modified(true)
        .set_content_disposition(actix_web::http::header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![actix_web::http::header::DispositionParam::Filename(title)],
        })
        .into_response(&req);
    return ret;
}

// #[get("/")]
// async fn page_index() -> impl Responder {
//     return HttpResponse::build(StatusCode::OK)
//         .content_type("text/html; charset=utf-8")
//         .body(include_str!("../../static/page_index/index.html"));
// }

pub async fn server_run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // css & js files
            .service(Files::new("/static", "./static").show_files_listing())
            .service(api_weathers)
            .service(api_news)
            .service(api_stockPrice)
            .service(api_stockLink)
            .service(api_yotubeAudio)
            // get["/"]
            .service(Files::new("/", "./static").index_file("index.html"))

        // .service(actix_web::web::redirect("/", "/dailyReport"))

        // .service(page_index)
    })
    .bind(("127.0.0.1", 40000))?
    .run()
    .await
}
