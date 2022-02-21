<script lang="ts">
  import { onMount } from "svelte";
  import {
    Page,
    List,
    Block,
    BlockHeader,
    BlockTitle,
    Checkbox,
    ListInput,
    Button,
    f7,
  } from "framework7-svelte";
  import * as aes from "../../../lib/crypto";

  $: resultKey = "";
  $: resultNonce = "";
  $: resultId = "";
  $: title = "";
  $: content = "";
  $: isLoading = false;
  let noPassword = false;
  let root;
  let _;

  onMount(() => {
    _ = root.querySelector.bind(root);
    _("#result").style.display = "none";
  });

  async function post() {
    let showPassword = noPassword
      ? ""
      : await passwordAsync("This Pasta's password", "PastaBin").catch(
          () => ""
        );
    if (showPassword.trim().length === 0)
      f7.dialog.alert("Warning: empty password");
    isLoading = true;
    let result = _("#result");
    let input = _("#input");
    let key = rand(32);
    let nonce = rand(12);
    content = content
      .replaceAll(/<div( class="")?>/g, "\n")
      .replaceAll("</div>", "")
      .replaceAll("&nbsp;", " ");
    let { id } = await (
      await fetch("api/pasta", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          title,
          showPasswordHash: !showPassword
            ? ""
            : aes.toHex(
                new Uint8Array(
                  await crypto.subtle.digest(
                    "SHA-512",
                    aes.toU8(showPassword, true)
                  )
                )
              ),
          content: aes.toHex(aes.encrypt(key, nonce, aes.toU8(content, true))),
        }),
      })
    )
      .json()
      .catch(() => {
        isLoading = false;
      });
    resultId = id;
    resultKey = aes.toHex(key);
    resultNonce = aes.toHex(nonce);
    result.style.display = "block";
    input.style.display = "none";
  }

  function rand(bit: number): Uint8Array {
    return crypto.getRandomValues(new Uint8Array(bit));
  }
  function passwordAsync(title: string, text: string): Promise<string> {
    return new Promise((res, rej) => {
      f7.dialog.password(title, text, res, rej);
    });
  }
</script>

<Page>
  <div bind:this={root}>
    <div id="input">
      <BlockTitle>Boil the pasta</BlockTitle>
      <BlockHeader>Warning: Title value was not encrypted</BlockHeader>
      <BlockHeader
        >No Password (not recommented): <Checkbox
          value={noPassword}
          on:change={() => (noPassword = !noPassword)}
        /></BlockHeader
      >
      <List>
        <ListInput
          type="texteditor"
          label="Title"
          resizable
          placeholder="ex: very very secret source code"
          textEditorParams={{
            el: "",
            mode: "popver",
          }}
          onTextEditorChange={(v) => (title = v)}
        />
        <ListInput
          type="texteditor"
          label="Content"
          placeholder="Enter text...."
          resizable
          textEditorParams={{
            el: "",
            mode: "popover",
          }}
          onTextEditorChange={(v) => (content = v)}
        />
        <Button fill preloader loading={isLoading} onClick={post}>Boil</Button>
      </List>
    </div>
    <Block id="result" inset>
      <BlockTitle>Id</BlockTitle>
      <Block>
        <p>{resultId}</p>
      </Block>
      <BlockTitle>Key</BlockTitle>
      <Block><p>{resultKey}</p></Block>
      <BlockTitle>Nonce</BlockTitle>
      <Block><p>{resultNonce}</p></Block>
      <BlockTitle>Url (Unsecure)</BlockTitle>
      <Block>
        <p id="url">
          {location.origin}/pasta/{resultId}?k={resultKey}&amp;iv={resultNonce}
        </p>
        <Button
          onClick={() => {
            navigator.clipboard
              .writeText(_("#url").innerHTML.replaceAll("&amp;", "&"))
              .then(() => {
                f7.notification
                  .create({
                    icon: '<i class="f7-icons" style="font-size: 16px; width: 16px; height: 16px;">house_fill</i>',
                    title: "PastaBin",
                    titleRightText: "now",
                    subtitle: "Copied!",
                    text: "Been copied the url to your clipboard",
                    closeTimeout: 1500,
                  })
                  .open();
              })
              .catch(alert);
          }}
          fill
        >
          Copy
        </Button>
      </Block>
    </Block>
  </div>
</Page>
