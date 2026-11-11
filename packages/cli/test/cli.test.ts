import { test } from "node:test";
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const MAIN = fileURLToPath(new URL("../src/main.ts", import.meta.url));
const cli = (args: string[], cwd: string) => spawnSync(process.execPath, [MAIN, ...args], { cwd, encoding: "utf8", env: { ...process.env, NO_COLOR: "1" } });

test("new creates a project that passes check", () => {
  const tmp = mkdtempSync(join(tmpdir(), "diracord-cli-"));
  const r = cli(["new", "meu-bot"], tmp);
  assert.equal(r.status, 0, r.stderr);
  const root = join(tmp, "meu-bot");
  for (const f of ["diracord.toml", ".env.example", ".gitignore", "commands/ping.dcs", "events/join.dcs"]) {
    assert.ok(existsSync(join(root, f)), f);
  }
  assert.ok(!existsSync(join(root, ".env")), "nunca criar .env com token");
  const c = cli(["check"], root);
  assert.equal(c.status, 0, c.stdout + c.stderr);
  assert.match(c.stdout, /nenhum problema encontrado/);
});

test("check shows a clear error in a broken file", () => {
  const tmp = mkdtempSync(join(tmpdir(), "diracord-err-"));
  cli(["new", "bot"], tmp);
  const root = join(tmp, "bot");
  writeFileSync(join(root, "commands/ping.dcs"), 'command("ping") {\n  rpely "Pong!"\n}\n');
  const r = cli(["check"], root);
  assert.equal(r.status, 1);
  assert.match(r.stdout, /erro\[E0201\]: A função `rpely` não existe/);
  assert.match(r.stdout, /--> commands\/ping\.dcs:2:3/);
  assert.match(r.stdout, /\^\^\^\^\^ função desconhecida/);
  assert.match(r.stdout, /você quis dizer `reply`\?/);
  assert.match(r.stdout, /1 erro/);
});

test("check --fix applies safe fixes", () => {
  const tmp = mkdtempSync(join(tmpdir(), "diracord-fix-"));
  cli(["new", "bot"], tmp);
  const root = join(tmp, "bot");
  writeFileSync(join(root, "commands/x.dcs"), 'command("x") {\n  let n = 1\n  if n = 1 { $reply["um"] }\n}\n');
  const before = cli(["check"], root);
  assert.equal(before.status, 1);
  assert.match(before.stdout, /diracord check --fix/);
  const r = cli(["check", "--fix"], root);
  assert.equal(r.status, 0, r.stdout);
  assert.equal(readFileSync(join(root, "commands/x.dcs"), "utf8"), 'command("x") {\n  local n = 1\n  if n == 1 { reply("um") }\n}\n');
});

test("errors outside a project and without token are explained", () => {
  const tmp = mkdtempSync(join(tmpdir(), "diracord-none-"));
  const r = cli(["check"], tmp);
  assert.equal(r.status, 1);
  assert.match(r.stderr, /E0500/);
  cli(["new", "b"], tmp);
  const run = cli(["run"], join(tmp, "b"));
  assert.equal(run.status, 1);
  assert.match(run.stderr, /E0502/);
  assert.match(run.stderr, /\.env/);
});

test("help", () => {
  const out = execFileSync(process.execPath, [MAIN, "help"], { encoding: "utf8" });
  assert.match(out, /diracord check/);
});
