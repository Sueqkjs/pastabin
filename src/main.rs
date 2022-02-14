mod routes;
use actix_redis::RedisActor;
use actix_web::{App, HttpServer, middleware::{Compress, Logger}, http::ContentEncoding};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  dotenv().expect("failed load .env. please check working directory.");
  let bind = std::env::var("bind").unwrap();
  let workers = std::env::var("workers").unwrap().parse::<usize>().unwrap();
  HttpServer::new(|| {
    let redis_addr = RedisActor::start(std::env::var("redis_addr").unwrap());
    App::new()
      .wrap(Compress::new(ContentEncoding::Br))
      .wrap(Logger::default())
      .data(redis_addr)
      .service(routes::statics)
      .service(routes::index)
      .service(routes::index_html)
      .service(routes::create)
      .service(routes::pasta)
      .service(routes::status)
      .service(routes::get_pasta)
      .service(routes::create_post)
  })
  .bind(bind)?
  .workers(workers)
  .run()
  .await
}
