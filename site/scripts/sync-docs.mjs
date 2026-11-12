// Copies the generated references from the repository into the site:
//   ../docs/functions.md → content/docs/reference-functions.md
//   ../docs/errors.md    → content/docs/reference-errors.md
//   ../llm.txt           → public/llm.txt
// The copies are committed, so the site also builds where ../ is not available.
// A test in crates/diracord-engine/tests/docs_sync.rs checks they are up to date.
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const site = join(dirname(fileURLToPath(import.meta.url)), "..");
const repo = join(site, "..");

/** Drops the "# Title" line and the "Gerado por" note: the site shows its own header. */
function body(markdown) {
  let open = false;
  return markdown
    .split("\n")
    .filter((l, i) => !(i === 0 && l.startsWith("# ")))
    .filter((l) => !l.startsWith("> Gerado por"))
    .map((l) => {
      if (!l.startsWith("```")) return l;
      open = !open;
      // Snippets like `reply typeOf(x)` are not whole programs: highlight them,
      // but as "diracord" so the docs test does not check them as programs.
      return open && l === "```" ? "```diracord" : l;
    })
    .join("\n")
    .replace(/^\n+/, "");
}

const copies = [
  ["docs/functions.md", "content/docs/reference-functions.md", body],
  ["docs/errors.md", "content/docs/reference-errors.md", body],
  ["llm.txt", "public/llm.txt", (s) => s],
];

if (!existsSync(join(repo, "docs", "functions.md"))) {
  console.log("sync-docs: repositório não encontrado, usando as cópias salvas");
  process.exit(0);
}
for (const [from, to, transform] of copies) {
  writeFileSync(join(site, to), transform(readFileSync(join(repo, from), "utf8")));
  console.log(`sync-docs: ${from} → ${to}`);
}
