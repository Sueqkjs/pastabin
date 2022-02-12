use actix_files as fs;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  dotenv().expect("failed load .env. please check working directory.");
  let bind = std::env::var("bind").unwrap();
  let workers = std::env::var("workers").unwrap().parse::<usize>().unwrap();
  HttpServer::new(||
    App::new().service(
      fs::Files::new("/static", "./static")
    )
  )
  .bind(bind)?
  .workers(workers)
  .run()
  .await
}