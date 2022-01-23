<script defer lang="ts">
  import * as aes from "../wasm/web";
  import hljs from "highlight.js";
  let root;
  $: content = "Loading...";
  $: title = "Loading...";
  $: uploadedTimestamp = 0;
  window.onload = (async() => {
    const params = new URLSearchParams(location.search.slice(1));
    const key = params.get("k");
    const nonce = params.get("iv");
    const id = location.pathname.slice(1).split("/")[1];
    // @ts-ignore
    const res = await (await fetch("/api/pasta/" + id)).json().catch(console.error);
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

  interface IPasta {
    title: string
    content: string,
    uploadedTimestamp: number
  }

  async function decrypt() {
    const key = root.querySelector("#key").value;
    const nonce = root.querySelector("#nonce").value;
    let highlighted = hljs.highlightAuto(unescapeHTML((await aes.decrypt(content, key, nonce)).plainText));
    content = highlighted.value.replace("\n", "<br>");
    console.log(content)
  }

  function unescapeHTML(html) {
    return html.replace(/&(amp|#x27|#x60|quot|lt|gt);/g, (str) =>  {
      return {
        "&amp;": "&",
        "&#x27;": "'",
        "&#x60;": "`",
        "&quot;": "\"",
        "&lt;": "<",
        "&gt;": ">",
      }[str]
    });
  }

  function escapeHTML(html) {
    return html.replace(/[&'`"<>]/g, (str) =>  {
      return {
        "&": "&amp;",
        "'": '&#x27;',
        "`": "&#x60;",
        "\"": "&quot;",
        "<": "&lt;",
        ">": "&gt;",
      }[str]
    });
  }
</script>

<main>
  <h1>{title}</h1>
  <div bind:this={root} id="pasta">
    <input id="key" placeholder="Encrypt Key"> <br>
    <input id="nonce" placeholder="Encrypt nonce(iv)"> <br>
    <input on:click={decrypt} id="decrypt" type="button" value="Decrypt"> <br> <br>
    <div id="code">{@html content}</div>
  </div>
</main>

<style lang="scss">
  input, 
  #code {
    background: #222;
    color: white;
    border: 2px solid #333;
    border-radius: 4px;
  }
  #pasta {
    width: 80%;
    #code {
      background: #111;
      font-weight: lighter;
      font-family: 'Courier New', Courier, monospace;
      width: 100%;
      overflow: scroll;
      white-space: pre-wrap;
    }
  }
</style>