// @ts-ignore
import App from "./index.svelte";
// @ts-ignore
import ErrorPage from "./error.svelte";
// @ts-ignore
import Navigator from "./navigator.svelte";
// @ts-ignore
import Pasta from "./pasta.svelte";
// @ts-ignore
import Create from "./create.svelte";

const path = location.pathname.slice(1).split("/");
const args = {
  target: document.body,
};

new Navigator(args);

if (path[0] === "index.html" || !path[0]) {
  new App(args);
} else if (path[0] === "status") {
  new ErrorPage(args);
} else if (path[0] === "pasta") {
  new Pasta(args);
} else if(path[0] === "create") {
  new Create(args);
}
