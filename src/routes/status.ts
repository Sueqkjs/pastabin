import { Router } from "express";
import { STATUS_CODES as codes } from "statuses";
import html from "../html";

const router = Router();

router.all("/:code", async (req, res) => {
  if (!req.params.code) res.redirect("/status/404");
  if (!codes[req.params.code]) res.redirect("/status/404");
  if (req.query.json)
    return res
      .status(parseInt(req.params.code))
      .json({ message: codes[req.params.code] });
  res.status(+req.params.code).end(html);
});

export default router;
export const path = "/status";
