import express from "express";
import dotenv from "dotenv";
import dataStore from "./utils/dataStore";
import routes from "./routes";
import { join } from "node:path";
dotenv.config();
const app: express.Application = express();

(async () => {
  global.pastas = (await dataStore(
    process.env.pastasPath,
    []
  )) as typeof global.pastas;

  for (let modName of Object.keys(routes)) {
    const mod = routes[modName];
    app.use(mod.path, mod.default);
  }

  app.use("/", express.static(join(__dirname, "app")));
  app.use(express.json());

  if (process.env.dev)
    app.use(async (_, res, next) => {
      res.set("Cache-Control", [
        "no-store, no-cache, must-revalidate, max-age=0",
        "post-check=0, pre-check=0",
      ]);
      res.set("Pragma", "no-cache");
      next();
    });
  app.all("*", async (_, res) => res.redirect("/status/404"));
  app.listen(process.env.port, () => console.log("ready!"));
})();
