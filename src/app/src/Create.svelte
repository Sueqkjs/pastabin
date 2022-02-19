<script lang="ts">
  import { onMount } from "svelte";
  import { Page, List, BlockHeader, BlockTitle, ListInput, Button } from "framework7-svelte";
  import * as aes from "../../../lib/crypto";

  $: resultKey = "";
  $: resultNonce = "";
  $: resultId = "";
  $: title = "";
  $: content = "";
  let root;
  let _;
  let isLoading = false;

  onMount(() => {
    _ = root.querySelector.bind(root);
    setInterval(() => {
      console.log(title, content)
    }, 1500)
  });

  async function post() {
    isLoading = true;
    let r = _("#result");
    let key = rand(32);
    let nonce = rand(12);
    content = content.replaceAll("<div>", "\n").replaceAll("</div>", "").replaceAll("&nbsp;", " ");

    let { id } = await (
      await fetch("api/pasta", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          title,
          content: aes.toHex(aes.encrypt(key, nonce, aes.toU8(content, true))),
          showPasswordHash: "",
        }),
      })
    )
      .json()
      .catch((e) => {
        isLoading = false;
      });
    resultId = id;
    resultKey = aes.toHex(key);
    resultNonce = aes.toHex(nonce);
    r.style.display = "block";
  }

  function rand(bit: number): Uint8Array {
    return crypto.getRandomValues(new Uint8Array(bit));
  }
</script>

<Page>
  <div bind:this={root}>
    <BlockTitle>Boil the pasta</BlockTitle>
    <BlockHeader>Warning: Title value was not encrypted</BlockHeader>
    <List>
      <ListInput
        type="texteditor"
        label="Title"
        resizable
        placeholder="ex: very very secret source code"
        textEditorParams={{
          el: "",
          mode: "popver"
        }}
        onTextEditorChange={(v) => title = v}
      />
      <ListInput
        type="texteditor"
        label="Content"
        placeholder="Enter text...."
        resizable
        textEditorParams={{
          el: "",
          mode: "popover"
        }}
        onTextEditorChange={(v) => content = v}
      />
      <Button fill preloader loading={isLoading} onClick={post}>Boil</Button>
    </List>
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
  </div>
</Page>

<style lang="scss">
  #result {
    display: none;
  }
</style>
