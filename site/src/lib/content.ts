import "server-only";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { cacheLife } from "next/cache";
import type { SectionId } from "./nav";

/** Reads `content/<section>/<slug>.md`. Cached, so pages are fully static. */
export async function readContent(section: SectionId, slug: string): Promise<string> {
  "use cache";
  cacheLife("max");
  return readFile(path.join(process.cwd(), "content", section, `${slug}.md`), "utf8");
}
