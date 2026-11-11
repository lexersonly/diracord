/**
 * Terminal output helpers (colors respect `NO_COLOR` and non-TTY output).
 * @module
 */

/** `true` when colors should be used on stdout. */
export const useColor = process.stdout.isTTY === true && !process.env["NO_COLOR"];

const paint = (code: string) => (s: string) => (useColor ? `\x1b[${code}m${s}\x1b[0m` : s);
export const bold = paint("1");
export const red = paint("1;31");
export const green = paint("1;32");
export const yellow = paint("1;33");
export const dim = paint("2");

/** Prints to stdout. */
export function out(s = ""): void {
  process.stdout.write(`${s}\n`);
}

/** Prints to stderr. */
export function err(s = ""): void {
  process.stderr.write(`${s}\n`);
}

/** Prints an error that has a `render()` (config errors) or a message with a code. */
export function printError(e: unknown): void {
  if (e && typeof e === "object" && "render" in e && typeof (e as { render: unknown }).render === "function") {
    err((e as { render(): string }).render());
  } else if (e && typeof e === "object" && "code" in e && "message" in e) {
    const { code, message } = e as { code: unknown; message: unknown };
    err(`${red(`erro[${String(code)}]`)}: ${String(message)}`);
  } else {
    err(`${red("erro")}: ${e instanceof Error ? e.message : String(e)}`);
  }
}
