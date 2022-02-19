<script lang="ts">
  import statuses from "statuses";
  import { BlockTitle, BlockHeader, Block, Page } from "framework7-svelte";
  const code = location.pathname.slice(1).split("/").pop();
  const message = "Status: " + statuses.message[code];
</script>

<Page>
  <Block inset>
    <BlockTitle>{code}</BlockTitle>
    <BlockHeader>{message}</BlockHeader>
  </Block>
</Page>
