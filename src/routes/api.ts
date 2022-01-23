import express, { Router } from "express";
import uniqueId from "../utils/uniqueId";
import * as aes from "../wasm/index";

const router = Router();

router.use(express.json());

router.get("/", async (_, res) => {
  res.status(201).json(false);
});

router.get("/pasta/:id", async (req, res) => {
  const pasta = pastas.find((x) => x.id === req.params.id);
  if (!pasta) return res.status(404).json({ message: "not found" });
  const { content, title, uploadedTimestamp } = pasta;
  res.status(200).json({ content, title, uploadedTimestamp });
});

router.post("/pasta", async (req, res) => {
  if (!req.body) return res.status(400).json({ message: "body is required" });
  // @ts-ignore
  const {
    title,
    content,
    editPasswordHash,
    showPasswordHash,
    hiddenTimestamp,
  }: {
    title: string;
    content: string;
    editPasswordHash: string;
    showPasswordHash?: string;
    hiddenTimestamp: number;
  } = req.body;
  const encrypted = await aes.encrypt(content);
  const id = (await uniqueId()).toString();
  if (typeof content !== "string" || typeof editPasswordHash !== "string")
    return res.status(400).json({ message: "incorrect type" });
  const pasta = {
    title,
    id,
    content: encrypted.cipherText,
    editPasswordHash,
    showPasswordHash: showPasswordHash ?? "",
    uploadedTimestamp: hiddenTimestamp ? 0 : Date.now(),
  };
  pastas.push(pasta);
  res.json({ id, key: encrypted.key, nonce: encrypted.nonce });
});

router.all("*", async (_, res) => {
  res.status(404).json({ message: "not found" });
});

export default router;
export const path = "/api";
