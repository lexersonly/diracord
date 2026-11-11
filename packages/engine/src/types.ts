/**
 * Types of the engine protocol (see `docs/engine-protocol.md`).
 * @module
 */

/** A source file sent to the engine. */
export interface SourceFile {
  /** Path relative to the project root, with `/`. */
  path: string;
  /** Full text. */
  source: string;
}

/** An automatic fix. Only `safe` fixes are applied by `diracord check --fix`. */
export interface Fix {
  message: string;
  safe: boolean;
  edits: { start: number; end: number; replacement: string }[];
}

/** One problem found in the code. */
export interface Diagnostic {
  /** Stable code, e.g. `E0201`. */
  code: string;
  /** `"erro"`, `"aviso"` or `"nota"`. */
  severity: string;
  message: string;
  file: string;
  line: number;
  col: number;
  endLine: number;
  endCol: number;
  help: string[];
  notes: string[];
  fixes: Fix[];
  /** Ready-to-print text with the code excerpt and arrow. */
  rendered: string;
}

/** Answer to `check`. */
export interface CheckResult {
  ok: boolean;
  errors: number;
  warnings: number;
  diagnostics: Diagnostic[];
  /** Every diagnostic plus a summary line ("1 erro"). Empty when clean. */
  rendered: string;
}

/** Answer to `fix`. */
export interface FixResult extends CheckResult {
  files: { path: string; source: string; changed: boolean }[];
}

/** A command argument as declared in code. */
export interface CommandParam {
  name: string;
  type: "text" | "number" | "bool" | "member" | "user" | "channel" | "role";
  optional: boolean;
}

/** What a loaded project defines. */
export interface Manifest {
  commands: { name: string; params: CommandParam[] }[];
  events: string[];
  interactions: { kind: "button" | "menu" | "modal"; id: string }[];
}

/** Answer to `load`. */
export interface LoadResult extends Partial<Manifest> {
  ok: boolean;
  diagnostics: Diagnostic[];
  rendered: string;
}

/** Options for `load`. */
export interface LoadOptions {
  syntax?: { dollar?: boolean };
  /** Dirabank file; omitted = in memory. */
  data?: { path: string } | null;
  defaultScope?: "user" | "server" | "member" | "global";
  limits?: { maxSteps?: number; maxTimeMs?: number; maxMemoryKb?: number; maxActions?: number };
  color?: boolean;
}

/** What to run. */
export type Trigger =
  | { kind: "command"; name: string; args?: Record<string, unknown> }
  | { kind: "event"; name: string }
  | { kind: "interaction"; type: "button" | "menu" | "modal"; id: string };

/** Automatic variables for a run. Entities are plain objects with at least `id`. */
export interface RunContext {
  user?: unknown;
  server?: unknown;
  channel?: unknown;
  message?: unknown;
  isInteraction?: boolean;
  extra?: Record<string, unknown>;
}

/** Something the engine asks the host to do on Discord. */
export interface Action {
  name: string;
  args: unknown[];
  options: Record<string, unknown>;
}

/** Performs an action; resolve with the result value, reject to report failure. */
export type ActionHandler = (action: Action) => Promise<unknown>;

/** How a run ended. */
export type DispatchResult =
  | { ok: true; steps: number; actions: number; value: unknown }
  | { ok: false; error: { code: string; message: string; file: string; rendered: string } };
