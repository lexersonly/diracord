/** Test helpers: a fake `fetch` that records calls. */
import type { FetchLike } from "../src/index.ts";

export interface Call {
  method: string;
  url: string;
  headers: Record<string, string>;
  body: unknown;
}

export function fakeFetch(answer: (c: Call) => { status?: number; body?: unknown; headers?: Record<string, string> } = () => ({})) {
  const calls: Call[] = [];
  const fetch: FetchLike = async (url, init) => {
    const call: Call = { method: init.method, url, headers: init.headers, body: init.body ? JSON.parse(init.body) : undefined };
    calls.push(call);
    const a = answer(call);
    const status = a.status ?? (a.body === undefined ? 204 : 200);
    const headers = a.headers ?? {};
    return {
      status,
      headers: { get: (n: string) => headers[n.toLowerCase()] ?? null },
      text: async () => (a.body === undefined ? "" : JSON.stringify(a.body)),
    };
  };
  return { fetch, calls };
}
