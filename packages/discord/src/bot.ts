/**
 * The running bot: Gateway events → engine triggers → Discord actions.
 * @module
 */

import type { EngineClient, Manifest, RunContext, Trigger } from "@diracord/engine";
import { NameCache } from "./cache.ts";
import { parsePrefix, slashArgs, slashCommands } from "./commands.ts";
import { channel as toChannel, member as toMember, message as toMessage, user as toUser, type Raw } from "./entities.ts";
import { Gateway, type SocketFactory } from "./gateway.ts";
import { DiscordHost, type RunTarget } from "./host.ts";
import { intentBits } from "./intents.ts";
import { Rest, type FetchLike } from "./rest.ts";

/** Bot settings (from `diracord.toml`). */
export interface BotOptions {
  token: string;
  engine: EngineClient;
  manifest: Manifest;
  prefix: string;
  prefixCommands: boolean;
  slashCommands: boolean;
  intents: string[];
  /** Commands per user per 10 s (simple rate limit). */
  userRateLimit?: number;
  log?: (message: string) => void;
  fetch?: FetchLike;
  socket?: SocketFactory;
}

const COMPONENT_KINDS: Record<number, "button" | "menu"> = { 2: "button", 3: "menu", 5: "menu", 6: "menu", 7: "menu", 8: "menu" };

/** Connects to Discord and runs the project's code. */
export class Bot {
  private readonly o: BotOptions;
  private readonly rest: Rest;
  private readonly cache: NameCache;
  private readonly host: DiscordHost;
  private gateway: Gateway | null = null;
  private applicationId = "";
  private readonly hits = new Map<string, number[]>();
  private readonly log: (m: string) => void;

  constructor(o: BotOptions) {
    this.o = o;
    this.rest = new Rest(o.token, o.fetch);
    this.cache = new NameCache(this.rest);
    this.host = new DiscordHost(this.rest, this.cache);
    this.log = o.log ?? ((m) => console.log(m));
  }

  /** Swaps the loaded code (used by `watch`). */
  setManifest(m: Manifest): void {
    this.o.manifest = m;
    if (this.applicationId) void this.registerSlash();
  }

  /** Connects to the Gateway. */
  async start(): Promise<void> {
    const info = await this.rest.request<{ url: string }>("GET", "/gateway/bot");
    const opts = {
      token: this.o.token,
      intents: intentBits(this.o.intents),
      url: info.url,
      onDispatch: (t: string, d: any) => void this.onEvent(t, d),
      onLog: (m: string) => this.log(`[gateway] ${m}`),
      ...(this.o.socket ? { socket: this.o.socket } : {}),
    };
    this.gateway = new Gateway(opts);
    this.gateway.connect();
  }

  /** Disconnects. */
  stop(): void {
    this.gateway?.close();
  }

  private limited(userId: string): boolean {
    const max = this.o.userRateLimit ?? 5;
    const now = Date.now();
    const list = (this.hits.get(userId) ?? []).filter((t) => now - t < 10_000);
    if (list.length >= max) return true;
    list.push(now);
    this.hits.set(userId, list);
    if (this.hits.size > 10_000) this.hits.clear();
    return false;
  }

  private async registerSlash(): Promise<void> {
    if (!this.o.slashCommands) return;
    const { commands, skipped } = slashCommands(this.o.manifest);
    for (const s of skipped) this.log(`aviso: "${s}" não vira slash command (nomes de slash usam só minúsculas, números, - e _)`);
    try {
      await this.rest.request("PUT", `/applications/${this.applicationId}/commands`, { body: commands });
      this.log(`${commands.length} slash command(s) registrado(s)`);
    } catch (e) {
      this.log(`não consegui registrar os slash commands: ${(e as Error).message}`);
    }
  }

  private async run(trigger: Trigger, ctx: RunContext, target: RunTarget): Promise<boolean> {
    const { onAction, state } = this.host.forRun(target);
    const r = await this.o.engine.dispatch(trigger, ctx, onAction);
    if (!r.ok) this.log(`\n${r.error.rendered}`);
    return state.responded;
  }

  private hasEvent(name: string): boolean {
    return this.o.manifest.events.includes(name);
  }

  private async event(name: string, ctx: RunContext, target: Partial<RunTarget> = {}): Promise<void> {
    if (!this.hasEvent(name)) return;
    await this.run({ kind: "event", name }, ctx, { guildId: null, channelId: null, messageId: null, interaction: null, ...target });
  }

  /** Handles one Gateway dispatch. */
  async onEvent(t: string, d: Raw): Promise<void> {
    try {
      switch (t) {
        case "READY":
          this.applicationId = String(d["application"]?.["id"] ?? d["user"]?.["id"] ?? "");
          this.log(`conectado como ${d["user"]?.["username"] ?? "?"}`);
          await this.registerSlash();
          await this.event("ready", { user: d["user"] ? toUser(d["user"]) : null });
          break;
        case "MESSAGE_CREATE":
          await this.onMessage(d);
          break;
        case "INTERACTION_CREATE":
          await this.onInteraction(d);
          break;
        case "GUILD_MEMBER_ADD":
        case "GUILD_MEMBER_REMOVE":
        case "GUILD_MEMBER_UPDATE": {
          const name = t === "GUILD_MEMBER_ADD" ? "memberJoin" : t === "GUILD_MEMBER_REMOVE" ? "memberLeave" : "memberUpdate";
          const u = t === "GUILD_MEMBER_REMOVE" ? toUser(d["user"]) : toMember(d);
          await this.event(name, { user: u, server: { id: String(d["guild_id"]) } }, { guildId: String(d["guild_id"]) });
          break;
        }
        case "MESSAGE_DELETE":
        case "MESSAGE_UPDATE": {
          const name = t === "MESSAGE_DELETE" ? "messageDelete" : "messageUpdate";
          const ctx: RunContext = { message: toMessage(d), channel: { id: String(d["channel_id"]) } };
          if (d["guild_id"]) ctx.server = { id: String(d["guild_id"]) };
          if (d["author"]) ctx.user = toUser(d["author"]);
          await this.event(name, ctx, { guildId: d["guild_id"] ?? null, channelId: String(d["channel_id"]) });
          break;
        }
        case "MESSAGE_REACTION_ADD":
        case "MESSAGE_REACTION_REMOVE": {
          const name = t === "MESSAGE_REACTION_ADD" ? "reactionAdd" : "reactionRemove";
          const emoji = d["emoji"]?.["id"] ? `<:${d["emoji"]["name"]}:${d["emoji"]["id"]}>` : String(d["emoji"]?.["name"] ?? "");
          const ctx: RunContext = {
            user: d["member"] ? toMember(d["member"]) : { id: String(d["user_id"]), mention: `<@${d["user_id"]}>` },
            channel: { id: String(d["channel_id"]) },
            message: { id: String(d["message_id"]) },
            extra: { reaction: { emoji } },
          };
          if (d["guild_id"]) ctx.server = { id: String(d["guild_id"]) };
          await this.event(name, ctx, { guildId: d["guild_id"] ?? null, channelId: String(d["channel_id"]), messageId: String(d["message_id"]) });
          break;
        }
        case "CHANNEL_CREATE":
        case "CHANNEL_DELETE":
          if (d["guild_id"]) this.cache.invalidate(String(d["guild_id"]));
          await this.event(t === "CHANNEL_CREATE" ? "channelCreate" : "channelDelete", { channel: toChannel(d), server: { id: String(d["guild_id"]) } }, { guildId: d["guild_id"] ?? null });
          break;
        case "GUILD_ROLE_CREATE":
        case "GUILD_ROLE_DELETE":
          this.cache.invalidate(String(d["guild_id"]));
          await this.event(t === "GUILD_ROLE_CREATE" ? "roleCreate" : "roleDelete", { server: { id: String(d["guild_id"]) }, extra: { role: d["role"] ?? { id: d["role_id"] } } }, { guildId: String(d["guild_id"]) });
          break;
        case "VOICE_STATE_UPDATE": {
          const name = d["channel_id"] ? "voiceJoin" : "voiceLeave";
          const ctx: RunContext = { user: d["member"] ? toMember(d["member"]) : { id: String(d["user_id"]) } };
          if (d["guild_id"]) ctx.server = { id: String(d["guild_id"]) };
          if (d["channel_id"]) ctx.channel = { id: String(d["channel_id"]) };
          await this.event(name, ctx, { guildId: d["guild_id"] ?? null });
          break;
        }
      }
    } catch (e) {
      this.log(`erro ao tratar ${t}: ${(e as Error).message}`);
    }
  }

  private async onMessage(d: Raw): Promise<void> {
    const author = d["author"] as Raw | undefined;
    if (!author || author["bot"]) return;
    const guildId = d["guild_id"] ? String(d["guild_id"]) : null;
    const target: RunTarget = { guildId, channelId: String(d["channel_id"]), messageId: String(d["id"]), interaction: null };
    const ctx: RunContext = {
      user: d["member"] ? toMember(d["member"], author) : toUser(author),
      channel: { id: String(d["channel_id"]), mention: `<#${d["channel_id"]}>` },
      message: toMessage(d),
    };
    if (guildId) ctx.server = { id: guildId };
    await this.event("messageCreate", ctx, target);
    if (!this.o.prefixCommands) return;
    const inv = parsePrefix(String(d["content"] ?? ""), this.o.prefix, this.o.manifest);
    if (!inv || this.limited(String(author["id"]))) return;
    const args = await this.resolvePrefixArgs(inv.name, inv.args, d, guildId);
    await this.run({ kind: "command", name: inv.name, args }, ctx, target);
  }

  /** Mentions/ids for member/user/channel/role parameters become entities. */
  private async resolvePrefixArgs(name: string, raw: Record<string, string>, msg: Raw, guildId: string | null): Promise<Record<string, unknown>> {
    const cmd = this.o.manifest.commands.find((c) => c.name === name);
    const out: Record<string, unknown> = { ...raw };
    for (const p of cmd?.params ?? []) {
      const v = raw[p.name];
      if (v === undefined) continue;
      const id = /^<?[@#]?[!&]?(\d{15,25})>?$/.exec(v)?.[1];
      if (!id) continue;
      if (p.type === "member" || p.type === "user") {
        const mentioned = (msg["mentions"] as Raw[] | undefined)?.find((u) => String(u["id"]) === id);
        if (mentioned) out[p.name] = mentioned["member"] ? toMember(mentioned["member"], mentioned) : toUser(mentioned);
        else if (guildId && p.type === "member") {
          try {
            out[p.name] = toMember(await this.rest.request<Raw>("GET", `/guilds/${guildId}/members/${id}`));
          } catch {
            out[p.name] = id;
          }
        } else out[p.name] = { id, mention: `<@${id}>` };
      } else if (p.type === "channel") out[p.name] = { id, mention: `<#${id}>` };
      else if (p.type === "role") out[p.name] = { id, mention: `<@&${id}>` };
    }
    return out;
  }

  private async onInteraction(d: Raw): Promise<void> {
    const author = (d["member"]?.["user"] ?? d["user"]) as Raw | undefined;
    const guildId = d["guild_id"] ? String(d["guild_id"]) : null;
    const target: RunTarget = {
      guildId,
      channelId: d["channel_id"] ? String(d["channel_id"]) : null,
      messageId: d["message"]?.["id"] ? String(d["message"]["id"]) : null,
      interaction: { id: String(d["id"]), token: String(d["token"]), applicationId: String(d["application_id"]) },
    };
    const ctx: RunContext = {
      user: author ? (d["member"] ? toMember(d["member"], author) : toUser(author)) : null,
      channel: d["channel_id"] ? { id: String(d["channel_id"]), mention: `<#${d["channel_id"]}>` } : null,
      message: { id: String(d["id"]), values: d["data"]?.["values"] ?? [] },
      isInteraction: true,
    };
    if (guildId) ctx.server = { id: guildId };
    let trigger: Trigger | null = null;
    if (d["type"] === 2) trigger = { kind: "command", name: String(d["data"]?.["name"]), args: slashArgs(d) };
    else if (d["type"] === 3) {
      const kind = COMPONENT_KINDS[d["data"]?.["component_type"] as number] ?? "button";
      trigger = { kind: "interaction", type: kind, id: String(d["data"]?.["custom_id"]) };
    } else if (d["type"] === 5) trigger = { kind: "interaction", type: "modal", id: String(d["data"]?.["custom_id"]) };
    if (!trigger) return;
    if (author && this.limited(String(author["id"]))) return;
    const responded = await this.run(trigger, ctx, target);
    if (!responded) {
      // Discord needs an answer within 3 s: acknowledge silently.
      const type = d["type"] === 2 ? 5 : 6;
      try {
        await this.rest.request("POST", `/interactions/${target.interaction!.id}/${target.interaction!.token}/callback`, { body: { type }, noAuth: true });
      } catch {
        // expired or already answered
      }
    }
  }
}
