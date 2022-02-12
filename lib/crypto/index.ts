import init, * as mod from "./pkg/wasm";
await init();

type Data = string | Uint8Array;

export const decrypt = (key: Data, nonce: Data, ciphertext: Data) => {
  return mod.decrypt(toU8(key), toU8(nonce), toU8(ciphertext));
}
export const encrypt = (key: Data, nonce: Data, plaintext: Data) => {
  return mod.encrypt(toU8(key), toU8(nonce), toU8(plaintext));
}

function toU8(arg: Data): Uint8Array {
  if (arg instanceof String) return Uint8Array.from(Array.from(arg).map(letter => letter.charCodeAt(0)));
  else if (arg instanceof Uint8Array) return arg;
}