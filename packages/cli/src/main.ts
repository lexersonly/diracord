#!/usr/bin/env node
/**
 * The `diracord` command.
 * @module
 */

import { check } from "./commands/check.ts";
import { create } from "./commands/new.ts";
import { run } from "./commands/run.ts";
import { bold, out, printError } from "./ui.ts";

const HELP = `${bold("diracord")}: crie bots de Discord com uma linguagem simples

uso:
  diracord new <pasta>     cria um projeto novo
  diracord check [--fix]   mostra os erros do código (--fix corrige o que for seguro)
  diracord run             liga o bot
  diracord watch           liga o bot e recarrega o código ao salvar
  diracord help            esta ajuda

opções:
  --dir <pasta>            pasta do projeto (padrão: a atual)
`;

/** Runs the CLI with `argv` (without `node` and script). Returns the exit code. */
export async function main(argv: string[]): Promise<number> {
  const args = [...argv];
  let dir = process.cwd();
  const d = args.indexOf("--dir");
  if (d >= 0) {
    dir = args[d + 1] ?? dir;
    args.splice(d, 2);
  }
  const [cmd, ...rest] = args;
  try {
    switch (cmd) {
      case "new":
        return create(rest[0], dir);
      case "check":
        return await check(dir, { fix: rest.includes("--fix") });
      case "run":
        return await run(dir);
      case "watch":
        return await run(dir, { watch: true });
      case undefined:
      case "help":
      case "--help":
      case "-h":
        out(HELP);
        return 0;
      case "--version":
      case "-V":
        out("diracord 0.1.1");
        return 0;
      default:
        out(`comando desconhecido: ${cmd}\n`);
        out(HELP);
        return 2;
    }
  } catch (e) {
    printError(e);
    return 1;
  }
}

if (import.meta.main) {
  process.exitCode = await main(process.argv.slice(2));
}
