import esbuild from "esbuild";
import svelte from "esbuild-svelte";
import _preprocess from "svelte-preprocess";
import gzip from "@luncheon/esbuild-plugin-gzip";
const { sass, typescript } = _preprocess;

await esbuild
  .build({
    entryPoints: ["./app.ts"],
    sourcemap: true,
    bundle: true,
    minify: true,
    outdir: "../../static",
    format: "esm",
    platform: "browser",
    write: false,
    plugins: [
      svelte({
        preprocess: [sass(), typescript()],
      }),
      gzip({
        brotli: false,
      })
    ],
  })
  .catch(console.error);
