import { Router } from "express";
import html from "../html";

const router = Router();

router.get("/", async (_, res) => {
  res.redirect("/index.html");
});

["index", "pasta/:id", "status/:code", "create"].forEach((v) => {
  router.get(`/${v}(.html)?`, async (_, res) => {
    res.end(html);
  });
});

export default router;
export const path = "/";
