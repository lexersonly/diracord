/**
 * `diracord run` and `diracord watch`.
 * @module
 */

import { watch as fsWatch } from "node:fs";
import { resolve } from "node:path";
import { ConfigError, getToken, loadProject, redact, SOURCE_EXTENSIONS, type Project } from "@diracord/config";
import { EngineClient, type LoadOptions, type LoadResult, type Manifest } from "@diracord/engine";
import { Bot } from "@diracord/discord";
import { dim, err, green, out, printError, useColor } from "../ui.ts";

function loadOptions(p: Project): LoadOptions {
  return {
    syntax: { dollar: p.config.syntax.dollar },
    data: { path: resolve(p.root, p.config.data.path) },
    defaultScope: p.config.data.defaultScope,
    limits: {
      maxSteps: p.config.limits.steps,
      maxTimeMs: p.config.limits.timeMs,
      maxMemoryKb: p.config.limits.memoryKb,
      maxActions: p.config.limits.actions,
    },
    color: useColor,
  };
}

function manifestOf(r: LoadResult): Manifest {
  return { commands: r.commands ?? [], events: r.events ?? [], interactions: r.interactions ?? [] };
}

/** Starts the bot. With `watch`, reloads code when files change. */
export async function run(dir: string, opts: { watch?: boolean } = {}): Promise<number> {
  let project = loadProject(dir);
  let token: string | null = null;
  try {
    token = getToken(project);
  } catch (e) {
    if (!(opts.watch && e instanceof ConfigError)) throw e;
  }
  const engine = await EngineClient.start({ cwd: project.root });
  const first = await engine.load(project.files, loadOptions(project));
  if (first.rendered) out(first.rendered.trimEnd());
  if (!first.ok && !opts.watch) {
    await engine.close();
    return 1;
  }
  let bot: Bot | null = null;
  if (token && first.ok) {
    out(dim(`token ${redact(token)}, ${project.files.length} arquivo(s)`));
    bot = new Bot({
      token,
      engine,
      manifest: manifestOf(first),
      prefix: project.config.bot.prefix,
      prefixCommands: project.config.commands.prefix,
      slashCommands: project.config.commands.slash,
      intents: project.config.discord.intents,
      log: (m) => out(m),
    });
    await bot.start();
  } else if (opts.watch) {
    out(dim("sem DISCORD_TOKEN: modo só checagem (o código é verificado a cada mudança)"));
  }
  const stop = async () => {
    bot?.stop();
    await engine.close();
    process.exit(0);
  };
  process.once("SIGINT", stop);
  process.once("SIGTERM", stop);
  if (!opts.watch) return new Promise(() => {});

  let timer: ReturnType<typeof setTimeout> | null = null;
  const reload = async () => {
    try {
      project = loadProject(project.root);
      const r = await engine.load(project.files, loadOptions(project));
      if (r.ok) {
        out(`${green("recarregado")}: ${project.files.length} arquivo(s)${r.rendered ? "" : ", nenhum problema"}`);
        if (r.rendered) out(r.rendered.trimEnd());
        if (bot) bot.setManifest(manifestOf(r));
        else if (token) {
          bot = new Bot({
            token,
            engine,
            manifest: manifestOf(r),
            prefix: project.config.bot.prefix,
            prefixCommands: project.config.commands.prefix,
            slashCommands: project.config.commands.slash,
            intents: project.config.discord.intents,
            log: (m) => out(m),
          });
          await bot.start();
        }
      } else {
        out(r.rendered.trimEnd());
        err(dim("o código antigo continua rodando até os erros serem corrigidos"));
      }
    } catch (e) {
      printError(e);
    }
  };
  fsWatch(project.root, { recursive: true }, (_ev, file) => {
    const name = String(file ?? "");
    if (!name.endsWith("diracord.toml") && !SOURCE_EXTENSIONS.some((x) => name.endsWith(x))) return;
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => void reload(), 150);
  });
  out(dim(`observando ${project.root} (Ctrl+C para sair)`));
  return new Promise(() => {});
}
