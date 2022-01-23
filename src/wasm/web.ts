import * as wasm from "./web_pkg/wasm";
import core from "./core";
import rand from "../utils/randomBytesWeb";
wasm.default("/wasm_bg.wasm");

const mod = core(wasm, rand);

export const encrypt = mod.encrypt;
export const decrypt = mod.decrypt;
export default mod;
