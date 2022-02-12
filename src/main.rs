mod routes;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  dotenv().expect("failed load .env. please check working directory.");
  let bind = std::env::var("bind").unwrap();
  let workers = std::env::var("workers").unwrap().parse::<usize>().unwrap();
  HttpServer::new(||
    App::new()
    .service(
      routes::statics
    )
    .service(routes::index)
    .service(routes::index_html)
    .service(routes::create)
    .service(routes::pasta)
    .service(routes::status)
  )
  .bind(bind)?
  .workers(workers)
  .run()
  .await
}