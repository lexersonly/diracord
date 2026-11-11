/**
 * Minimal cache: only channel and role *names* per server, fetched on demand,
 * expiring after a minute, and bounded in size. Nothing else is cached.
 * @module
 */

import type { Rest } from "./rest.ts";
import { channel as toChannel, role as toRole, type Raw } from "./entities.ts";

interface Entry {
  at: number;
  items: Raw[];
}

/** Small TTL + LRU cache of per-server lists. */
export class NameCache {
  private readonly rest: Rest;
  private readonly ttlMs: number;
  private readonly max: number;
  private readonly channels = new Map<string, Entry>();
  private readonly roles = new Map<string, Entry>();

  constructor(rest: Rest, opts: { ttlMs?: number; maxServers?: number } = {}) {
    this.rest = rest;
    this.ttlMs = opts.ttlMs ?? 60_000;
    this.max = opts.maxServers ?? 50;
  }

  private async get(map: Map<string, Entry>, key: string, load: () => Promise<Raw[]>): Promise<Raw[]> {
    const hit = map.get(key);
    if (hit && Date.now() - hit.at < this.ttlMs) {
      map.delete(key);
      map.set(key, hit);
      return hit.items;
    }
    const items = await load();
    map.set(key, { at: Date.now(), items });
    while (map.size > this.max) map.delete(map.keys().next().value as string);
    return items;
  }

  /** Channels of a server (id, name, mention, type). */
  serverChannels(guildId: string): Promise<Raw[]> {
    return this.get(this.channels, guildId, async () =>
      (await this.rest.request<Raw[]>("GET", `/guilds/${guildId}/channels`)).map(toChannel),
    );
  }

  /** Roles of a server (id, name, mention, color). */
  serverRoles(guildId: string): Promise<Raw[]> {
    return this.get(this.roles, guildId, async () => (await this.rest.request<Raw[]>("GET", `/guilds/${guildId}/roles`)).map(toRole));
  }

  /** Forgets a server (after channel/role create/delete events). */
  invalidate(guildId: string): void {
    this.channels.delete(guildId);
    this.roles.delete(guildId);
  }
}

/** `"123"`, `"<#123>"`, `"<@&123>"`, `"<@!123>"` → `"123"`; otherwise `null`. */
export function idOf(query: string): string | null {
  const m = /^<?[@#]?[!&]?(\d{15,25})>?$/.exec(query.trim());
  return m ? (m[1] as string) : null;
}
