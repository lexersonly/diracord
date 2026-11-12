import "server-only";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { cacheLife } from "next/cache";

export * from "./docs-nav";

/** Reads the markdown of a page. Cached, so pages are fully static. */
export async function readDoc(slug: string): Promise<string> {
  "use cache";
  cacheLife("max");
  return readFile(path.join(process.cwd(), "content", "docs", `${slug}.md`), "utf8");
}
