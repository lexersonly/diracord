/**
 * The Discord side of the engine's `Host`: turns engine actions into REST calls.
 * @module
 */

import type { Action, ActionHandler } from "@diracord/engine";
import { idOf, type NameCache } from "./cache.ts";
import { messageBody } from "./content.ts";
import { channel as toChannel, member as toMember, message as toMessage, type Raw } from "./entities.ts";
import type { Rest } from "./rest.ts";

/** Where a run happens: which message or interaction to answer. */
export interface RunTarget {
  guildId: string | null;
  channelId: string | null;
  messageId: string | null;
  interaction: { id: string; token: string; applicationId: string } | null;
}

/** Tracks whether an interaction was already answered (Discord needs exactly one first response). */
export interface RunState {
  responded: boolean;
}

/** Raised for problems the bot author can fix ("não achei o cargo VIP"). */
export class HostError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "HostError";
  }
}

const MAX_TIMEOUT_S = 28 * 24 * 3600;

/** Builds action handlers for runs. One instance per bot. */
export class DiscordHost {
  private readonly rest: Rest;
  private readonly cache: NameCache;

  constructor(rest: Rest, cache: NameCache) {
    this.rest = rest;
    this.cache = cache;
  }

  /** A handler for one run, plus its state. */
  forRun(target: RunTarget): { onAction: ActionHandler; state: RunState } {
    const state: RunState = { responded: false };
    const onAction: ActionHandler = (a) => this.perform(a, target, state);
    return { onAction, state };
  }

  private guild(t: RunTarget): string {
    if (!t.guildId) throw new HostError("essa ação só funciona dentro de um servidor");
    return t.guildId;
  }

  private str(a: Action, i: number): string | null {
    const v = a.args[i];
    return v == null ? null : String(v);
  }

  /** Performs one action. Exposed for tests. */
  async perform(a: Action, t: RunTarget, state: RunState): Promise<unknown> {
    const r = this.rest;
    switch (a.name) {
      case "reply":
        return this.reply(a, t, state);
      case "send": {
        const m = await r.request<Raw>("POST", `/channels/${this.str(a, 0)}/messages`, { body: messageBody(a.args[1]) });
        return toMessage(m);
      }
      case "dm": {
        const ch = await r.request<Raw>("POST", "/users/@me/channels", { body: { recipient_id: this.str(a, 0) } });
        const m = await r.request<Raw>("POST", `/channels/${ch["id"]}/messages`, { body: messageBody(a.args[1]) });
        return toMessage(m);
      }
      case "react": {
        if (!t.channelId || !t.messageId) throw new HostError("não há mensagem para reagir");
        const emoji = encodeURIComponent(String(a.args[0]).replace(/^<a?:/, "").replace(/>$/, ""));
        await r.request("PUT", `/channels/${t.channelId}/messages/${t.messageId}/reactions/${emoji}/@me`);
        return null;
      }
      case "deleteMessage": {
        const id = this.str(a, 0) ?? t.messageId;
        if (!t.channelId || !id) throw new HostError("não há mensagem para apagar");
        await r.request("DELETE", `/channels/${t.channelId}/messages/${id}`);
        return null;
      }
      case "ban":
        await r.request("PUT", `/guilds/${this.guild(t)}/bans/${this.str(a, 0)}`, { body: {}, reason: this.str(a, 1) });
        return null;
      case "unban":
        await r.request("DELETE", `/guilds/${this.guild(t)}/bans/${this.str(a, 0)}`, { reason: this.str(a, 1) });
        return null;
      case "kick":
        await r.request("DELETE", `/guilds/${this.guild(t)}/members/${this.str(a, 0)}`, { reason: this.str(a, 1) });
        return null;
      case "timeout": {
        const secs = Math.min(Math.max(Number(a.args[1]) || 0, 0), MAX_TIMEOUT_S);
        const until = secs === 0 ? null : new Date(Date.now() + secs * 1000).toISOString();
        await r.request("PATCH", `/guilds/${this.guild(t)}/members/${this.str(a, 0)}`, {
          body: { communication_disabled_until: until },
          reason: this.str(a, 2),
        });
        return null;
      }
      case "addRole":
      case "removeRole": {
        const g = this.guild(t);
        const role = await this.roleId(g, String(a.args[1]));
        const method = a.name === "addRole" ? "PUT" : "DELETE";
        await r.request(method, `/guilds/${g}/members/${this.str(a, 0)}/roles/${role}`);
        return null;
      }
      case "hasRole": {
        const g = this.guild(t);
        const role = await this.roleId(g, String(a.args[1]));
        const m = await r.request<Raw>("GET", `/guilds/${g}/members/${this.str(a, 0)}`);
        return (m["roles"] as string[]).includes(role);
      }
      case "channel":
        return this.findChannel(t, String(a.args[0] ?? ""));
      case "role":
        return this.findRole(t, String(a.args[0] ?? ""));
      case "member":
        return this.findMember(t, String(a.args[0] ?? ""));
      default:
        throw new HostError(`ação desconhecida: ${a.name}`);
    }
  }

  private async reply(a: Action, t: RunTarget, state: RunState): Promise<unknown> {
    const body = messageBody(a.args[0], a.options);
    const i = t.interaction;
    if (i) {
      if (!state.responded) {
        state.responded = true;
        await this.rest.request("POST", `/interactions/${i.id}/${i.token}/callback`, { body: { type: 4, data: body }, noAuth: true });
        return null;
      }
      const m = await this.rest.request<Raw>("POST", `/webhooks/${i.applicationId}/${i.token}`, { body, noAuth: true });
      return toMessage(m);
    }
    if (!t.channelId) throw new HostError("não há canal para responder");
    if (t.messageId) body["message_reference"] = { message_id: t.messageId, fail_if_not_exists: false };
    const m = await this.rest.request<Raw>("POST", `/channels/${t.channelId}/messages`, { body });
    return toMessage(m);
  }

  private async roleId(guildId: string, query: string): Promise<string> {
    const id = idOf(query);
    if (id) return id;
    const roles = await this.cache.serverRoles(guildId);
    const found = roles.find((x) => String(x["name"]).toLowerCase() === query.toLowerCase());
    if (!found) throw new HostError(`não achei o cargo "${query}"`);
    return String(found["id"]);
  }

  private async findChannel(t: RunTarget, query: string): Promise<unknown> {
    const id = idOf(query);
    if (!t.guildId) {
      return id ? toChannel(await this.rest.request<Raw>("GET", `/channels/${id}`)) : null;
    }
    const list = await this.cache.serverChannels(t.guildId);
    const q = query.replace(/^#/, "").toLowerCase();
    return list.find((c) => c["id"] === id || String(c["name"]).toLowerCase() === q) ?? null;
  }

  private async findRole(t: RunTarget, query: string): Promise<unknown> {
    const list = await this.cache.serverRoles(this.guild(t));
    const id = idOf(query);
    return list.find((x) => x["id"] === id || String(x["name"]).toLowerCase() === query.toLowerCase()) ?? null;
  }

  private async findMember(t: RunTarget, query: string): Promise<unknown> {
    const g = this.guild(t);
    const id = idOf(query);
    try {
      if (id) return toMember(await this.rest.request<Raw>("GET", `/guilds/${g}/members/${id}`));
      const found = await this.rest.request<Raw[]>("GET", `/guilds/${g}/members/search?query=${encodeURIComponent(query)}&limit=1`);
      return found[0] ? toMember(found[0]) : null;
    } catch (e) {
      if ((e as { status?: number }).status === 404) return null;
      throw e;
    }
  }
}
