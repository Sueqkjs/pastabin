import esbuild from "esbuild";
import svelte from "esbuild-svelte";
import _preprocess from "svelte-preprocess";
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
    plugins: [
      svelte({
        preprocess: [sass(), typescript()],
      }),
    ],
  })
  .catch(console.error);
