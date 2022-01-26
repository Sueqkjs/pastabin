
import esbuild from "esbuild";
import svelte from "esbuild-svelte";
import _preprocess from "svelte-preprocess";
import fs from "node:fs/promises";
import path from "node:path";

const { sass, typescript } = _preprocess;
const __dirname = path.dirname(import.meta.url).replace(/^file:\/\//, "");
const jsdomPatch = {
  name: "jsdomPatch",
  setup(build) {
    build.onLoad(
      { filter: /jsdom\/living\/xhr\/XMLHttpRequest-impl\.js$/ },
      async (args) => {
        let contents = await fs.readFile(args.path, "utf-8");
        contents = contents.replace(
          'const syncWorkerFile = require.resolve ? require.resolve("./xhr-sync-worker.js") : null;',
          `const syncWorkerFile = "${path.join(
            __dirname,
            "node_modules",
            "jsdom/lib/jsdom/living/xhr-sync-worker.js"
          )}";`
        );
        return { contents, loader: "js" };
      }
    );
  },
};

await fs
  .readdir("src/routes")
  .then((files) => {
    let code = "";
    let exports = [];
    for (let file of files) {
      if (file.endsWith("d.ts")) continue;
      if (file === "index.ts") continue;
      const name = file.replace(".ts", "");
      const path = JSON.stringify("./" + name);
      code += "import * as " + name + " from " + path + ";\n";
      exports.push(name);
    }
    code += "export default {\n" + exports.map((v) => "  " + v).join(",\n") + "\n};";
    fs.writeFile("src/routes/index.ts", code);
  })
  .catch(console.error);

await esbuild
  .build({
    entryPoints: ["./src/app/app.js"],
    sourcemap: true,
    bundle: true,
    minify: true,
    outdir: "./dist/app",
    plugins: [
      svelte({
        preprocess: [sass(), typescript()],
      }),
    ],
  })
  .catch(console.error);

await esbuild
  .build({
    entryPoints: ["./src/index.ts"],
    external: ["canvas"],
    sourcemap: true,
    outfile: "./dist/index.cjs",
    bundle: true,
    minify: true,
    platform: "node",
    target: "node17",
    format: "cjs",
    plugins: [jsdomPatch],
  })
  .catch(console.error);

await fs.copyFile(
  path.join(__dirname, "src", "wasm", "node_pkg", "wasm_bg.wasm"),
  path.join(__dirname, "dist", "wasm_bg.wasm")
);

await fs.copyFile(
  path.join(__dirname, "src", "wasm", "web_pkg", "wasm_bg.wasm"),
  path.join(__dirname, "dist", "app", "wasm_bg.wasm")
);