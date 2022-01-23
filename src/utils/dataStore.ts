import ProxyDeep from "proxy-deep";
import fs from "node:fs/promises";

export default async function (path: string, _: Object | any[] = {}) {
  const write = async (v) =>
    await fs.writeFile(path, JSON.stringify(v, null, 2));
  const data = await fs
    .readFile(path, "utf-8")
    .then(JSON.parse)
    .catch(async () => {
      await write(_);
      return _;
    });
  return new ProxyDeep(data, {
    get(t, k, r) {
      const v = Reflect.get(t, k, r);
      if (typeof v === "object" && v !== null) return this.nest(v);
      return v;
    },
    set(t, k, v, r) {
      Reflect.set(t, k, v, r);
      write(this.rootTarget);
      return true;
    },
  });
}
