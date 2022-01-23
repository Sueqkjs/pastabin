import randomBytes from "./randomBytes";

export default async function (size: number = 6) {
  const buffer = await randomBytes(size);
  const timestamp = Math.floor(Date.now() / 100_000_000);
  return [
    ...BigInt("0x" + timestamp.toString(16) + buffer.toString("hex")).toString(
      16
    ),
  ]
    .reverse()
    .join("");
}
