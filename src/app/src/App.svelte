<script lang="ts">
  import Framework7 from "framework7/lite-bundle";
  import Framework7Svelte, { Navbar, Icon } from "framework7-svelte";
  Framework7.use(Framework7Svelte);
  import Create from "./Create.svelte";
  import Status from "./Status.svelte";
  import Index from "./Index.svelte";
  import Pasta from "./Pasta.svelte";
  const path = location.pathname.slice(1).split("/");
  let Page = Index;

  switch (path[0]) {
    case "index.html":
      Page = Index;
      break;
    case "pasta":
      Page = Pasta;
      break;
    case "create":
      Page = Create;
      break;
    case "status":
      Page = Status;
      break;
    default:
      location.replace("/status/404");
  }
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width" />
  <link
    rel="stylesheet"
    href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.4.0/styles/github-dark.min.css"
  />
  <link rel="stylesheet" href="/static/app.css" />
  <title>PastaBin</title>
</svelte:head>

<Navbar>
  <Icon f7="house_fill"></Icon>
</Navbar>

<svelte:component this={Page} />
