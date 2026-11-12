/** Code shown on the landing page. Diracord samples are checked by the repo tests. */

export const pingSample = `command("ping") {
  reply "Pong!"
}`;

export const banDiracord = `// Works as !ban @Joe spam and as /ban — same code.
command("ban", user: member, reason: text = "no reason") {
  testing {
    ban user reason
    reply "$user.name was banned. Reason: $reason"
  } catch err {
    reply "Could not ban: $err"
  }
}`;

export const banDiscordJs = `// commands/ban.js (discord.js v14) — slash command only.
const { SlashCommandBuilder, PermissionFlagsBits } = require("discord.js");

module.exports = {
  data: new SlashCommandBuilder()
    .setName("ban")
    .setDescription("Ban a member")
    .addUserOption((o) => o.setName("user").setDescription("Who").setRequired(true))
    .addStringOption((o) => o.setName("reason").setDescription("Why"))
    .setDefaultMemberPermissions(PermissionFlagsBits.BanMembers),

  async execute(interaction) {
    const user = interaction.options.getUser("user", true);
    const reason = interaction.options.getString("reason") ?? "no reason";
    try {
      await interaction.guild.members.ban(user, { reason });
      await interaction.reply(\`\${user.username} was banned. Reason: \${reason}\`);
    } catch (err) {
      await interaction.reply(\`Could not ban: \${err.message}\`);
    }
  },
};

// + index.js with the Client, a command loader and the interactionCreate event
// + deploy-commands.js to register the slash command through the REST API
// + a second handler if you also want !ban`;

export const banAoiJs = `// index.js (aoi.js v6, simplified — check the aoi.js docs for details)
const { AoiClient } = require("aoi.js");

const client = new AoiClient({
  token: process.env.TOKEN,
  prefix: "!",
  intents: ["MessageContent", "Guilds", "GuildMessages"],
  events: ["onMessage"],
});

client.command({
  name: "ban",
  code: \`
  $ban[$guildID;$findMember[$message[1];false];0;$messageSlice[1]]
  $username[$findMember[$message[1];false]] was banned. Reason: $messageSlice[1]
  $onlyIf[$findMember[$message[1];false]!=;Could not find that member]
  \`,
});

// slash commands are created and handled separately`;

export const componentsSample = `command("rules") {
  reply container(
    text("## Welcome, $user.name"),
    section("Read the rules before you continue.", thumbnail: user.avatar),
    separator(),
    row(
      button("accept", "I agree", style: "success"),
      button(label: "Website", url: "https://example.com")
    ),
    color: "blurple"
  )
}

button("accept") {
  addRole user "Member"
  reply "Welcome aboard!" ephemeral: true
}`;

export const brokenSample = `command("ping") {
  rpely "Pong!"
}`;

export const errorOutput = `erro[E0201]: A função \`rpely\` não existe
 --> commands/ping.dcs:2:3
  |
2 |   rpely "Pong!"
  |   ^^^^^ função desconhecida
  |
  = ajuda: você quis dizer \`reply\`?
  = correção sugerida: trocar \`rpely\` por \`reply\``;

export const quickstart = `npx diracord new my-bot
cd my-bot
cp .env.example .env   # paste your bot token here
npx diracord run`;
