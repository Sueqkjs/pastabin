import { JSDOM } from "jsdom";

export default async function () {
  const dom = new JSDOM(
    `<!doctype html>
    <html>
      <head>
        <meta name="viewport" content="width=device-width">
        <script type="module" src="https://unpkg.com/ionicons@5.0.0/dist/ionicons/ionicons.esm.js"></script>
        <script nomodule="" src="https://unpkg.com/ionicons@5.0.0/dist/ionicons/ionicons.js"></script>
        <link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.4.0/styles/github.min.css">
        <link rel="stylesheet" href="/app.css"></script>
        <title>PastaBin</title>
      </head>
      <body>
        <div class="navigator"></div>
        <main></main>
        <script src="/app.js"></script>
      </body>
    </html>`
  );
  const { window } = dom;
  const document = window._document;
  return dom;
}
