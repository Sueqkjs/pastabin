import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte"
import preprocess from "svelte-preprocess"

export default defineConfig({
  plugins: [
    svelte({
      preprocess: preprocess({
        sourceMap: true,
        preserve: [ "module" ]
      })
    })
  ],
  build: {
    /*rollupOptions: {
      input: "src/app.ts",
    },*/
    target: ["esnext"],
    sourcemap: true,
    outDir: "../../static",
  }
}) 
