use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

static HTML: &str = r#"<!doctype html>
<html>
  <head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  </head>
  <body>
    <script type="module" src="/static/app.js"></script>
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
pub async fn statics(req: HttpRequest) -> impl Responder {
  let path = req.match_info().query("filename");
  NamedFile::open(format!("./static/{}", path).parse::<std::path::PathBuf>()?)?
    .respond_to(&req)
    .await
}
