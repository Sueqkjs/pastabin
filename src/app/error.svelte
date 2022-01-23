<script lang="ts">
  import { STATUS_CODES as codes } from "statuses";
  const code = location.pathname.slice(1).split("/").pop();
  const message = "Error: " + codes[code];
</script>

<main>
  <h1>{code}</h1>
  <p>{message}</p>
</main>