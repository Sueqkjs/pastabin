import { Buffer } from "buffer";

type _ = number[] | Buffer | Uint8Array;

export default function (
  wasm,
  rand: (size: number) => Promise<Uint8Array | Buffer>
) {
  return {
    async encrypt(message: string | _, key?: _, nonce?: _) {
      key = toU8(await rand(32));
      nonce = toU8(await rand(12));
      message = toU8(message);
      return {
        cipherText: toHex(wasm.encrypt(key, nonce, message)),
        key: toHex(key),
        nonce: toHex(nonce),
      };
    },

    async decrypt(message: string | _, key: string | _, nonce: string | _) {
      key = toU8(key, true);
      nonce = toU8(nonce, true);
      message = toU8(message, true);
      return {
        plainText: toPlain(wasm.decrypt(key, nonce, message)),
      };
    },
  };
}

function toU8(s: string | _, hex?: boolean) {
  if (Buffer.isBuffer(s)) {
    return Uint8Array.from(s);
  } else if (s instanceof Uint8Array) {
    return s;
  } else if (typeof s === "string") {
    return Uint8Array.from(Buffer.from(s, hex ? "hex" : null));
  }
}

function toHex(s: string | _) {
  return Buffer.from(s).toString("hex");
}

function toPlain(s: string | _) {
  return Buffer.from(s).toString("utf-8");
}
