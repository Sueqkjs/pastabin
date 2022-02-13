<script lang="ts">
  import { onMount } from "svelte";
  import * as aes from "../../../lib/crypto";
  $: resultKey = "";
  $: resultNonce = "";
  $: resultId = "";
  let root;
  let _;

  onMount(() => {
    _ = root.querySelector.bind(root);
  });
  const contentChange = () => {
    const content = _(".wrapper #content");
    let maxC = Math.max(...content.value.split("\n").map((x) => x.length));
    let maxL = content.value.split("\n").length;
    content.rows = maxL;
    content.cols = maxC;
  };
  async function post() {
    let title = _("#title");
    let content = _(".wrapper #content");
    let submit = _("#submit");
    let wrapper = _(".wrapper");
    let r = _("#result");

    let { key, id, nonce } = await (
      await fetch("api/pasta", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          title: title.value,
          content: content.value,
          showPasswordHash: "",
        }),
      })
    )
      .json()
      .catch(alert);
    resultId = id;
    resultKey = key;
    resultNonce = nonce;
    root.removeChild(title);
    root.removeChild(wrapper);
    root.removeChild(submit);
    r.style.display = "block";
  }
</script>

<main bind:this={root}>
  <h1>Boil the pasta</h1>
  <input id="title" placeholder="Title" /> <br />
  <div class="wrapper">
    <textarea on:input={contentChange} id="content" placeholder="Content" />
    <br />
  </div>
  <input id="submit" on:click={post} type="button" value="Boil" />
  <div id="result">
    <p>Id:</p>
    <input readonly class="result" value={resultId} />
    <p>Key:</p>
    <input readonly class="result" value={resultKey} />
    <p>Nonce:</p>
    <input readonly class="result" value={resultNonce} />
    <p>Url(UnSecure):</p>
    <input
      readonly
      class="result"
      value="{location.origin}/pasta/{resultId}?iv={resultNonce}&k={resultKey}"
    />
  </div>
</main>

<style lang="scss">
  #result {
    display: none;
  }
  input,
  textarea {
    color: white;
    background: #222;
    border-radius: 4px;
    border: 2px solid #333;
  }
  .wrapper {
    width: 80%;
    #content {
      width: 100%;
    }
  }
</style>
