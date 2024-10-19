#![allow(non_snake_case)]

use crate::_Import::*;

use actix_files::Files;
use actix_web::{get, App, HttpServer, Responder};

use _StockApis::LocalApi as StockApi;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};

#[derive(serde::Deserialize)]
struct StockQuery {
    stock: String,
}

#[get("/stock/link")]
async fn api_stockLink(query: actix_web::web::Query<StockQuery>) -> impl Responder {
    let sStock = &query.stock;
    let stock = EStock::From_String(sStock);
    return stock.Get_ChartUrl_All_Json().to_string();
}

#[get("/stock/price")]
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

#[get("/weathers")]
async fn api_weathers() -> impl Responder {
    let mut api = WeatherLocalApi::new("database.db");
    return api.Get_Weathers(EArea::서울시_강남구).await;
}

#[get("/news")]
async fn api_news() -> impl Responder {
    let mut api = ArticleLocalApi::new("database.db");
    return api.Get_Response().await;
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
            // get["/"]
            .service(Files::new("/", "./static/page/index").index_file("index.html"))

        // .service(page_index)
    })
    .bind(("127.0.0.1", 40000))?
    .run()
    .await
}
