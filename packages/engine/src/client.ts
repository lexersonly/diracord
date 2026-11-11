/**
 * Talks to the Rust engine process over JSON lines.
 * @module
 */

import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { createInterface, type Interface } from "node:readline";
import { findEngine } from "./find.ts";
import type {
  Action,
  ActionHandler,
  CheckResult,
  DispatchResult,
  FixResult,
  LoadOptions,
  LoadResult,
  RunContext,
  SourceFile,
  Trigger,
} from "./types.ts";

type Msg = Record<string, unknown> & { op?: string };

/** The engine could not be started or stopped answering (code `E0504`). */
export class EngineError extends Error {
  readonly code = "E0504";
  constructor(message: string) {
    super(message);
    this.name = "EngineError";
  }
}

/**
 * A running engine. Requests are queued and handled one at a time
 * (the protocol is sequential), so callers may fire them concurrently.
 */
export class EngineClient {
  private readonly child: ChildProcessWithoutNullStreams;
  private readonly lines: Interface;
  private waiter: ((m: Msg) => void) | null = null;
  private readonly backlog: Msg[] = [];
  private queue: Promise<unknown> = Promise.resolve();
  private exited: EngineError | null = null;
  private nextId = 1;
  /** Engine version from the `hello` message. */
  version = "";

  private constructor(child: ChildProcessWithoutNullStreams) {
    this.child = child;
    this.lines = createInterface({ input: child.stdout });
    this.lines.on("line", (line) => {
      if (!line.trim()) return;
      let m: Msg;
      try {
        m = JSON.parse(line) as Msg;
      } catch {
        return;
      }
      const w = this.waiter;
      if (w) {
        this.waiter = null;
        w(m);
      } else this.backlog.push(m);
    });
    child.stderr.on("data", (d: Buffer) => process.stderr.write(d));
    child.on("exit", (code) => {
      this.exited = new EngineError(`o motor parou (código ${code ?? "?"})`);
      const w = this.waiter;
      this.waiter = null;
      w?.({ op: "__exit__" });
    });
  }

  /** Starts the engine. `binary` defaults to {@link findEngine}. */
  static async start(opts: { binary?: string; cwd?: string } = {}): Promise<EngineClient> {
    const bin = opts.binary ?? findEngine(opts.cwd);
    if (!bin) {
      throw new EngineError(
        "motor do Diracord (diracord-engine) não encontrado: rode `cargo build --release -p diracord-engine` ou defina DIRACORD_ENGINE",
      );
    }
    const child = spawn(bin, [], { stdio: ["pipe", "pipe", "pipe"] });
    const client = new EngineClient(child);
    await new Promise<void>((ok, fail) => {
      child.once("error", (e) => fail(new EngineError(`não consegui iniciar o motor: ${e.message}`)));
      child.once("spawn", () => ok());
    });
    const hello = await client.next();
    if (hello.op !== "hello") throw new EngineError("o motor não respondeu com hello");
    client.version = String(hello["version"] ?? "");
    return client;
  }

  private next(): Promise<Msg> {
    const m = this.backlog.shift();
    if (m) return Promise.resolve(m);
    if (this.exited) return Promise.reject(this.exited);
    return new Promise((ok, fail) => {
      this.waiter = (msg) => (msg.op === "__exit__" ? fail(this.exited) : ok(msg));
    });
  }

  private send(m: Msg): void {
    if (this.exited) throw this.exited;
    this.child.stdin.write(`${JSON.stringify(m)}\n`);
  }

  private enqueue<T>(job: () => Promise<T>): Promise<T> {
    const run = this.queue.then(job, job);
    this.queue = run.catch(() => undefined);
    return run;
  }

  private async request<T>(m: Msg, expected: string): Promise<T> {
    this.send(m);
    const r = await this.next();
    if (r.op === "error") throw new EngineError(String(r["message"]));
    if (r.op !== expected) throw new EngineError(`resposta inesperada do motor: ${r.op}`);
    return r as unknown as T;
  }

  /** Checks files without loading them. */
  check(files: SourceFile[], opts: { dollar?: boolean; color?: boolean } = {}): Promise<CheckResult> {
    return this.enqueue(() =>
      this.request({ op: "check", files, syntax: { dollar: !!opts.dollar }, color: !!opts.color }, "checked"),
    );
  }

  /** Checks files and returns them with every safe fix applied. */
  fix(files: SourceFile[], opts: { dollar?: boolean } = {}): Promise<FixResult> {
    return this.enqueue(() => this.request({ op: "fix", files, syntax: { dollar: !!opts.dollar } }, "fixed"));
  }

  /** Loads a project (replacing the previous one) if it has no errors. */
  load(files: SourceFile[], options: LoadOptions = {}): Promise<LoadResult> {
    return this.enqueue(() => this.request({ op: "load", files, ...options }, "loaded"));
  }

  /**
   * Runs a trigger. Every action the code performs is passed to `onAction`;
   * its resolved value goes back to the code, a rejection becomes a catchable error.
   */
  dispatch(trigger: Trigger, context: RunContext, onAction: ActionHandler): Promise<DispatchResult> {
    return this.enqueue(async () => {
      const id = this.nextId++;
      this.send({ op: "dispatch", id, trigger, context });
      for (;;) {
        const m = await this.next();
        if (m.op === "error") throw new EngineError(String(m["message"]));
        if (m.op === "done" && m["id"] === id) return m as unknown as DispatchResult;
        if (m.op === "action" && m["id"] === id) {
          const seq = m["seq"];
          try {
            const value = await onAction(m["action"] as Action);
            this.send({ op: "result", id, seq, ok: true, value: value ?? null });
          } catch (e) {
            this.send({ op: "result", id, seq, ok: false, error: e instanceof Error ? e.message : String(e) });
          }
        }
      }
    });
  }

  /** Flushes data and stops the engine. */
  async close(): Promise<void> {
    if (this.exited) return;
    await this.enqueue(async () => {
      try {
        await this.request({ op: "shutdown" }, "bye");
      } catch {
        // already gone
      }
    });
    this.lines.close();
    this.child.stdin.end();
  }
}
