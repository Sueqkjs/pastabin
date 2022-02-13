<script lang="ts">
  export let name = "logo-apple";
  export let href = "/";
</script>

<div class="wrapper">
  <li>
    <a {href}>
      <ion-icon {name} />
    </a>
  </li>
</div>

<style lang="scss">
  .wrapper {
    display: inline-block;
    width: 30%;
    a {
      color: white;
    }
    li {
      display: inline-block;
      text-align: center;
      font-size: 2em;
      width: 100%;
      :hover {
        opacity: 50%;
        transition: 0.5s;
      }
    }
  }
</style>
