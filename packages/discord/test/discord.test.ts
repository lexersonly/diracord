import { test } from "node:test";
import assert from "node:assert/strict";
import { EngineClient } from "@diracord/engine";
import {
  Bot,
  bucketPath,
  color,
  DiscordHost,
  Gateway,
  intentBits,
  messageBody,
  NameCache,
  parsePrefix,
  Rest,
  slashArgs,
  slashCommands,
  splitArgs,
  type SocketLike,
} from "../src/index.ts";
import { fakeFetch } from "./fake.ts";

const API = "https://discord.com/api/v10";
const manifest = {
  commands: [
    { name: "ping", params: [] },
    {
      name: "ban",
      params: [
        { name: "user", type: "member" as const, optional: false },
        { name: "reason", type: "text" as const, optional: true },
      ],
    },
  ],
  events: [],
  interactions: [],
};

test("prefix parsing keeps the rest of the line for the last text argument", () => {
  assert.deepEqual(splitArgs('a "b c" d'), ["a", "b c", "d"]);
  assert.deepEqual(parsePrefix("!PING", "!", manifest), { name: "ping", args: {} });
  assert.deepEqual(parsePrefix("!ban <@123456789012345678> spam de   link", "!", manifest), {
    name: "ban",
    args: { user: "<@123456789012345678>", reason: "spam de   link" },
  });
  assert.equal(parsePrefix("ping", "!", manifest), null);
  assert.equal(parsePrefix("!nada", "!", manifest), null);
});

test("slash commands JSON and argument resolution", () => {
  const { commands, skipped } = slashCommands({ ...manifest, commands: [...manifest.commands, { name: "Bad Name", params: [] }] });
  assert.deepEqual(skipped, ["Bad Name"]);
  assert.deepEqual(commands[1]?.["options"], [
    { type: 6, name: "user", description: "user", required: true },
    { type: 3, name: "reason", description: "reason", required: false },
  ]);
  const args = slashArgs({
    data: {
      options: [
        { name: "user", type: 6, value: "5" },
        { name: "reason", type: 3, value: "spam" },
      ],
      resolved: { users: { "5": { id: "5", username: "ze" } }, members: { "5": { nick: "Zé", roles: [] } } },
    },
  });
  assert.equal((args["user"] as { name: string }).name, "Zé");
  assert.equal(args["reason"], "spam");
});

test("message bodies: text, embeds, colors, safe mentions", () => {
  assert.deepEqual(messageBody("oi"), { allowed_mentions: { parse: ["users"] }, content: "oi" });
  const e = messageBody({ title: "Oi", color: "red", footer: "rodapé", fields: [{ name: "a", value: 1 }] }, { ephemeral: true });
  assert.deepEqual(e["embeds"], [{ title: "Oi", color: 0xed4245, footer: { text: "rodapé" }, fields: [{ name: "a", value: "1", inline: false }] }]);
  assert.equal(e["flags"], 64);
  assert.equal(color("#00ff00"), 0x00ff00);
  assert.equal(color("nope"), undefined);
});

test("intents and rate-limit buckets", () => {
  assert.equal(intentBits(["guilds", "messageContent"]), 1 | (1 << 15));
  assert.equal(bucketPath("/channels/123456789012345678/messages/223456789012345678"), "/channels/123456789012345678/messages/:id");
});

test("rest retries after 429 and sends audit reasons", async () => {
  let n = 0;
  const { fetch, calls } = fakeFetch(() => (n++ === 0 ? { status: 429, body: { retry_after: 0.01 } } : { body: { ok: 1 } }));
  const rest = new Rest("TOKEN", fetch);
  assert.deepEqual(await rest.request("PUT", "/guilds/1/bans/2", { body: {}, reason: "spam ação" }), { ok: 1 });
  assert.equal(calls.length, 2);
  assert.equal(calls[1]?.headers["Authorization"], "Bot TOKEN");
  assert.equal(calls[1]?.headers["X-Audit-Log-Reason"], encodeURIComponent("spam ação"));
  const bad = new Rest("T", fakeFetch(() => ({ status: 403, body: { message: "Missing Permissions", code: 50013 } })).fetch);
  await assert.rejects(bad.request("GET", "/x"), /Missing Permissions/);
});

test("host maps actions to REST calls", async () => {
  const { fetch, calls } = fakeFetch((c) => {
    if (c.url.endsWith("/roles")) return { body: [{ id: "900000000000000001", name: "VIP" }] };
    if (c.method === "POST") return { body: { id: "m2", channel_id: "c1", content: "x" } };
    if (c.url.includes("/members/") && c.method === "GET") return { body: { roles: ["900000000000000001"] } };
    return {};
  });
  const rest = new Rest("T", fetch);
  const host = new DiscordHost(rest, new NameCache(rest));
  const target = { guildId: "g1", channelId: "c1", messageId: "m1", interaction: null };
  const state = { responded: false };
  const a = (name: string, args: unknown[], options = {}) => host.perform({ name, args, options }, target, state);
  await a("reply", ["Pong!"]);
  await a("ban", ["5", "spam"]);
  await a("timeout", ["5", 60, null]);
  await a("addRole", ["5", "vip"]);
  assert.equal(await a("hasRole", ["5", "VIP"]), true);
  await a("react", ["👍"]);
  const summary = calls.map((c) => `${c.method} ${c.url.replace(API, "")}`);
  assert.deepEqual(summary, [
    "POST /channels/c1/messages",
    "PUT /guilds/g1/bans/5",
    "PATCH /guilds/g1/members/5",
    "GET /guilds/g1/roles",
    "PUT /guilds/g1/members/5/roles/900000000000000001",
    "GET /guilds/g1/members/5",
    `PUT /channels/c1/messages/m1/reactions/${encodeURIComponent("👍")}/@me`,
  ]);
  assert.deepEqual((calls[0]?.body as Record<string, unknown>)["message_reference"], { message_id: "m1", fail_if_not_exists: false });
  await assert.rejects(a("addRole", ["5", "nada"]), /não achei o cargo "nada"/);
});

test("interaction replies: first is a callback, then follow-ups", async () => {
  const { fetch, calls } = fakeFetch((c) => (c.url.includes("/webhooks/") ? { body: { id: "f1" } } : {}));
  const rest = new Rest("T", fetch);
  const host = new DiscordHost(rest, new NameCache(rest));
  const { onAction, state } = host.forRun({ guildId: "g", channelId: "c", messageId: null, interaction: { id: "i1", token: "tok", applicationId: "app" } });
  await onAction({ name: "reply", args: ["a"], options: {} });
  await onAction({ name: "reply", args: ["b"], options: {} });
  assert.equal(state.responded, true);
  assert.equal(calls[0]?.url, `${API}/interactions/i1/tok/callback`);
  assert.equal((calls[0]?.body as { type: number }).type, 4);
  assert.equal(calls[0]?.headers["Authorization"], undefined);
  assert.equal(calls[1]?.url, `${API}/webhooks/app/tok`);
});

class FakeSocket implements SocketLike {
  onopen: ((ev: unknown) => void) | null = null;
  onmessage: ((ev: { data: unknown }) => void) | null = null;
  onclose: ((ev: { code: number }) => void) | null = null;
  onerror: ((ev: unknown) => void) | null = null;
  sent: { op: number; d: any }[] = [];
  send(data: string) {
    this.sent.push(JSON.parse(data));
  }
  close() {}
  emit(p: unknown) {
    this.onmessage?.({ data: JSON.stringify(p) });
  }
}

test("gateway identifies, heartbeats and dispatches", async () => {
  const sock = new FakeSocket();
  const events: string[] = [];
  const gw = new Gateway({ token: "T", intents: 513, url: "wss://x", socket: () => sock, onDispatch: (t) => events.push(t) });
  gw.connect();
  sock.emit({ op: 10, d: { heartbeat_interval: 60_000 } });
  assert.equal(sock.sent[0]?.op, 2);
  assert.equal(sock.sent[0]?.d.intents, 513);
  sock.emit({ op: 0, t: "READY", s: 1, d: { session_id: "s", resume_gateway_url: "wss://r" } });
  sock.emit({ op: 1, d: null });
  assert.deepEqual(events, ["READY"]);
  assert.deepEqual(sock.sent[1], { op: 1, d: 1 });
  gw.close();
});

test("!ping end to end: gateway message → engine → REST reply", async () => {
  const engine = await EngineClient.start();
  try {
    const loaded = await engine.load([{ path: "ping.dcs", source: 'command("ping") { reply "Pong!" }' }]);
    assert.equal(loaded.ok, true);
    const { fetch, calls } = fakeFetch(() => ({ body: { id: "m2", channel_id: "c1" } }));
    const logs: string[] = [];
    const bot = new Bot({
      token: "T",
      engine,
      manifest: { commands: loaded.commands ?? [], events: loaded.events ?? [], interactions: loaded.interactions ?? [] },
      prefix: "!",
      prefixCommands: true,
      slashCommands: true,
      intents: ["guilds"],
      fetch,
      log: (m) => logs.push(m),
    });
    await bot.onEvent("MESSAGE_CREATE", {
      id: "m1",
      channel_id: "c1",
      guild_id: "g1",
      content: "!ping",
      author: { id: "42", username: "ana" },
      member: { roles: [] },
    });
    assert.equal(calls.length, 1, logs.join("\n"));
    assert.equal(calls[0]?.url, `${API}/channels/c1/messages`);
    assert.equal((calls[0]?.body as { content: string }).content, "Pong!");
    // Bots are ignored.
    await bot.onEvent("MESSAGE_CREATE", { id: "m3", channel_id: "c1", content: "!ping", author: { id: "1", bot: true } });
    assert.equal(calls.length, 1);
  } finally {
    await engine.close();
  }
});

test("components v2: layout object → Discord components", () => {
  const body = messageBody(
    {
      layout: [
        {
          container: [
            { text: "## Oi" },
            { section: ["Leia"], thumbnail: { url: "https://x/a.png", description: "logo" } },
            { separator: true, spacing: "large", line: false },
            { row: [{ button: "ok", label: "Ok", style: "success", emoji: "👍" }, { button: null, label: "Site", url: "https://a.b" }] },
            { row: [{ menu: "cor", options: ["Azul", { label: "Verde", value: "g" }], placeholder: "Escolha" }] },
          ],
          color: "purple",
        },
        { gallery: ["https://x/1.png"] },
      ],
    },
    { ephemeral: true },
  );
  assert.equal(body["flags"], (1 << 15) | 64);
  assert.equal(body["content"], undefined);
  assert.deepEqual(body["components"], [
    {
      type: 17,
      accent_color: 0x9b59b6,
      components: [
        { type: 10, content: "## Oi" },
        { type: 9, components: [{ type: 10, content: "Leia" }], accessory: { type: 11, media: { url: "https://x/a.png" }, description: "logo" } },
        { type: 14, spacing: 2, divider: false },
        {
          type: 1,
          components: [
            { type: 2, style: 3, custom_id: "ok", label: "Ok", emoji: { name: "👍" } },
            { type: 2, style: 5, url: "https://a.b", label: "Site" },
          ],
        },
        {
          type: 1,
          components: [
            { type: 3, custom_id: "cor", placeholder: "Escolha", options: [{ label: "Azul", value: "Azul" }, { label: "Verde", value: "g" }] },
          ],
        },
      ],
    },
    { type: 12, items: [{ media: { url: "https://x/1.png" } }] },
  ]);
});

test("components v2 end to end: layout block → engine → REST body", async () => {
  const engine = await EngineClient.start();
  try {
    const src = 'command("menu") {\n  reply layout {\n    text "Escolha:"\n    row {\n      button("a", "A")\n      button("b", "B", style: "danger")\n    }\n  }\n}\nbutton("a") { reply "A!" }';
    const loaded = await engine.load([{ path: "m.dcs", source: src }]);
    assert.equal(loaded.ok, true, loaded.rendered);
    const { fetch, calls } = fakeFetch(() => ({ body: { id: "m2", channel_id: "c1" } }));
    const rest = new Rest("T", fetch);
    const host = new DiscordHost(rest, new NameCache(rest));
    const { onAction } = host.forRun({ guildId: "g", channelId: "c1", messageId: "m1", interaction: null });
    const r = await engine.dispatch({ kind: "command", name: "menu" }, {}, onAction);
    assert.equal(r.ok, true);
    const body = calls[0]?.body as Record<string, unknown>;
    assert.equal(body["flags"], 1 << 15);
    assert.deepEqual(body["components"], [
      { type: 10, content: "Escolha:" },
      {
        type: 1,
        components: [
          { type: 2, style: 1, custom_id: "a", label: "A" },
          { type: 2, style: 4, custom_id: "b", label: "B" },
        ],
      },
    ]);
  } finally {
    await engine.close();
  }
});
