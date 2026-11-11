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
    "commands/ping.dcs": `// Answers "Pong!" to !ping and /ping
command("ping") {
  reply "Pong!"
}
`,
    "commands/coins.dcs": `// Coins per user, saved in Dirabank. giveCoins is a function of your own (fn).
fn giveCoins(amount: number = 1) {
  local total = get("coins", 0, scope: user) + amount
  set("coins", total, scope: user)
  return total
}

command("coins") {
  local total = giveCoins()
  reply "$user.name, you have $total coins"
}
`,
    "commands/rules.dcs": `// Components v2: pieces go inside each other, like discord.js builders.
command("rules") {
  reply container(
    text("## Welcome, $user.name"),
    text("Be nice and have fun."),
    row(button("accept", "I agree", style: "success")),
    color: "blurple"
  )
}

// Runs when someone clicks the button with id "accept".
button("accept") {
  reply "Thanks, $user.name!" ephemeral: true
}
`,
    "events/join.dcs": `// Says hi in the #welcome channel when someone joins.
memberJoin {
  local ch = channel("welcome")
  if ch {
    send ch "Welcome, $user.mention!"
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
