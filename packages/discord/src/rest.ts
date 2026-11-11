/**
 * Minimal Discord REST client over `fetch`, with rate-limit handling.
 * @module
 */

/** Discord API base URL (v10). */
export const API = "https://discord.com/api/v10";
const USER_AGENT = "DiscordBot (https://github.com/lexersonly/diracord, 0.1.0)";

/** A failed Discord API call. `message` is Discord's own text when available. */
export class DiscordError extends Error {
  readonly status: number;
  readonly discordCode: number | null;
  constructor(status: number, message: string, discordCode: number | null) {
    super(message);
    this.name = "DiscordError";
    this.status = status;
    this.discordCode = discordCode;
  }
}

/** The subset of `fetch` the client needs (injectable for tests). */
export type FetchLike = (url: string, init: { method: string; headers: Record<string, string>; body?: string }) => Promise<{
  status: number;
  headers: { get(name: string): string | null };
  text(): Promise<string>;
}>;

/** Options for one request. */
export interface RequestOptions {
  body?: unknown;
  /** Shown in the server's audit log. */
  reason?: string | null;
  /** Send without the bot token (interaction webhooks). */
  noAuth?: boolean;
}

/** REST client. Keeps only per-route bucket timers in memory. */
export class Rest {
  private readonly token: string;
  private readonly fetchImpl: FetchLike;
  private readonly waits = new Map<string, number>();
  private globalUntil = 0;

  constructor(token: string, fetchImpl: FetchLike = fetch as unknown as FetchLike) {
    this.token = token;
    this.fetchImpl = fetchImpl;
  }

  /** Calls `method path`. Resolves with the parsed JSON (or `null` for 204). */
  async request<T = unknown>(method: string, path: string, opts: RequestOptions = {}): Promise<T> {
    const route = `${method} ${bucketPath(path)}`;
    for (let attempt = 0; ; attempt++) {
      await sleepUntil(Math.max(this.globalUntil, this.waits.get(route) ?? 0));
      const headers: Record<string, string> = { "User-Agent": USER_AGENT };
      if (!opts.noAuth) headers["Authorization"] = `Bot ${this.token}`;
      if (opts.body !== undefined) headers["Content-Type"] = "application/json";
      if (opts.reason) headers["X-Audit-Log-Reason"] = encodeURIComponent(opts.reason.slice(0, 512));
      const init: { method: string; headers: Record<string, string>; body?: string } = { method, headers };
      if (opts.body !== undefined) init.body = JSON.stringify(opts.body);
      const res = await this.fetchImpl(`${API}${path}`, init);
      const remaining = res.headers.get("x-ratelimit-remaining");
      const resetAfter = Number(res.headers.get("x-ratelimit-reset-after") ?? "0");
      if (remaining === "0" && resetAfter > 0) this.waits.set(route, Date.now() + resetAfter * 1000);
      const text = await res.text();
      const data = text ? safeJson(text) : null;
      if (res.status === 429 && attempt < 3) {
        const retry = Number((data as { retry_after?: number } | null)?.retry_after ?? resetAfter ?? 1);
        const until = Date.now() + Math.max(retry, 0.05) * 1000;
        if ((data as { global?: boolean } | null)?.global) this.globalUntil = until;
        else this.waits.set(route, until);
        continue;
      }
      if (res.status >= 400) {
        const d = data as { message?: string; code?: number } | null;
        throw new DiscordError(res.status, d?.message ?? `HTTP ${res.status}`, d?.code ?? null);
      }
      return data as T;
    }
  }
}

/** Keeps the first id (channel/server: the "major parameter") and collapses the rest. */
export function bucketPath(path: string): string {
  let seen = false;
  return path.split("?")[0]!.replace(/\d{15,25}/g, (id) => {
    if (!seen) {
      seen = true;
      return id;
    }
    return ":id";
  });
}

function safeJson(text: string): unknown {
  try {
    return JSON.parse(text);
  } catch {
    return null;
  }
}

function sleepUntil(t: number): Promise<void> {
  const ms = t - Date.now();
  return ms > 0 ? new Promise((r) => setTimeout(r, ms)) : Promise.resolve();
}
