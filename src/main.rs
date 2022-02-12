mod routes;
use actix_redis::RedisActor;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().expect("failed load .env. please check working directory.");
  let bind = std::env::var("bind").unwrap();
  let workers = std::env::var("workers").unwrap().parse::<usize>().unwrap();
  HttpServer::new(|| {
    let redis_addr = RedisActor::start(std::env::var("redis_addr").unwrap());
    App::new()
      .data(redis_addr)
      .service(routes::statics)
      .service(routes::index)
      .service(routes::index_html)
      .service(routes::create)
      .service(routes::pasta)
      .service(routes::status)
      .service(routes::get_pasta)
  })
  .bind(bind)?
  .workers(workers)
  .run()
  .await
}
