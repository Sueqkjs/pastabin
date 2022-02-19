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
    f7,
  } from "framework7-svelte";
  import { onMount } from "svelte";

  const params = new URLSearchParams(location.search.slice(1));
  $: key = escapeHTML(params.get("k"));
  $: nonce = escapeHTML(params.get("iv"));
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
      password = await passwordAsync("This Pasta's password", "PastaBin").catch(
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

  function passwordAsync(title: string, text: string): Promise<string> {
    return new Promise((res, rej) => {
      f7.dialog.password(title, text, res, rej);
    });
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

<Page>
  <BlockTitle>{title}</BlockTitle>
  <BlockHeader
    >Uploaded At: {new Intl.DateTimeFormat("en-US", {
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
