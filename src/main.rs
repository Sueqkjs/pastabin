mod routes;
use actix_redis::RedisActor;
use actix_web::{
  http::ContentEncoding,
  middleware::{Compress, Logger},
  web, App, HttpServer,
};
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
      .service(routes::get_pasta)
      .service(routes::create_post)
      .route("/index.html", web::get().to(routes::app_page))
      .route("/create", web::get().to(routes::app_page))
      .route("/pasta/{id}", web::get().to(routes::app_page))
      .route("/status/{code}", web::get().to(routes::app_page))
  })
  .bind(bind)?
  .workers(workers)
  .run()
  .await
}
