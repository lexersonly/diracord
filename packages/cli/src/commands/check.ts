/**
 * `diracord check [--fix]`: shows every problem in the project, with the
 * code excerpt, an arrow and a suggestion. `--fix` applies safe fixes.
 * @module
 */

import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { loadProject } from "@diracord/config";
import { EngineClient } from "@diracord/engine";
import { green, out, useColor } from "../ui.ts";

/** Returns the process exit code: 0 clean, 1 errors found. */
export async function check(dir: string, opts: { fix?: boolean } = {}): Promise<number> {
  const project = loadProject(dir);
  const engine = await EngineClient.start({ cwd: project.root });
  try {
    const dollar = project.config.syntax.dollar;
    let files = project.files;
    if (opts.fix) {
      const fixed = await engine.fix(files, { dollar });
      for (const f of fixed.files) {
        if (!f.changed) continue;
        writeFileSync(join(project.root, f.path), f.source);
        out(`${green("corrigido")}: ${f.path}`);
      }
      files = fixed.files.map(({ path, source }) => ({ path, source }));
    }
    const r = await engine.check(files, { dollar, color: useColor });
    if (r.diagnostics.length === 0) {
      out(`${green("ok")}: ${files.length} arquivo(s), nenhum problema encontrado`);
      return 0;
    }
    out(r.rendered.trimEnd());
    const fixable = r.diagnostics.some((d) => d.fixes.some((f) => f.safe));
    if (fixable && !opts.fix) out(`\ndica: rode \`diracord check --fix\` para aplicar as correções automáticas seguras`);
    return r.ok ? 0 : 1;
  } finally {
    await engine.close();
  }
}
