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
pub async fn statics(req: HttpRequest) -> Result<NamedFile> {
  let path = req.match_info().query("filename");
  let mut ext_vec = path.split(".").collect::<Vec<&str>>();
  ext_vec.remove(0);
  let ext: &str = &ext_vec.join(".");
  let mut content_type = TEXT_PLAIN;
  let mut encoding = ContentEncoding::Br;
  if ext == "jgz" {
    content_type = APPLICATION_JAVASCRIPT;
    encoding = ContentEncoding::Gzip;
  } else if ext == "cgz" {
    content_type = TEXT_CSS;
    encoding = ContentEncoding::Gzip;
  } else if ext == "js" {
    content_type = APPLICATION_JAVASCRIPT;
  } else if ext == "css" {
    content_type = TEXT_CSS;
  } else if ext == "wasm" {
    content_type = "application/wasm".parse::<Mime>().unwrap();
  }
  println!("{:?}", encoding);
  Ok(
    NamedFile::open(format!("./static/{}", path).parse::<std::path::PathBuf>()?)?
      .set_content_type(content_type)
      .set_content_encoding(encoding),
  )
}
