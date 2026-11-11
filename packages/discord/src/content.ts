/**
 * Turns `reply` content (text or embed object) into a Discord message body.
 * @module
 */

const COLORS: Record<string, number> = {
  red: 0xed4245,
  green: 0x57f287,
  blue: 0x3498db,
  blurple: 0x5865f2,
  yellow: 0xfee75c,
  orange: 0xe67e22,
  purple: 0x9b59b6,
  pink: 0xeb459e,
  white: 0xffffff,
  black: 0x23272a,
  gray: 0x95a5a6,
  grey: 0x95a5a6,
};

/** Accepts `"red"`, `"#ff0000"`, `"ff0000"` or a number. Unknown → `undefined`. */
export function color(c: unknown): number | undefined {
  if (typeof c === "number") return c;
  if (typeof c !== "string") return undefined;
  const k = c.trim().toLowerCase();
  if (k in COLORS) return COLORS[k];
  const hex = k.replace(/^#/, "");
  return /^[0-9a-f]{6}$/.test(hex) ? parseInt(hex, 16) : undefined;
}

type Obj = Record<string, unknown>;
const asText = (v: unknown) => (v == null ? undefined : String(v));
const asUrl = (v: unknown) => (typeof v === "string" ? { url: v } : (v as Obj | undefined));

/** Embed object written in Diracord → Discord embed. */
export function embed(e: Obj): Obj {
  const out: Obj = {};
  if (e["title"] != null) out["title"] = asText(e["title"]);
  if (e["description"] != null) out["description"] = asText(e["description"]);
  if (e["url"] != null) out["url"] = asText(e["url"]);
  const c = color(e["color"]);
  if (c !== undefined) out["color"] = c;
  if (e["footer"] != null) out["footer"] = typeof e["footer"] === "object" ? e["footer"] : { text: asText(e["footer"]) };
  if (e["author"] != null) out["author"] = typeof e["author"] === "object" ? e["author"] : { name: asText(e["author"]) };
  if (e["image"] != null) out["image"] = asUrl(e["image"]);
  if (e["thumbnail"] != null) out["thumbnail"] = asUrl(e["thumbnail"]);
  if (e["timestamp"] === true) out["timestamp"] = new Date().toISOString();
  else if (typeof e["timestamp"] === "string") out["timestamp"] = e["timestamp"];
  if (Array.isArray(e["fields"])) {
    out["fields"] = (e["fields"] as Obj[]).map((f) => ({ name: asText(f["name"]) ?? "", value: asText(f["value"]) ?? "", inline: !!f["inline"] }));
  }
  return out;
}

import { layoutBody } from "./components.ts";

/**
 * Message body for `reply` / `send`. Text, an embed object, or a Components v2
 * layout (`{ layout: [...] }`). Mentions are limited to users so code
 * can never ping @everyone by accident.
 */
export function messageBody(content: unknown, options: Obj = {}): Obj {
  const body: Obj = { allowed_mentions: { parse: ["users"] } };
  let flags = options["ephemeral"] ? 64 : 0;
  if (content && typeof content === "object" && !Array.isArray(content) && "layout" in content) {
    const l = layoutBody(content as Obj);
    body["components"] = l.components;
    flags |= l.flags;
  } else if (content && typeof content === "object" && !Array.isArray(content)) body["embeds"] = [embed(content as Obj)];
  else body["content"] = String(content ?? "").slice(0, 2000);
  if (flags) body["flags"] = flags;
  return body;
}
