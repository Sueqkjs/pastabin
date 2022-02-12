use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use js_sys::*;
use console_error_panic_hook;
use aes_gcm::{Aes256Gcm, aead::{Aead, NewAead, generic_array::GenericArray}};

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