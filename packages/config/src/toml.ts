/**
 * A small TOML reader: enough for `diracord.toml`, with no dependencies.
 *
 * Supported: comments, `[table]` and `[a.b]` headers, bare/quoted keys,
 * dotted keys, basic `"..."` and literal `'...'` strings, integers (with `_`),
 * floats, booleans, arrays (also multi-line) and inline tables.
 * Not supported (not needed here): dates, multi-line strings, `[[arrays of tables]]`.
 *
 * @module
 */

/** A parsed TOML value. */
export type TomlValue = string | number | boolean | TomlValue[] | TomlTable;
/** A TOML table. */
export interface TomlTable {
  [key: string]: TomlValue;
}

/** A syntax error, with a 1-based line and column. */
export class TomlError extends Error {
  readonly line: number;
  readonly col: number;
  constructor(message: string, line: number, col: number) {
    super(`${message} (linha ${line}, coluna ${col})`);
    this.name = "TomlError";
    this.line = line;
    this.col = col;
  }
}

/** Parses TOML text into a plain object. Throws {@link TomlError}. */
export function parseToml(text: string): TomlTable {
  return new Parser(text).document();
}

class Parser {
  private i = 0;
  private readonly s: string;
  constructor(s: string) {
    this.s = s;
  }

  private fail(message: string): never {
    const before = this.s.slice(0, this.i);
    const line = before.split("\n").length;
    const col = this.i - before.lastIndexOf("\n");
    throw new TomlError(message, line, col);
  }

  private peek(): string {
    return this.s[this.i] ?? "";
  }

  /** Spaces and tabs (not newlines). */
  private spaces(): void {
    while (this.peek() === " " || this.peek() === "\t") this.i++;
  }

  /** Spaces, newlines and comments. */
  private blank(): void {
    for (;;) {
      const c = this.peek();
      if (c === " " || c === "\t" || c === "\r" || c === "\n") this.i++;
      else if (c === "#") while (this.i < this.s.length && this.peek() !== "\n") this.i++;
      else return;
    }
  }

  private endOfLine(): void {
    this.spaces();
    if (this.peek() === "#") while (this.i < this.s.length && this.peek() !== "\n") this.i++;
    if (this.peek() === "\r") this.i++;
    if (this.i < this.s.length && this.peek() !== "\n") this.fail(`esperava o fim da linha, encontrei '${this.peek()}'`);
  }

  document(): TomlTable {
    const root: TomlTable = {};
    let current = root;
    for (;;) {
      this.blank();
      if (this.i >= this.s.length) return root;
      if (this.peek() === "[") {
        this.i++;
        if (this.peek() === "[") this.fail("[[tabelas em lista]] não são suportadas");
        this.spaces();
        const path = this.keyPath();
        this.spaces();
        if (this.peek() !== "]") this.fail("faltou fechar o ] da tabela");
        this.i++;
        this.endOfLine();
        current = this.table(root, path);
        continue;
      }
      const path = this.keyPath();
      this.spaces();
      if (this.peek() !== "=") this.fail(`faltou '=' depois de '${path.join(".")}'`);
      this.i++;
      this.spaces();
      const value = this.value();
      this.assign(current, path, value);
      this.endOfLine();
    }
  }

  private table(root: TomlTable, path: string[]): TomlTable {
    let t = root;
    for (const k of path) {
      const next = t[k];
      if (next === undefined) {
        const n: TomlTable = {};
        t[k] = n;
        t = n;
      } else if (typeof next === "object" && !Array.isArray(next)) {
        t = next;
      } else {
        this.fail(`'${k}' já é um valor, não uma tabela`);
      }
    }
    return t;
  }

  private assign(t: TomlTable, path: string[], value: TomlValue): void {
    const last = path[path.length - 1] as string;
    const parent = this.table(t, path.slice(0, -1));
    if (last in parent) this.fail(`a chave '${path.join(".")}' aparece duas vezes`);
    parent[last] = value;
  }

  private keyPath(): string[] {
    const parts = [this.key()];
    for (;;) {
      this.spaces();
      if (this.peek() !== ".") return parts;
      this.i++;
      this.spaces();
      parts.push(this.key());
    }
  }

  private key(): string {
    const c = this.peek();
    if (c === '"') return this.basicString();
    if (c === "'") return this.literalString();
    const start = this.i;
    while (/[A-Za-z0-9_-]/.test(this.peek())) this.i++;
    if (start === this.i) this.fail("esperava o nome de uma chave");
    return this.s.slice(start, this.i);
  }

  private value(): TomlValue {
    const c = this.peek();
    if (c === '"') return this.basicString();
    if (c === "'") return this.literalString();
    if (c === "[") return this.array();
    if (c === "{") return this.inlineTable();
    if (this.s.startsWith("true", this.i)) {
      this.i += 4;
      return true;
    }
    if (this.s.startsWith("false", this.i)) {
      this.i += 5;
      return false;
    }
    const m = /^[+-]?(\d[\d_]*)(\.\d[\d_]*)?([eE][+-]?\d+)?/.exec(this.s.slice(this.i));
    if (!m) this.fail("valor inválido (use \"texto\", número, true/false ou [lista])");
    this.i += m[0].length;
    const n = Number(m[0].replaceAll("_", ""));
    if (!Number.isFinite(n)) this.fail("número inválido");
    return n;
  }

  private basicString(): string {
    this.i++;
    let out = "";
    for (;;) {
      const c = this.peek();
      if (c === "" || c === "\n") this.fail("texto sem fechar aspas");
      this.i++;
      if (c === '"') return out;
      if (c !== "\\") {
        out += c;
        continue;
      }
      const e = this.peek();
      this.i++;
      const map: Record<string, string> = { n: "\n", t: "\t", r: "\r", '"': '"', "\\": "\\", b: "\b", f: "\f" };
      if (e in map) out += map[e];
      else if (e === "u" || e === "U") {
        const len = e === "u" ? 4 : 8;
        const hex = this.s.slice(this.i, this.i + len);
        if (!/^[0-9a-fA-F]+$/.test(hex) || hex.length !== len) this.fail("escape \\u inválido");
        out += String.fromCodePoint(parseInt(hex, 16));
        this.i += len;
      } else this.fail(`escape inválido: \\${e}`);
    }
  }

  private literalString(): string {
    this.i++;
    const end = this.s.indexOf("'", this.i);
    const nl = this.s.indexOf("\n", this.i);
    if (end < 0 || (nl >= 0 && nl < end)) this.fail("texto sem fechar aspas");
    const out = this.s.slice(this.i, end);
    this.i = end + 1;
    return out;
  }

  private array(): TomlValue[] {
    this.i++;
    const out: TomlValue[] = [];
    for (;;) {
      this.blank();
      if (this.peek() === "]") {
        this.i++;
        return out;
      }
      out.push(this.value());
      this.blank();
      if (this.peek() === ",") this.i++;
      else if (this.peek() !== "]") this.fail("esperava ',' ou ']' na lista");
    }
  }

  private inlineTable(): TomlTable {
    this.i++;
    const t: TomlTable = {};
    this.spaces();
    if (this.peek() === "}") {
      this.i++;
      return t;
    }
    for (;;) {
      this.spaces();
      const path = this.keyPath();
      this.spaces();
      if (this.peek() !== "=") this.fail("faltou '=' na tabela");
      this.i++;
      this.spaces();
      this.assign(t, path, this.value());
      this.spaces();
      if (this.peek() === "}") {
        this.i++;
        return t;
      }
      if (this.peek() !== ",") this.fail("esperava ',' ou '}'");
      this.i++;
    }
  }
}
