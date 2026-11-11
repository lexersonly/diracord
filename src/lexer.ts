import { DiracordError } from "./errors.js";

export type Tok =
  | { k: "num"; v: number; line: number; col: number; end: number }
  | { k: "str"; parts: (string | { code: string; col: number })[]; line: number; col: number; end: number }
  | { k: "id"; v: string; line: number; col: number; end: number }
  | { k: "op"; v: string; line: number; col: number; end: number }
  /** aoi.js-style call: $name[arg;arg] — args are kept as raw text */
  | { k: "legacy"; name: string; args: string[]; line: number; col: number; end: number }
  | { k: "nl" | "indent" | "dedent" | "eof"; line: number; col: number; end: number };

const OPS3 = ["..."];
const OPS2 = ["==", "!=", "<=", ">=", "+=", "-=", "*=", "/=", "&&", "||"];
const OPS1 = "+-*/%<>=!().,[]:";

const isDigit = (c: string) => c >= "0" && c <= "9";
const isIdStart = (c: string) => /[A-Za-z_]/.test(c);
const isIdChar = (c: string) => /[A-Za-z0-9_]/.test(c);

/** Finds the `close` that matches an already-consumed `open`. Returns its index. */
function matchClose(s: string, from: number, open: string, close: string, line: number): number {
  let depth = 1;
  for (let i = from; i < s.length; i++) {
    if (s[i] === open) depth++;
    else if (s[i] === close && --depth === 0) return i;
  }
  throw new DiracordError(`Missing "${close}" — this "${open}" is never closed`, line, from);
}

/** Splits "a;b;$x[1;2]" on top-level semicolons only. */
export function splitLegacyArgs(raw: string): string[] {
  const out: string[] = [];
  let depth = 0;
  let cur = "";
  for (const c of raw) {
    if (c === "[") depth++;
    if (c === "]") depth--;
    if (c === ";" && depth === 0) {
      out.push(cur);
      cur = "";
    } else cur += c;
  }
  out.push(cur);
  return out;
}

/** Tokenizes one line of code (no indentation handling). */
export function lexLine(text: string, line: number, colBase = 0): Tok[] {
  const toks: Tok[] = [];
  let i = 0;
  while (i < text.length) {
    const c = text[i];
    const col = colBase + i;
    if (c === " " || c === "\t") { i++; continue; }
    if (c === "/" && text[i + 1] === "/") break; // comment

    if (isDigit(c)) {
      let j = i;
      while (j < text.length && isDigit(text[j])) j++;
      if (text[j] === "." && isDigit(text[j + 1] ?? "")) {
        j++;
        while (j < text.length && isDigit(text[j])) j++;
      }
      toks.push({ k: "num", v: Number(text.slice(i, j)), line, col, end: colBase + j });
      i = j;
      continue;
    }

    if (c === '"' || c === "'") {
      const quote = c;
      const parts: (string | { code: string; col: number })[] = [];
      let cur = "";
      let j = i + 1;
      let closed = false;
      while (j < text.length) {
        const d = text[j];
        if (d === "\\") {
          const n = text[j + 1];
          cur += n === "n" ? "\n" : n === "t" ? "\t" : (n ?? "");
          j += 2;
        } else if (d === quote) { closed = true; j++; break; }
        else if (d === "{") {
          const end = matchClose(text, j + 1, "{", "}", line);
          if (cur) parts.push(cur);
          cur = "";
          parts.push({ code: text.slice(j + 1, end), col: colBase + j + 1 });
          j = end + 1;
        } else { cur += d; j++; }
      }
      if (!closed) throw new DiracordError(`This text starts with ${quote} but never ends`, line, col);
      if (cur || parts.length === 0) parts.push(cur);
      toks.push({ k: "str", parts, line, col, end: colBase + j });
      i = j;
      continue;
    }

    if (c === "$") {
      let j = i + 1;
      if (!isIdStart(text[j] ?? "")) throw new DiracordError(`"$" must be followed by a name, like $reply[...] or $user`, line, col);
      while (j < text.length && isIdChar(text[j])) j++;
      const name = text.slice(i + 1, j);
      if (text[j] === "[") {
        const end = matchClose(text, j + 1, "[", "]", line);
        toks.push({ k: "legacy", name, args: splitLegacyArgs(text.slice(j + 1, end)), line, col, end: colBase + end + 1 });
        i = end + 1;
      } else {
        toks.push({ k: "id", v: name, line, col, end: colBase + j }); // $user == user
        i = j;
      }
      continue;
    }

    if (isIdStart(c)) {
      let j = i;
      while (j < text.length && isIdChar(text[j])) j++;
      toks.push({ k: "id", v: text.slice(i, j), line, col, end: colBase + j });
      i = j;
      continue;
    }

    const three = OPS3.find((o) => text.startsWith(o, i));
    const two = OPS2.find((o) => text.startsWith(o, i));
    const op = three ?? two ?? (OPS1.includes(c) ? c : undefined);
    if (op) {
      toks.push({ k: "op", v: op, line, col, end: colBase + i + op.length });
      i += op.length;
      continue;
    }
    throw new DiracordError(`I don't understand the character "${c}"`, line, col);
  }
  return toks;
}

/** Turns a whole file into tokens, with indent/dedent/newline markers. */
export function lex(source: string): Tok[] {
  const toks: Tok[] = [];
  const stack = [0];
  const lines = source.replace(/\r/g, "").split("\n");

  lines.forEach((raw, idx) => {
    const line = idx + 1;
    const expanded = raw.replace(/^\t+/, (t) => "  ".repeat(t.length)); // tab = 2 spaces
    let body = expanded.trimStart();
    if (body === "" || body.startsWith("//")) return;
    const indent = expanded.length - body.length;

    // `{reply "hi"}` on its own line is the same as `reply "hi"`
    body = body.trimEnd();
    let shift = 0;
    if (body.startsWith("{") && body.endsWith("}")) {
      body = body.slice(1, -1).trim();
      shift = 1;
    }

    const top = stack[stack.length - 1];
    if (indent > top) {
      stack.push(indent);
      toks.push({ k: "indent", line, col: 0, end: 0 });
    } else {
      while (indent < stack[stack.length - 1]) {
        stack.pop();
        toks.push({ k: "dedent", line, col: 0, end: 0 });
      }
      if (indent !== stack[stack.length - 1]) {
        throw new DiracordError("This line is indented differently from the lines above it", line);
      }
    }
    toks.push(...lexLine(body, line, indent + shift));
    toks.push({ k: "nl", line, col: body.length, end: body.length });
  });

  const last = lines.length;
  while (stack.length > 1) {
    stack.pop();
    toks.push({ k: "dedent", line: last, col: 0, end: 0 });
  }
  toks.push({ k: "eof", line: last, col: 0, end: 0 });
  return toks;
}
