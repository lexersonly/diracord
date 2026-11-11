/**
 * Finding the `diracord-engine` binary.
 * @module
 */

import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const EXE = process.platform === "win32" ? "diracord-engine.exe" : "diracord-engine";

/**
 * Where the engine binary is, in this order:
 * 1. `DIRACORD_ENGINE` environment variable;
 * 2. `target/release` or `target/debug` in this repository or a parent of `from`.
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
      for (const profile of ["release", "debug"]) {
        const p = join(dir, "target", profile, EXE);
        if (existsSync(p)) return p;
      }
      const up = dirname(dir);
      if (up === dir) break;
      dir = up;
    }
  }
  return null;
}
