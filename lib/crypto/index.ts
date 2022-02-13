import init, * as mod from "./pkg/crypto";
import { Buffer } from "buffer";

type _ = Uint8Array;

await init();

export const encrypt = (key: _, nonce: _, plaintext: _): Uint8Array => mod.encrypt(key, nonce, plaintext);
export const decrypt = (key: _, nonce: _, ciphertext: _): Uint8Array => mod.decrypt(key, nonce, ciphertext);

export function toPlain(s: string | _) {
  return Buffer.from(s).toString("utf-8");
}

export function toHex(s: Uint8Array) {
  return Buffer.from(s).toString("hex");
}

export function toU8(s: string) {
  return Uint8Array.from(Buffer.from(s, "hex"));
}
