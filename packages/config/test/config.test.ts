import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { ConfigError, getToken, loadProject, parseEnv, parseToml, readConfig, redact, TomlError } from "../src/index.ts";

test("toml: tables, values, arrays, inline tables", () => {
  const t = parseToml(`
# comentário
ram = "100mb"
[bot]
name = "x" # outro
prefix = '!'
[a.b]
n = 1_000
f = -1.5
ok = true
list = [1, "dois",
  [3],
]
inline = { x = 1, "y z" = "w" }
`);
  assert.deepEqual(t, {
    ram: "100mb",
    bot: { name: "x", prefix: "!" },
    a: { b: { n: 1000, f: -1.5, ok: true, list: [1, "dois", [3]], inline: { x: 1, "y z": "w" } } },
  });
});

test("toml: errors carry line numbers", () => {
  assert.throws(() => parseToml("[bot]\nname = \"x"), (e: unknown) => e instanceof TomlError && e.line === 2);
  assert.throws(() => parseToml("a = 1\na = 2"), TomlError);
  assert.throws(() => parseToml("a = nope"), TomlError);
});

test("config: defaults and validation", () => {
  const c = readConfig({});
  assert.equal(c.bot.prefix, "!");
  assert.equal(c.commands.slash, true);
  assert.equal(c.syntax.dollar, false);
  assert.equal(c.data.defaultScope, "user");
  assert.throws(() => readConfig({ data: { default_scope: "guild" } }), (e: unknown) => e instanceof ConfigError && e.code === "E0501");
  assert.throws(() => readConfig({ syntax: { dollar: "yes" } }), ConfigError);
  assert.throws(() => readConfig({ commands: { prefix: false, slash: false } }), ConfigError);
  assert.throws(() => readConfig({ banana: {} }), ConfigError);
});

test("env: quotes, comments, export", () => {
  assert.deepEqual(parseEnv('# c\nexport A=1\nB="x y"\nC=\'z\'\nD=v # c\n'), { A: "1", B: "x y", C: "z", D: "v" });
  assert.equal(redact("abcdefghijklmnop"), "abcd…mnop");
  assert.equal(redact("short"), "••••");
});

test("loadProject finds files and the token", () => {
  const root = mkdtempSync(join(tmpdir(), "diracord-cfg-"));
  writeFileSync(join(root, "diracord.toml"), '[bot]\nprefix = "?"\n');
  writeFileSync(join(root, ".env"), "DISCORD_TOKEN=abc\n");
  mkdirSync(join(root, "commands"));
  mkdirSync(join(root, "data"));
  writeFileSync(join(root, "commands", "ping.dcs"), 'command("ping") { reply "Pong!" }');
  writeFileSync(join(root, "data", "ignored.dcs"), "");
  writeFileSync(join(root, "notes.txt"), "");
  const p = loadProject(join(root, "commands"));
  assert.equal(p.root, root);
  assert.equal(p.config.bot.prefix, "?");
  assert.deepEqual(p.files.map((f) => f.path), ["commands/ping.dcs"]);
  assert.equal(getToken(p, {}), "abc");
});

test("loadProject errors", () => {
  const empty = mkdtempSync(join(tmpdir(), "diracord-none-"));
  assert.throws(() => loadProject(empty), (e: unknown) => e instanceof ConfigError && e.code === "E0500");
  writeFileSync(join(empty, "diracord.toml"), "");
  assert.throws(() => loadProject(empty), (e: unknown) => e instanceof ConfigError && e.code === "E0503");
  writeFileSync(join(empty, "diracord.toml"), "[bot\n");
  assert.throws(() => loadProject(empty), (e: unknown) => e instanceof ConfigError && e.code === "E0501" && e.line === 1);
  writeFileSync(join(empty, "diracord.toml"), "");
  writeFileSync(join(empty, "a.dcs"), "");
  assert.throws(() => getToken(loadProject(empty), {}), (e: unknown) => e instanceof ConfigError && e.code === "E0502");
});
