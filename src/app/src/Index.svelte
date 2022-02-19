<script lang="ts">
  import { Page } from "framework7-svelte";
</script>

<Page class="page-home">
  <h1>PastaBin</h1>
</Page>

<style lang="scss">
  $textColor: white;
  $backgroundColor: #222;
  :root {
    font-family: Arial, Helvetica, sans-serif;
    color: $textColor;
    background: $backgroundColor;
  }
  h1 {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }
</style>
