import App from "./App.svelte";
import Framework7 from "framework7/lite-bundle";
import Framework7Svelte from "framework7-svelte";
import "framework7/framework7-bundle.css";
import "./style/app.css";
import "highlight.js/scss/github-dark.scss";

Framework7.use(Framework7Svelte);

const app = new App({
  target: document.querySelector("body"),
});

export default app;
