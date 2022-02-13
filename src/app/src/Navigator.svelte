<script lang="ts">
  import NaviButton from "./NaviButton.svelte";
</script>

<div class="navigator">
  <ul>
    <NaviButton name="home" href="/" />
    <NaviButton name="add" href="/create" />
    <NaviButton name="logo-github" href="https://github.com/Sueqkjs/pastabin" />
  </ul>
</div>

<style lang="scss">
  .navigator {
    ul {
      padding-left: 0;
      text-align: center;
      width: 100%;
      list-style: none;
    }
    border-bottom: solid;
  }
</style>
