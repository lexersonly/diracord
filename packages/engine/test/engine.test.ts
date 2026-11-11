import { test, before, after } from "node:test";
import assert from "node:assert/strict";
import { EngineClient, findEngine, type Action } from "../src/index.ts";

let engine: EngineClient;

before(async () => {
  assert.ok(findEngine(), "rode `cargo build -p diracord-engine` antes dos testes TS");
  engine = await EngineClient.start();
});

after(async () => {
  await engine.close();
});

test("check shows a clear error", async () => {
  const r = await engine.check([{ path: "commands/ping.dcs", source: 'command("ping") {\n  rpely "Pong!"\n}\n' }]);
  assert.equal(r.ok, false);
  assert.equal(r.errors, 1);
  assert.equal(r.diagnostics[0]?.code, "E0201");
  assert.equal(r.diagnostics[0]?.line, 2);
  assert.match(r.rendered, /você quis dizer `reply`\?/);
});

test("fix applies safe fixes", async () => {
  const r = await engine.fix([{ path: "a.dcs", source: "memberjoin { let x = 1 }" }]);
  assert.equal(r.files[0]?.source, "memberJoin { local x = 1 }");
  assert.equal(r.files[0]?.changed, true);
});

test("ping runs end to end", async () => {
  const loaded = await engine.load([{ path: "ping.dcs", source: 'command("ping") { reply "Pong!" }' }]);
  assert.equal(loaded.ok, true);
  assert.deepEqual(loaded.commands, [{ name: "ping", params: [] }]);
  const seen: Action[] = [];
  const r = await engine.dispatch({ kind: "command", name: "ping" }, { user: { id: "1" } }, async (a) => {
    seen.push(a);
    return { id: "m1" };
  });
  assert.equal(r.ok, true);
  assert.deepEqual(seen, [{ name: "reply", args: ["Pong!"], options: {} }]);
});

test("host failures reach testing/catch, concurrent dispatches are queued", async () => {
  await engine.load([
    { path: "b.dcs", source: 'command("b", u: member) { testing { ban u } catch err { reply "Falhou: $err" } }' },
  ]);
  const replies: unknown[] = [];
  const onAction = async (a: Action) => {
    if (a.name === "ban") throw new Error("Missing Permissions");
    replies.push(a.args[0]);
    return null;
  };
  const runs = [1, 2, 3].map((n) =>
    engine.dispatch({ kind: "command", name: "b", args: { u: { id: String(n) } } }, {}, onAction),
  );
  const results = await Promise.all(runs);
  assert.ok(results.every((r) => r.ok));
  assert.equal(replies.length, 3);
  assert.match(String(replies[0]), /Falhou: O Discord recusou `ban`: Missing Permissions/);
});

test("runtime errors come back rendered", async () => {
  await engine.load([{ path: "z.dcs", source: 'command("z") {\n  reply 1 / 0\n}' }]);
  const r = await engine.dispatch({ kind: "command", name: "z" }, {}, async () => null);
  assert.equal(r.ok, false);
  if (!r.ok) {
    assert.equal(r.error.code, "E0203");
    assert.match(r.error.rendered, /z\.dcs:2:9/);
  }
});
