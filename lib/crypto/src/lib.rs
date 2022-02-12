use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use js_sys::*;
use console_error_panic_hook;
use aes_gcm::{Error, Aes256Gcm, aead::{Aead, NewAead, generic_array::GenericArray}};
#[cfg(not(target_arch = "wasm32"))]
use rand::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use rand_chacha::ChaCha20Rng;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn encrypt(_key: Uint8Array, _nonce: Uint8Array, _plaintext: Uint8Array) -> JsValue {
  console_error_panic_hook::set_once();
  let key_vec = &_key.to_vec();
  let key = GenericArray::from_slice(key_vec);
  let nonce_vec = &_nonce.to_vec();
  let nonce = GenericArray::from_slice(nonce_vec);
  let plaintext = _plaintext.to_vec();
  let cipher = Aes256Gcm::new(key);
  match cipher.encrypt(nonce, plaintext.as_ref()) {
    Ok(value) => JsValue::from(unsafe { Uint8Array::view(&value) }),
    Err(error) => JsValue::from_str(&error.to_string())
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn decrypt(_key: Uint8Array, _nonce: Uint8Array, _ciphertext: Uint8Array) -> JsValue {
  let key_vec = &_key.to_vec();
  let key = GenericArray::from_slice(key_vec);
  let nonce_vec = &_nonce.to_vec();
  let nonce = GenericArray::from_slice(nonce_vec);
  let ciphertext = _ciphertext.to_vec();
  let cipher = Aes256Gcm::new(key);
  match cipher.decrypt(nonce, ciphertext.as_ref()) {
    Ok(value) => JsValue::from(unsafe { Uint8Array::view(&value) }),
    Err(error) => JsValue::from_str(&error.to_string())
  }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn decrypt(_key: Vec<u8>, _nonce: Vec<u8>, ciphertext: Vec<u8>) -> Result<Vec<u8>, Error> {
  let key = GenericArray::from_slice(_key.as_slice());
  let nonce = GenericArray::from_slice(_nonce.as_slice());
  let cipher = Aes256Gcm::new(key);
  Ok(cipher.decrypt(nonce, ciphertext.as_ref())?)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn encrypt(_key: Vec<u8>, _nonce: Vec<u8>, plaintext: Vec<u8>) -> Result<Vec<u8>, Error> {
  let key = GenericArray::from_slice(_key.as_slice());
  let nonce_vec = &_nonce.to_vec();
  let nonce = GenericArray::from_slice(nonce_vec);
  let cipher = Aes256Gcm::new(key);
  Ok(cipher.encrypt(nonce, plaintext.as_ref())?)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn rand(bit: usize) -> Vec<u8> {
  let mut csp_rng = ChaCha20Rng::from_entropy();
  let mut data: Vec<u8> = Vec::with_capacity(bit);
  csp_rng.fill_bytes(&mut data);
  data
}