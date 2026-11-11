/**
 * `diracord new <name>`: creates a ready-to-run project.
 * @module
 */

import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { bold, green, out } from "../ui.ts";

/** The files of a new project (no secrets: the token goes in `.env`, which is git-ignored). */
export function template(name: string): Record<string, string> {
  return {
    "diracord.toml": `# Configuração do bot. Documentação: docs/getting-started.md
ram = "100mb"

[bot]
name = "${name}"
prefix = "!"

[commands]
prefix = true   # !ping
slash = true    # /ping

[syntax]
dollar = false  # ligue para usar o atalho $ban[user;motivo]

[data]
path = "data/bot.dirabank"
default_scope = "user"

[limits]
time_ms = 3000
steps = 100000
memory_kb = 4096
actions = 50

[discord]
intents = ["guilds", "guildMessages", "messageContent", "guildMembers"]
`,
    ".env.example": "# Copie para .env e coloque o token do seu bot. NUNCA envie o .env para o git.\nDISCORD_TOKEN=\n",
    ".gitignore": ".env\ndata/\nnode_modules/\n",
    "commands/ping.dcs": `// Responde "Pong!" para !ping e /ping
command("ping") {
  reply "Pong!"
}
`,
    "commands/coins.dcs": `// Moedas por usuário, salvas no Dirabank
command("coins") {
  local coins = get("coins", 0, scope: user) + 1
  set("coins", coins, scope: user)
  reply "$user.name, você tem $coins moedas"
}
`,
    "events/join.dcs": `// Boas-vindas
memberJoin {
  local ch = channel("welcome")
  if ch {
    send ch "Bem-vindo, $user.mention!"
  }
}
`,
    "README.md": `# ${name}

Bot feito com [Diracord](https://github.com/lexersonly/diracord).

1. Copie \`.env.example\` para \`.env\` e coloque o token.
2. \`diracord check\` para ver se o código está certo.
3. \`diracord run\` para ligar o bot (ou \`diracord watch\` para recarregar ao salvar).
`,
  };
}

/** Creates `name/` with the template. Returns the exit code. */
export function create(name: string | undefined, cwd = process.cwd()): number {
  if (!name) {
    out("uso: diracord new <nome-da-pasta>");
    return 2;
  }
  const dir = resolve(cwd, name);
  if (existsSync(dir) && readdirSync(dir).length > 0) {
    out(`a pasta ${name} já existe e não está vazia`);
    return 1;
  }
  for (const [path, text] of Object.entries(template(basename(dir)))) {
    const full = join(dir, path);
    mkdirSync(join(full, ".."), { recursive: true });
    writeFileSync(full, text);
  }
  out(`${green("criado")}: ${name}/`);
  out(`\npróximos passos:\n  ${bold(`cd ${name}`)}\n  ${bold("cp .env.example .env")}   (e coloque o token)\n  ${bold("diracord run")}`);
  return 0;
}
