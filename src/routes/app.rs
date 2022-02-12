use actix_files::NamedFile;
use actix_web::{get, http::header::ContentEncoding, HttpRequest, HttpResponse, Responder, Result};
use mime::{Mime, APPLICATION_JAVASCRIPT, TEXT_CSS, TEXT_PLAIN};

static HTML: &str = r#"<!doctype html>
<html>
  <head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  </head>
  <body>
    <script defer type="module" src="/static/app.jgz"></script>
  </body>
</html>"#;

#[get("/index.html")]
pub async fn index_html() -> impl Responder {
  HttpResponse::Ok().body(HTML)
}

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Found()
    .header("Location", "/index.html")
    .body(HTML)
}

#[get("/create")]
pub async fn create() -> impl Responder {
  HttpResponse::Ok().body(HTML)
}

#[get("/pasta/{id}")]
pub async fn pasta() -> impl Responder {
  HttpResponse::Ok().body(HTML)
}

#[get("/status/{code}")]
pub async fn status() -> impl Responder {
  HttpResponse::Ok().body(HTML)
}

#[get("/static/{filename:.*}")]
pub async fn statics(req: HttpRequest) -> Result<HttpResponse> {
  let path = req.match_info().query("filename");
  let mut ext_vec = path.split(".").collect::<Vec<&str>>();
  ext_vec.remove(0);
  let ext: &str = &ext_vec.join(".");
  let mut content_type = "text/plain;";
  let mut encoding = "identify";
  if ext == "jgz" {
    content_type = "application/javascript";
    encoding = "gzip";
  } else if ext == "cgz" {
    content_type = "text/css";
    encoding = "gzip";
  } else if ext == "js" {
    content_type = "application/javascript";
  } else if ext == "css" {
    content_type = "text/css";
  } else if ext == "wasm" {
    content_type = "application/wasm";
  }
  println!("{:?}", encoding);
  let mut file = NamedFile::open(format!("./static/{}", path).parse::<std::path::PathBuf>()?)?
  .respond_to(&req).await?;
  Ok(HttpResponse::Ok().header("Content-type", content_type).header("Content-encoding", encoding).streaming(file.take_body()))
}
