<script lang="ts">
  import * as aes from "../../../lib/crypto/index";
  import hljs from "highlight.js";
  import { onMount } from "svelte";

  let root;
  $: content = "Loading...";
  $: title = "Loading...";
  $: uploadedTimestamp = 0;
  onMount(async () => {
    const params = new URLSearchParams(location.search.slice(1));
    const key = params.get("k");
    const nonce = params.get("iv");
    const id = location.pathname.slice(1).split("/")[1];
    const res = await (await fetch("/api/pasta/" + id))
      .json()
      .catch(console.error);
    if (!res) return alert("Something went wrong. " + res?.message ?? "");
    content = escapeHTML(res.content);
    title = escapeHTML(res.title);
    uploadedTimestamp = res.uploadedTimestamp;
    if (key && nonce) {
      root.querySelector("#key").value = escapeHTML(key);
      root.querySelector("#nonce").value = escapeHTML(nonce);
      await decrypt();
    }
  });

  async function decrypt() {
    const key = aes.toU8(root.querySelector("#key").value);
    const nonce = aes.toU8(root.querySelector("#nonce").value);
    console.log(content)
    let highlighted = hljs.highlightAuto(
      unescapeHTML(aes.toPlain(aes.decrypt(key, nonce, aes.toU8(content))))
    );
    content = highlighted.value.replace("\n", "<br>");
  }

  function unescapeHTML(html) {
    return html.replace(/&(amp|#x27|#x60|quot|lt|gt);/g, (str) => {
      return {
        "&amp;": "&",
        "&#x27;": "'",
        "&#x60;": "`",
        "&quot;": '"',
        "&lt;": "<",
        "&gt;": ">",
      }[str];
    });
  }

  function escapeHTML(html) {
    return html.replace(/[&'`"<>]/g, (str) => {
      return {
        "&": "&amp;",
        "'": "&#x27;",
        "`": "&#x60;",
        '"': "&quot;",
        "<": "&lt;",
        ">": "&gt;",
      }[str];
    });
  }
</script>

<main>
  <h1>{title}</h1>
  <div bind:this={root} id="pasta">
    <input id="key" placeholder="Encrypt Key" /> <br />
    <input id="nonce" placeholder="Encrypt nonce(iv)" /> <br />
    <input on:click={decrypt} id="decrypt" type="button" value="Decrypt" />
    <br /> <br />
    <div id="code">{@html content}</div>
  </div>
</main>

<style lang="scss">
  input,
  #code {
    color: white;
    background: #222;
    border: 2px solid #333;
    border-radius: 4px;
  }
  #pasta {
    width: 80%;
    #code {
      background: #111;
      font-weight: lighter;
      font-family: "Courier New", Courier, monospace;
      width: 100%;
      overflow: scroll;
      white-space: pre-wrap;
    }
  }
</style>
