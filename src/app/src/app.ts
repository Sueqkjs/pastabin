// @ts-ignore
import App from "./App.svelte";
import Framework7 from "framework7/lite-bundle";
import Framework7Svelte from "framework7-svelte";

Framework7.use(Framework7Svelte);

const app = new App({
  target: document.body,
});

export default app;
