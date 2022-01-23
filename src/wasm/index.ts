import * as wasm from "./node_pkg/wasm";
import core from "./core";
import randomBytes from "../utils/randomBytes";

const mod = core(wasm, randomBytes);

export const encrypt = mod.encrypt;
export const decrypt = mod.decrypt;
