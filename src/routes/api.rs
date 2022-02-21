use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{get, post, web, Error as AWError, HttpRequest, HttpResponse, Result};
use chrono::{DateTime, Utc};
use hex;
use redis_async::{resp::RespValue, resp_array};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
  id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PastaRequest {
  #[serde(alias = "showPassword")]
  show_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCheckResponse {
  passworded: bool,
}

impl Pasta {
  pub fn default() -> Self {
    Self {
      id: "".into(),
      title: "".into(),
      content: "".into(),
      uploaded_timestamp: 0,
      show_password_hash: "".into(),
    }
  }
}

#[post("/api/pasta")]
pub async fn create_post(
  data: web::Json<UserPostedPasta>,
  db: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
  let id = hex::encode(crypto::rand(16));
  let uploaded_timestamp = (Utc::now() as DateTime<Utc>).timestamp_millis();

  db.send(Command(resp_array![
    "HMSET",
    &id,
    "title",
    &data.title,
    "content",
    &data.content,
    "show_password_hash",
    &data.show_password_hash,
    "uploaded_timestamp",
    uploaded_timestamp.to_string()
  ]))
  .await??;
  Ok(HttpResponse::Ok().json(APIResponse { id }))
}

#[post("/api/pasta/{id}")]
pub async fn get_pasta(
  req: HttpRequest,
  data: web::Json<PastaRequest>,
  db: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
  let id = req.match_info().query("id").parse::<String>()?;
  match db
    .send(Command(resp_array![
      "HMGET",
      &id,
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
      let title = &resp[0];
      let content = &resp[1];
      let show_password_hash = &resp[2];
      let uploaded_timestamp = &resp[3].parse::<i64>().unwrap();
      let mut hasher = Sha512::new();
      hasher.update(&data.show_password);
      let requested_show_password_hash = hasher.finalize();
      let hash_ok = hex::decode(&show_password_hash)
        .unwrap()
        .eq(&requested_show_password_hash.to_vec());
      if show_password_hash.len() == 0 || hash_ok {
        Ok(HttpResponse::Ok().json(Pasta {
          id,
          title: title.into(),
          content: content.into(),
          uploaded_timestamp: *uploaded_timestamp,
          show_password_hash: show_password_hash.into(),
        }))
      } else {
        Ok(HttpResponse::Forbidden().json(Pasta::default()))
      }
    }
    _ => Ok(HttpResponse::Ok().body("")),
  }
}

#[get("/api/pasta/{id}")]
pub async fn password_check(
  req: HttpRequest,
  db: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
  let id = req.match_info().query("id").parse::<String>()?;
  match db
    .send(Command(resp_array![
      "HMGET",
      &id,
      "show_password_hash"
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
      if resp[0].len() == 0 {
        Ok(HttpResponse::Ok().json(PasswordCheckResponse { passworded: false }))
      } else {
        Ok(HttpResponse::Ok().json(PasswordCheckResponse { passworded: true }))
      }
    }
    _ => Ok(HttpResponse::NotFound().json(PasswordCheckResponse { passworded: false })),
  }
}
