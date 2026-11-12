import Link from "next/link";
import { ButtonLink } from "@/components/motion/button/base";
import { Code } from "@/components/site/code";
import { DiscordPreview } from "@/components/site/discord-preview";
import { SectionTitle } from "@/components/site/section";

const banCode = `command("ban", user: member, reason: text = "no reason") {
  testing {
    ban user reason
    reply "$user.name was banned. Reason: $reason"
  } catch err {
    reply "Could not ban: $err"
  }
}`;

const rulesCode = `command("rules") {
  reply container(
    text("## Welcome, $user.name"),
    text("Be nice and have fun."),
    row(button("accept", "I agree", style: "success")),
    color: "blurple"
  )
}`;

const errorOutput = `erro[E0201]: A função \`rpely\` não existe
 --> commands/ping.dcs:2:3
  |
2 |   rpely "Pong!"
  |   ^^^^^ função desconhecida
  |
  = ajuda: você quis dizer \`reply\`?`;

const quickstart = `npx diracord new my-bot
cd my-bot
cp .env.example .env   # cole o token do bot
npx diracord run`;

function Step({ n, title, text, children }: { n: string; title: string; text: React.ReactNode; children: React.ReactNode }) {
  return (
    <div className="grid items-center gap-8 md:grid-cols-[0.8fr_1.2fr]">
      <div className="space-y-3">
        <p className="font-mono text-sm text-violet">{n}</p>
        <h3 className="text-2xl font-medium tracking-tight">{title}</h3>
        <p className="text-muted-foreground">{text}</p>
      </div>
      <div className="min-w-0 space-y-3">{children}</div>
    </div>
  );
}

export function Showcase() {
  return (
    <section className="mx-auto max-w-6xl space-y-24 px-4 py-24 sm:px-6">
      <SectionTitle
        eyebrow="Como funciona"
        title="Você escreve isto. O Discord mostra aquilo."
        sub="Sem JavaScript, sem montar cliente, sem registrar slash command na mão."
      />

      <Step
        n="01"
        title="Um comando, dois jeitos de usar"
        text={
          <>
            O mesmo arquivo atende <code className="text-foreground">!ban</code> e <code className="text-foreground">/ban</code>. O tipo{" "}
            <code className="text-foreground">member</code> já transforma a menção no membro.
          </>
        }
      >
        <Code code={banCode} filename="commands/ban.dcs" />
        <DiscordPreview
          data={{
            messages: [
              { from: "user", text: "!ban @Joe link spam" },
              { from: "bot", text: "Joe was banned. Reason: link spam" },
            ],
          }}
        />
      </Step>

      <Step
        n="02"
        title="Mensagens bonitas com peças"
        text={
          <>
            Caixas, botões e menus do Components v2 se encaixam como peças.{" "}
            <Link href="/components/overview" className="text-foreground underline decoration-white/30 underline-offset-4">
              Veja todas, com exemplos
            </Link>
            .
          </>
        }
      >
        <Code code={rulesCode} filename="commands/rules.dcs" />
        <DiscordPreview
          data={{
            messages: [
              { from: "user", text: "!rules" },
              {
                from: "bot",
                layout: [
                  {
                    container: [
                      { text: "## Welcome, Ana" },
                      { text: "Be nice and have fun." },
                      { row: [{ button: "accept", label: "I agree", style: "success" }] },
                    ],
                    color: "blurple",
                  },
                ],
              },
            ],
          }}
        />
      </Step>

      <Step
        n="03"
        title="O erro aparece antes de o bot ligar"
        text={
          <>
            <code className="text-foreground">diracord check</code> lê o projeto inteiro, aponta a linha e sugere a correção. Os erros seguros
            ele corrige sozinho com <code className="text-foreground">--fix</code>.
          </>
        }
      >
        <Code code={errorOutput} lang="text" filename="npx diracord check" />
      </Step>

      <div className="metal grid items-center gap-8 rounded-3xl p-8 md:grid-cols-2 md:p-12">
        <div className="space-y-5">
          <h2 className="text-3xl font-medium tracking-tight sm:text-4xl">Do zero ao bot em quatro comandos.</h2>
          <p className="text-muted-foreground">
            O projeto novo já vem com ping, moedas salvas, uma mensagem com botão e boas-vindas. Precisa só do Node.js e do token do bot.
          </p>
          <div className="flex flex-wrap gap-3">
            <ButtonLink href="/docs/getting-started">Guia passo a passo</ButtonLink>
            <ButtonLink href="/docs/tips" variant="secondary" className="glass-thin">
              Comandos prontos
            </ButtonLink>
          </div>
        </div>
        <Code code={quickstart} lang="bash" filename="terminal" />
      </div>
    </section>
  );
}
