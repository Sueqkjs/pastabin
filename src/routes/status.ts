import { Router } from "express";
import { STATUS_CODES as codes } from "statuses";
import generate from "../html";

const router = Router();

router.all("/:code", async (req, res) => {
  if (!req.params.code) res.redirect("/status/404");
  if (!codes[req.params.code]) res.redirect("/status/404");
  if (req.query.json)
    return res
      .status(parseInt(req.params.code))
      .json({ message: codes[req.params.code] });
  const html = await generate();
  res.status(+req.params.code).end(html.serialize());
});

export default router;
export const path = "/status";
