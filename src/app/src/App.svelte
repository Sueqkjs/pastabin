<script lang="ts">
  import { HouseFill } from "framework7-icons/svelte";
  import { Navbar, Page as F7Page } from "framework7-svelte";
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
  <style lang="scss">
    @import "highlight.js/scss/github-dark.scss";
    HouseFill {
      font-size: 2em;
    }
  </style>
  <title>PastaBin</title>
</svelte:head>

<F7Page>
<Navbar title="PastaBin">
<HouseFill />
</Navbar>

<svelte:component this={Page} />
</F7Page>