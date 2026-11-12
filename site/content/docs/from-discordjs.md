Se você vem do discord.js, as ideias são as mesmas: eventos, comandos, interações e builders. A diferença é que o Diracord cuida do cliente, do registro dos slash commands e do `async/await` por você.

## Equivalências

| discord.js | Diracord |
|---|---|
| `new Client({ intents })` + `client.login(token)` | `diracord.toml` + `.env`, e `diracord run` |
| `SlashCommandBuilder` + deploy com REST | `command("ban", user: member, reason: text = "x")` (registrado sozinho) |
| `interaction.options.getUser("user")` | o argumento `user` já é o membro |
| `interaction.reply(...)` / `message.reply(...)` | `reply ...` (vale para os dois) |
| `client.on("guildMemberAdd", ...)` | `memberJoin { ... }` |
| `client.on("messageCreate", ...)` | `messageCreate { ... }` |
| `new EmbedBuilder().setTitle(...)` | `reply { title: ... }` |
| `ContainerBuilder`, `TextDisplayBuilder` | `container(...)`, `text(...)` |
| `ActionRowBuilder` + `ButtonBuilder` | `row(button("id", "Label", style: "success"))` |
| `interaction.isButton() && customId === "x"` | `button("x") { ... }` |
| `guild.members.ban(user, { reason })` | `ban user reason` |
| `member.roles.add(role)` | `addRole user "Role name"` |
| `try { await ... } catch (err) { }` | `testing { ... } catch err { ... }` |
| um banco de dados à sua escolha | `get` / `set` com escopos (Dirabank) |
| funções JavaScript | `fn name(args) { ... }` |

## Components v2 lado a lado

discord.js:

```javascript
const container = new ContainerBuilder()
  .setAccentColor(0x9b59b6)
  .addTextDisplayComponents((t) => t.setContent("## Welcome"))
  .addActionRowComponents((row) =>
    row.setComponents(
      new ButtonBuilder().setCustomId("accept").setLabel("Accept").setStyle(ButtonStyle.Success),
    ),
  );
await interaction.reply({ components: [container], flags: MessageFlags.IsComponentsV2 });
```

Diracord:

```preview
{"messages": [{"from": "bot", "command": "/welcome", "layout": [{"container": [{"text": "## Welcome"}, {"row": [{"button": "accept", "label": "Accept", "style": "success"}]}], "color": "purple"}]}]}
```

```dcs
command("welcome") {
  reply container(
    text("## Welcome"),
    row(button("accept", "Accept", style: "success")),
    color: "purple"
  )
}
```

## O que você ganha

- **Sem `await`:** cada ação espera o Discord sozinha.
- **Prefixo e slash juntos:** um `command` só.
- **Erros antes de rodar** com seta e sugestão, e limites que impedem um laço infinito de derrubar o bot.
- **Menos memória:** o motor é Rust e o Diracord não usa discord.js; só o necessário fica em cache.

## O que o Diracord não faz (ainda)

- Voz e música.
- Acesso a qualquer rota da API: o Diracord cobre o que as [funções embutidas](/docs/reference-functions) cobrem.
- Sharding para bots em milhares de servidores.

Se o seu bot depende disso, discord.js continua sendo a escolha certa.
