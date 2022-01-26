<script lang="ts">
  import Create from "./create.svelte";
  import Status from "./error.svelte";
  import Index from "./index.svelte";
  import Navigator from "./navigator.svelte";
  import Pasta from "./pasta.svelte";
  const path = location.pathname.slice(1).split("/");
  let Page;

  switch(path[0]) {
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

<Navigator/>
<Page/>