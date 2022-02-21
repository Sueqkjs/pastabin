use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use std::fs;

pub async fn app_page() -> impl Responder {
  HttpResponse::Ok().body(fs::read_to_string("./static/index.html").unwrap())
}

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Found()
    .header("Location", "/index.html")
    .body("")
}

#[get("/assets/{filename:.*}")]
pub async fn statics(req: HttpRequest) -> impl Responder {
  let path = req.match_info().query("filename");
  NamedFile::open(format!("./static/assets/{}", path).parse::<std::path::PathBuf>()?)?
    .respond_to(&req)
    .await
}
