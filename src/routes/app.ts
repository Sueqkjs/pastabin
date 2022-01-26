import { Router } from "express";
import html from "../html";

const router = Router();

router.get("/", async (req, res) => {
  res.redirect("/index.html");
});

router.get("/index.html", async (_, res) => {
  res.end(html);
});

router.get("/pasta/:id", async (_, res) => res.end(html));

router.get("/create", async (_, res) => res.end(html));

export default router;
export const path = "/";
