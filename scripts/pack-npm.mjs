#!/usr/bin/env node
// Builds the publishable `diracord` npm package in npm-dist/diracord:
//   dist/bin.js, dist/index.js   TypeScript packages bundled with esbuild (no runtime deps)
//   bin/<platform>-<arch>/       prebuilt diracord-engine binaries
//
// Usage:
//   node scripts/pack-npm.mjs                    # engine for this machine, from target/release
//   node scripts/pack-npm.mjs --engines <dir>    # <dir>/<platform>-<arch>/diracord-engine[.exe]
//
// It never publishes: run `npm publish npm-dist/diracord` yourself (or let the release workflow do it).
import { build } from "esbuild";
import { chmodSync, copyFileSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const out = join(root, "npm-dist", "diracord");
const args = process.argv.slice(2);
const enginesDir = args.includes("--engines") ? args[args.indexOf("--engines") + 1] : null;
const version = JSON.parse(readFileSync(join(root, "packages", "diracord", "package.json"), "utf8")).version;

rmSync(out, { recursive: true, force: true });
mkdirSync(join(out, "dist"), { recursive: true });

// 1. JavaScript bundle.
await build({
  entryPoints: {
    bin: join(root, "packages/diracord/src/bin.ts"),
    index: join(root, "packages/diracord/src/index.ts"),
  },
  outdir: join(out, "dist"),
  bundle: true,
  platform: "node",
  format: "esm",
  target: "node22",
  // The CLI's `if (import.meta.main)` must not run again inside the bundle.
  define: { "import.meta.main": "false" },
  logLevel: "warning",
});
chmodSync(join(out, "dist", "bin.js"), 0o755);

// 2. Engine binaries.
const exe = (key) => (key.startsWith("win32") ? "diracord-engine.exe" : "diracord-engine");
const engines = [];
if (enginesDir) {
  for (const key of readdirSync(enginesDir)) {
    const src = join(enginesDir, key, exe(key));
    if (existsSync(src)) engines.push([key, src]);
  }
} else {
  const key = `${process.platform}-${process.arch}`;
  const src = join(root, "target", "release", exe(key));
  if (!existsSync(src)) throw new Error("rode antes: cargo build --release -p diracord-engine");
  engines.push([key, src]);
}
if (engines.length === 0) throw new Error("nenhum binário do motor encontrado");
for (const [key, src] of engines) {
  mkdirSync(join(out, "bin", key), { recursive: true });
  copyFileSync(src, join(out, "bin", key, exe(key)));
  chmodSync(join(out, "bin", key, exe(key)), 0o755);
}

// 3. package.json, README, LICENSE.
const pkg = {
  name: "diracord",
  version,
  description: "Diracord: crie bots de Discord com uma linguagem simples. Erros que ensinam, prefixo e slash no mesmo código, motor em Rust.",
  keywords: ["discord", "discord-bot", "bot", "language", "aoi.js", "slash-commands", "components-v2"],
  homepage: "https://github.com/lexersonly/diracord#readme",
  repository: { type: "git", url: "git+https://github.com/lexersonly/diracord.git" },
  bugs: { url: "https://github.com/lexersonly/diracord/issues" },
  license: "MIT",
  type: "module",
  bin: { diracord: "dist/bin.js" },
  exports: { ".": "./dist/index.js" },
  files: ["dist", "bin", "README.md", "LICENSE"],
  engines: { node: ">=22.18" },
  os: [...new Set(engines.map(([k]) => k.split("-")[0]))],
  cpu: [...new Set(engines.map(([k]) => k.split("-")[1]))],
};
writeFileSync(join(out, "package.json"), `${JSON.stringify(pkg, null, 2)}\n`);
copyFileSync(join(root, "packages", "diracord", "README.md"), join(out, "README.md"));
copyFileSync(join(root, "LICENSE"), join(out, "LICENSE"));
console.log(`npm-dist/diracord pronto (v${version}) com motor para: ${engines.map(([k]) => k).join(", ")}`);
