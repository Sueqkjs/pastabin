import { f7 } from "framework7-svelte";

export function passwordAsync(text: string): Promise<string> {
  return new Promise((res, rej) => {
    f7.dialog.password("PastaBin", text, res, rej);
  });
}

export function alert(text: any) {
  if (typeof text !== "string") text = text+"";
  f7.dialog.alert(text, "PastaBin");
}

export function rand(bit: number): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(bit));
}

export function unescapeHTML(html: string) {
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

export function escapeHTML(html: string) {
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