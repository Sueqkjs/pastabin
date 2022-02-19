<script lang="ts">
  import {
    App,
    View,
    Link,
    Navbar,
    NavRight,
    NavLeft,
    NavTitle,
    Icon,
    RouterContextProvider,
  } from "framework7-svelte";
  import routes from "./routes";

  const f7Params = {
    id: "net.sueqk.pastabin",
    theme: "ios",
    autoDarkTheme: true,
    routes,
    popup: {
      closeOnEscape: true,
    },
    sheet: {
      closeOnEscape: true,
    },
    popover: {
      closeOnEscape: true,
    },
    actions: {
      closeOnEscape: true,
    },
  };

  function mainInit(view) {
    let url = "";
    setInterval(() => {
      if (url === view.router.url) return;
      url = view.router.url;
      history.pushState({}, "", view.router.url);
    }, 100);
  }
</script>

<svelte:head>
  <meta name="viewport" content="width=device-width" />
  <title>PastaBin</title>
</svelte:head>

<App {...f7Params}>
  <Navbar>
    <NavLeft>
      <NavTitle>PastaBin</NavTitle>
    </NavLeft>
    <NavRight>
      <Link
        href="/index.html"
        reloadCurrent
        iconF7="house_fill"
        iconSize="40px"
      />
      <Link href="/create" reloadCurrent iconF7="plus" iconSize="40px" />
      <Link
        href="https://github.com/Sueqkjs/pastabin"
        external
        iconF7="logo_github"
        iconSize="40px"
      />
    </NavRight>
  </Navbar>
  <View main onViewInit={mainInit} />
</App>
