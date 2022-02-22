use std::string::FromUtf8Error;
use std::str::FromStr;
use hex::FromHexError;
use std::io::Error as IOError;
use actix_web::{ResponseError, HttpResponse};
use derive_more::Display;
use actix::MailboxError;
use actix_redis::Error as ARError;
use std::num::ParseIntError;

#[derive(Debug, Display)]
pub enum Error {
  FromU8(FromUtf8Error),
  FromHex(FromHexError),
  ParseI64(<i64 as FromStr>::Err),
  ParseString(<String as FromStr>::Err),
  IOError(IOError),
  MailboxError(MailboxError),
  RedisError(ARError)
}

impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    match self {
      Error::FromU8(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
      Error::FromHex(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
      Error::ParseI64(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
      Error::ParseString(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
      Error::IOError(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
      Error::MailboxError(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
      Error::RedisError(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
  }
}

impl From<IOError> for Error {
  fn from(e: IOError) -> Self {
    Error::IOError(e)
  }
}

impl From<ParseIntError> for Error {
  fn from(e: ParseIntError) -> Self {
    Error::ParseI64(e)
  }
}


impl From<FromHexError> for Error {
  fn from(e: FromHexError) -> Self {
    Error::FromHex(e)
  }
}


impl From<MailboxError> for Error {
  fn from(e: MailboxError) -> Self {
    Error::MailboxError(e)
  }
}

impl From<ARError> for Error {
  fn from(e: ARError) -> Self {
    Error::RedisError(e)
  }
}

impl From<std::convert::Infallible> for Error {
  fn from(val: std::convert::Infallible) -> Self {
    match val {}
  }
}