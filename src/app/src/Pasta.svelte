<script lang="ts">
  import * as aes from "../../../lib/crypto/index";
  import hljs from "highlight.js";
  import {
    Page,
    Block,
    BlockTitle,
    BlockHeader,
    List,
    ListInput,
    Button,
  } from "framework7-svelte";
  import { passwordAsync, unescapeHTML, escapeHTML, alert } from "./utils";
  import { onMount } from "svelte";

  const params = new URLSearchParams(location.search.slice(1));
  $: key = params.get("k");
  $: nonce = params.get("iv");
  $: content = "Loading...";
  $: title = "Loading...";
  $: uploadedTimestamp = 0;

  onMount(async () => {
    let password = "";
    const id = location.pathname.slice(1).split("/")[1];
    const passworded = (
      await (await fetch("/api/pasta/" + id)).json().catch(alert)
    )?.passworded;
    if (passworded)
      password = await passwordAsync("This Pasta's password").catch(
        (x) => ""
      );
    const res = await (
      await fetch("/api/pasta/" + id, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          showPassword: password,
        }),
      })
    )
      .json()
      .catch(alert);
    if (!res || (!res.content && !res.title && !res.id))
      return alert("Something went wrong. " + res ?? "");
    content = res.content;
    title = escapeHTML(res.title);
    uploadedTimestamp = res.uploaded_timestamp;
    if (key && nonce) {
      await decrypt();
    }
  });

  async function decrypt() {
    const _key = aes.toU8(key);
    const _nonce = aes.toU8(nonce);
    content = hljs
      .highlightAuto(
        unescapeHTML(aes.toPlain(aes.decrypt(_key, _nonce, aes.toU8(content))))
      )
      .value.replaceAll("\n", "<br>");
  }
</script>

<Page>
  <BlockTitle>{title}</BlockTitle>
  <BlockHeader
    >Uploaded At: {new Intl.DateTimeFormat("en-US", {
      // @ts-ignore
      dateStyle: "full",
      timeStyle: "long",
    }).format(uploadedTimestamp)}</BlockHeader
  >
  <List>
    <ListInput
      type="texteditor"
      label="Key"
      placeholder="Decrypt Key"
      resizable
      textEditorParams={{
        mode: "popover",
        el: "",
      }}
      value={key}
      onTextEditorChange={(value) => (key = value)}
    />
    <ListInput
      type="texteditor"
      label="Nonce"
      placeholder="Decrypt Nonce"
      resizable
      textEditorParams={{
        mode: "popover",
        el: "",
      }}
      value={nonce}
      onTextEditorChange={(value) => (nonce = value)}
    />
    <Button fill preloader onClick={decrypt}>Decrypt</Button>
  </List>
  <BlockTitle>Content</BlockTitle>
  <Block inset id="content"><pre>{@html content}</pre></Block>
  <br />
  <br />
</Page>
