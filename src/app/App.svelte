<script lang="ts">
  import Create from "./create.svelte";
  import Status from "./error.svelte";
  import Index from "./index.svelte";
  import Navigator from "./navigator.svelte";
  import Pasta from "./pasta.svelte";
  const path = location.pathname.slice(1).split("/");
  let Page;

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
  <script
    type="module"
    src="https://unpkg.com/ionicons@5.0.0/dist/ionicons/ionicons.esm.js"></script>
  <link
    rel="stylesheet"
    href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.4.0/styles/github.min.css"
  />
  <link rel="stylesheet" href="/app.css" />
  <title>PastaBin</title>
</svelte:head>

<Navigator />
<svelte:component this={Page} />
