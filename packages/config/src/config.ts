/**
 * Reads a Diracord project: `diracord.toml`, `.env` and the source files.
 * @module
 */

import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { parseEnv } from "./env.ts";
import { parseToml, TomlError, type TomlTable, type TomlValue } from "./toml.ts";

/** Source file extensions (SPEC §3.6). */
export const SOURCE_EXTENSIONS = [".dcs", ".dira", ".diracord"] as const;

/** Data scopes for `get`/`set`. */
export type Scope = "user" | "server" | "member" | "global";

/** Gateway intents the bot asks Discord for. */
export type Intent = "guilds" | "guildMembers" | "guildMessages" | "guildMessageReactions" | "guildVoiceStates" | "messageContent" | "directMessages";

/** Everything in `diracord.toml`, with defaults filled in. */
export interface DiracordConfig {
  /** `[bot]` */
  bot: { name: string; prefix: string };
  /** `[commands]`: which command modes are on. */
  commands: { prefix: boolean; slash: boolean };
  /** `[syntax]`: optional syntaxes. */
  syntax: { dollar: boolean };
  /** `[data]`: Dirabank. */
  data: { path: string; defaultScope: Scope };
  /** `[limits]`: per-run limits. */
  limits: { timeMs: number; steps: number; memoryKb: number; actions: number };
  /** `[discord]` */
  discord: { intents: Intent[] };
  /** Top-level `ram = "100mb"`: memory budget hint (informative for now). */
  ram: string | null;
}

/** A configuration problem with a stable code (`E0500`...). */
export class ConfigError extends Error {
  readonly code: string;
  readonly file: string | null;
  readonly line: number | null;
  readonly help: string | null;
  constructor(code: string, message: string, opts: { file?: string; line?: number; help?: string } = {}) {
    super(message);
    this.name = "ConfigError";
    this.code = code;
    this.file = opts.file ?? null;
    this.line = opts.line ?? null;
    this.help = opts.help ?? null;
  }

  /** Human-readable text in the same style as the engine's errors. */
  render(): string {
    let out = `erro[${this.code}]: ${this.message}\n`;
    if (this.file) out += ` --> ${this.file}${this.line ? `:${this.line}` : ""}\n`;
    if (this.help) out += `  = ajuda: ${this.help}\n`;
    return out;
  }
}

/** A loaded project. */
export interface Project {
  /** Folder with `diracord.toml`. */
  root: string;
  /** Parsed configuration. */
  config: DiracordConfig;
  /** Variables from `.env` (merged over `process.env` by the caller if wanted). */
  env: Record<string, string>;
  /** Source files, paths relative to `root` with `/` separators, sorted. */
  files: { path: string; source: string }[];
}

const DEFAULT_INTENTS: Intent[] = ["guilds", "guildMessages", "messageContent", "guildMembers"];
const ALL_INTENTS: Intent[] = ["guilds", "guildMembers", "guildMessages", "guildMessageReactions", "guildVoiceStates", "messageContent", "directMessages"];
const SCOPES: Scope[] = ["user", "server", "member", "global"];

/** Walks up from `start` to find the folder containing `diracord.toml`. */
export function findRoot(start: string): string | null {
  let dir = resolve(start);
  for (;;) {
    if (existsSync(join(dir, "diracord.toml"))) return dir;
    const up = dirname(dir);
    if (up === dir) return null;
    dir = up;
  }
}

/** Turns TOML into a {@link DiracordConfig}, checking types. */
export function readConfig(t: TomlTable, file = "diracord.toml"): DiracordConfig {
  const table = (name: string): TomlTable => {
    const v = t[name];
    if (v === undefined) return {};
    if (typeof v !== "object" || Array.isArray(v)) throw bad(`[${name}] precisa ser uma tabela`);
    return v;
  };
  const bad = (message: string, help?: string) => new ConfigError("E0501", message, { file, ...(help ? { help } : {}) });
  const str = (tb: TomlTable, k: string, d: string, where: string): string => {
    const v = tb[k];
    if (v === undefined) return d;
    if (typeof v !== "string") throw bad(`${where}.${k} precisa ser um texto entre aspas`);
    return v;
  };
  const bool = (tb: TomlTable, k: string, d: boolean, where: string): boolean => {
    const v = tb[k];
    if (v === undefined) return d;
    if (typeof v !== "boolean") throw bad(`${where}.${k} precisa ser true ou false`);
    return v;
  };
  const num = (tb: TomlTable, k: string, d: number, where: string): number => {
    const v = tb[k];
    if (v === undefined) return d;
    if (typeof v !== "number" || v <= 0 || !Number.isInteger(v)) throw bad(`${where}.${k} precisa ser um número inteiro maior que 0`);
    return v;
  };
  const known = new Set(["bot", "commands", "syntax", "data", "limits", "discord", "ram"]);
  for (const k of Object.keys(t)) {
    if (!known.has(k)) throw bad(`seção desconhecida: ${k}`, `seções: ${[...known].join(", ")}`);
  }
  const bot = table("bot");
  const commands = table("commands");
  const syntax = table("syntax");
  const data = table("data");
  const limits = table("limits");
  const discord = table("discord");
  const scope = str(data, "default_scope", "user", "data") as Scope;
  if (!SCOPES.includes(scope)) throw bad(`data.default_scope inválido: ${scope}`, "use user, server, member ou global");
  const intentsRaw: TomlValue = discord["intents"] ?? DEFAULT_INTENTS;
  if (!Array.isArray(intentsRaw) || !intentsRaw.every((x) => typeof x === "string")) throw bad("discord.intents precisa ser uma lista de textos");
  for (const i of intentsRaw) {
    if (!ALL_INTENTS.includes(i as Intent)) throw bad(`intent desconhecida: ${String(i)}`, `intents: ${ALL_INTENTS.join(", ")}`);
  }
  const ram = t["ram"];
  if (ram !== undefined && (typeof ram !== "string" || !/^\d+\s*(mb|gb)$/i.test(ram))) throw bad('ram precisa ser algo como "100mb"');
  const config: DiracordConfig = {
    bot: { name: str(bot, "name", "meu-bot", "bot"), prefix: str(bot, "prefix", "!", "bot") },
    commands: { prefix: bool(commands, "prefix", true, "commands"), slash: bool(commands, "slash", true, "commands") },
    syntax: { dollar: bool(syntax, "dollar", false, "syntax") },
    data: { path: str(data, "path", "data/bot.dirabank", "data"), defaultScope: scope },
    limits: {
      timeMs: num(limits, "time_ms", 3000, "limits"),
      steps: num(limits, "steps", 100000, "limits"),
      memoryKb: num(limits, "memory_kb", 4096, "limits"),
      actions: num(limits, "actions", 50, "limits"),
    },
    discord: { intents: intentsRaw as Intent[] },
    ram: typeof ram === "string" ? ram.toLowerCase().replace(/\s+/g, "") : null,
  };
  if (config.bot.prefix.length === 0) throw bad("bot.prefix não pode ser vazio");
  if (!config.commands.prefix && !config.commands.slash) throw bad("ligue pelo menos um modo em [commands]: prefix ou slash");
  return config;
}

/** Lists source files under `root`, skipping hidden folders, `node_modules` and the data folder. */
export function listSources(root: string, skip: string[] = []): string[] {
  const out: string[] = [];
  const skipAbs = new Set(skip.map((s) => resolve(root, s)));
  const walk = (dir: string) => {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || name === "node_modules") continue;
      const full = join(dir, name);
      if (skipAbs.has(full)) continue;
      const st = statSync(full);
      if (st.isDirectory()) walk(full);
      else if (SOURCE_EXTENSIONS.some((e) => name.endsWith(e))) out.push(full);
    }
  };
  walk(root);
  return out.map((f) => relative(root, f).split(sep).join("/")).sort();
}

/** Loads the project that contains `dir`. Throws {@link ConfigError}. */
export function loadProject(dir: string): Project {
  const root = findRoot(dir);
  if (!root) {
    throw new ConfigError("E0500", "diracord.toml não encontrado", {
      help: "rode o comando dentro da pasta do bot, ou crie um projeto com `diracord new meu-bot`",
    });
  }
  const tomlPath = join(root, "diracord.toml");
  let table: TomlTable;
  try {
    table = parseToml(readFileSync(tomlPath, "utf8"));
  } catch (e) {
    if (e instanceof TomlError) {
      throw new ConfigError("E0501", `diracord.toml inválido: ${e.message}`, { file: "diracord.toml", line: e.line });
    }
    throw e;
  }
  const config = readConfig(table);
  const envPath = join(root, ".env");
  const env = existsSync(envPath) ? parseEnv(readFileSync(envPath, "utf8")) : {};
  const dataDir = dirname(config.data.path);
  const files = listSources(root, dataDir === "." ? [] : [dataDir]).map((path) => ({
    path,
    source: readFileSync(join(root, path), "utf8"),
  }));
  if (files.length === 0) {
    throw new ConfigError("E0503", "nenhum arquivo de código no projeto", {
      help: `crie um arquivo ${SOURCE_EXTENSIONS.join(", ")}, por exemplo commands/ping.dcs`,
    });
  }
  return { root, config, env, files };
}

/** The bot token from `.env` or the environment. Throws `E0502` if missing. */
export function getToken(project: Project, environment: NodeJS.ProcessEnv = process.env): string {
  const token = project.env["DISCORD_TOKEN"] ?? environment["DISCORD_TOKEN"];
  if (!token) {
    throw new ConfigError("E0502", "token não encontrado", {
      file: ".env",
      help: "coloque DISCORD_TOKEN=seu_token no arquivo .env (e nunca no código nem no git)",
    });
  }
  return token;
}
