import Home from "./Index.svelte";
import Create from "./Create.svelte";
import Pasta from "./Pasta.svelte";
import Status from "./Status.svelte";

export default [
  {
    path: "/index.html",
    component: Home,
  },
  {
    path: "/create",
    component: Create,
  },
  {
    path: "/pasta/:id",
    component: Pasta,
  },
  {
    path: "/status/:code",
    component: Status,
  },
];
