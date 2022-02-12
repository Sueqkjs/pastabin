use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{get, post, web, Error as AWError, HttpRequest, HttpResponse, Result};
use chrono::{DateTime, Utc};
use hex;
use redis_async::{resp::RespValue, resp_array};
use serde::{Deserialize, Serialize};
extern crate crypto;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPostedPasta {
  pub title: String,
  pub content: String,
  #[serde(alias = "showPasswordHash")]
  pub show_password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pasta {
  pub title: String,
  pub id: String,
  pub content: String,
  #[serde(alias = "showPasswordHash")]
  pub show_password_hash: String,
  #[serde(alias = "uploadedTimestamp")]
  pub uploaded_timestamp: i64,
}

#[post("/api/pasta")]
pub async fn create_post(
  data: web::Json<UserPostedPasta>,
  db: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
  let key = crypto::rand(32);
  let nonce = crypto::rand(12);
  let plaintext = data.content.clone().into_bytes();
  let content = hex::encode(crypto::encrypt(key, nonce, plaintext).unwrap().as_slice());
  let id = hex::encode(crypto::rand(16));
  let dt: DateTime<Utc> = Utc::now();
  let uploaded_timestamp = dt.timestamp_millis();
  db.send(Command(resp_array![
    "HMSET",
    id.clone(),
    "title",
    data.title.clone(),
    "content",
    content.clone(),
    "show_password_hash",
    data.show_password_hash.clone(),
    "uploaded_timestamp",
    uploaded_timestamp.to_string()
  ]))
  .await??;
  Ok(HttpResponse::Ok().json(Pasta {
    title: data.title.clone(),
    content,
    uploaded_timestamp,
    id,
    show_password_hash: data.show_password_hash.clone(),
  }))
}

#[get("/api/pasta/{id}")]
pub async fn get_pasta(
  req: HttpRequest,
  db: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
  let id = req.match_info().query("id").parse::<String>()?;
  match db
    .send(Command(resp_array![
      "HMGET",
      id.clone(),
      "title",
      "content",
      "show_password_hash",
      "uploaded_timestamp"
    ]))
    .await??
  {
    RespValue::Array(pasta) => {
      let resp = pasta
        .iter()
        .map(|x| match x {
          RespValue::BulkString(y) => String::from_utf8(y.to_vec()).unwrap(),
          _ => "".to_string(),
        })
        .collect::<Vec<String>>();
      let title = resp[0].clone();
      let content = resp[1].clone();
      let show_password_hash = resp[2].clone();
      let uploaded_timestamp = resp[3].clone().parse::<i64>().unwrap();
      Ok(HttpResponse::Ok().json(Pasta {
        id,
        title,
        content,
        uploaded_timestamp,
        show_password_hash,
      }))
    }
    _ => Ok(HttpResponse::Ok().body("")),
  }
}
