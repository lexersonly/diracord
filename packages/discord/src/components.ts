/**
 * Components v2: the engine's layout object (`{ layout: [...] }`, already
 * validated in Rust) → Discord component JSON.
 * @module
 */

import { color } from "./content.ts";

type Obj = Record<string, unknown>;

/** Message flag that turns on Components v2. */
export const IS_COMPONENTS_V2 = 1 << 15;

const T = { ROW: 1, BUTTON: 2, SELECT: 3, SECTION: 9, TEXT: 10, THUMBNAIL: 11, GALLERY: 12, FILE: 13, SEPARATOR: 14, CONTAINER: 17 } as const;
const STYLES: Record<string, number> = { primary: 1, secondary: 2, success: 3, danger: 4, link: 5 };

const str = (v: unknown) => (v == null ? undefined : String(v));

function media(v: unknown): Obj {
  if (typeof v === "string") return { media: { url: v } };
  const o = (v ?? {}) as Obj;
  const out: Obj = { media: { url: String(o["url"] ?? "") } };
  if (o["description"] != null) out["description"] = String(o["description"]);
  if (o["spoiler"]) out["spoiler"] = true;
  return out;
}

/** Discord emoji object from `"👍"` or `"<:name:id>"`. */
export function emoji(e: unknown): Obj | undefined {
  if (e == null) return undefined;
  const s = String(e);
  const m = /^<(a?):(\w+):(\d+)>$/.exec(s);
  return m ? { id: m[3], name: m[2], animated: m[1] === "a" } : { name: s };
}

function button(b: Obj): Obj {
  const out: Obj = { type: T.BUTTON };
  if (b["url"] != null) {
    out["style"] = STYLES["link"];
    out["url"] = String(b["url"]);
  } else {
    out["style"] = STYLES[String(b["style"] ?? "primary")] ?? 1;
    out["custom_id"] = String(b["button"]);
  }
  if (b["label"] != null) out["label"] = String(b["label"]);
  const e = emoji(b["emoji"]);
  if (e) out["emoji"] = e;
  if (b["disabled"]) out["disabled"] = true;
  return out;
}

function menu(m: Obj): Obj {
  const options = ((m["options"] as unknown[]) ?? []).map((o) => {
    if (o && typeof o === "object") {
      const x = o as Obj;
      const out: Obj = { label: String(x["label"]), value: String(x["value"] ?? x["label"]) };
      if (x["description"] != null) out["description"] = String(x["description"]);
      const e = emoji(x["emoji"]);
      if (e) out["emoji"] = e;
      if (x["default"]) out["default"] = true;
      return out;
    }
    return { label: String(o), value: String(o) };
  });
  const out: Obj = { type: T.SELECT, custom_id: String(m["menu"]), options };
  if (m["placeholder"] != null) out["placeholder"] = String(m["placeholder"]);
  if (m["min"] != null) out["min_values"] = Number(m["min"]);
  if (m["max"] != null) out["max_values"] = Number(m["max"]);
  return out;
}

function text(v: unknown): Obj {
  const s = v && typeof v === "object" ? (v as Obj)["text"] : v;
  return { type: T.TEXT, content: str(s) ?? "" };
}

/** One layout item → one Discord component. */
export function component(item: Obj): Obj {
  if ("container" in item) {
    const out: Obj = { type: T.CONTAINER, components: (item["container"] as Obj[]).map(component) };
    const c = color(item["color"]);
    if (c !== undefined) out["accent_color"] = c;
    if (item["spoiler"]) out["spoiler"] = true;
    return out;
  }
  if ("section" in item) {
    const accessory = "thumbnail" in item ? { type: T.THUMBNAIL, ...media(item["thumbnail"]) } : button(item["button"] as Obj);
    return { type: T.SECTION, components: (item["section"] as unknown[]).map(text), accessory };
  }
  if ("row" in item) {
    const comps = (item["row"] as Obj[]).map((x) => ("menu" in x ? menu(x) : button(x)));
    return { type: T.ROW, components: comps };
  }
  if ("text" in item) return text(item["text"]);
  if ("separator" in item) {
    const out: Obj = { type: T.SEPARATOR };
    if (item["spacing"] != null) out["spacing"] = item["spacing"] === "large" ? 2 : 1;
    if (item["line"] != null) out["divider"] = Boolean(item["line"]);
    return out;
  }
  if ("gallery" in item) return { type: T.GALLERY, items: (item["gallery"] as unknown[]).map(media) };
  if ("file" in item) {
    const out: Obj = { type: T.FILE, file: { url: String(item["file"]) } };
    if (item["spoiler"]) out["spoiler"] = true;
    return out;
  }
  throw new Error("componente desconhecido no layout");
}

/** `{ layout: [...] }` → `{ components, flags }` for a message body. */
export function layoutBody(content: Obj): { components: Obj[]; flags: number } {
  return { components: (content["layout"] as Obj[]).map(component), flags: IS_COMPONENTS_V2 };
}
