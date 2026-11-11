/**
 * Finding the `diracord-engine` binary.
 * @module
 */

import { existsSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const EXE = process.platform === "win32" ? "diracord-engine.exe" : "diracord-engine";

/**
 * Where the engine binary is, in this order:
 * 1. `DIRACORD_ENGINE` environment variable;
 * 2. `target/release` or `target/debug` in this repository or a parent of `from`
 *    (the most recently built one wins, so a stale release build is not used by mistake).
 *
 * Returns `null` when not found.
 */
export function findEngine(from: string = process.cwd(), env: NodeJS.ProcessEnv = process.env): string | null {
  const fromEnv = env["DIRACORD_ENGINE"];
  if (fromEnv) return existsSync(fromEnv) ? resolve(fromEnv) : null;
  const here = dirname(fileURLToPath(import.meta.url));
  for (const start of [here, resolve(from)]) {
    let dir = start;
    for (;;) {
      const found = ["release", "debug"]
        .map((profile) => join(dir, "target", profile, EXE))
        .filter((p) => existsSync(p))
        .sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs);
      if (found[0]) return found[0];
      const up = dirname(dir);
      if (up === dir) break;
      dir = up;
    }
  }
  return null;
}
