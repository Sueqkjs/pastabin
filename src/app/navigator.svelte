<div class="navigator">
  <ul>
    <li>
      <a href="/">
        <ion-icon name="home" />
      </a>
    </li>
    <li>
      <a href="/create">
        <ion-icon name="add" />
      </a>
    </li>
    <li>
      <a href="https://github.com/Sueqkjs/pastabin">
        <ion-icon name="logo-github" />
      </a>
    </li>
  </ul>
</div>

<style lang="scss">
  .navigator {
    a {
      color: white;
    }
    border-bottom: solid;
    ul {
      padding-left: 0;
      text-align: center;
      width: 100%;
      list-style: none;
      li {
        display: inline-block;
        font-size: 2em;
        width: 30%;
        :hover {
          opacity: 50%;
          transition: 0.5s;
        }
      }
    }
  }
</style>
