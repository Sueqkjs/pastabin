export default async function (size: number = 64) {
  let array = new Uint8Array(size);
  crypto.getRandomValues(array);
  return array;
}
