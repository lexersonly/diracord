import { TiltCard } from "@/components/motion/tilt-card";
import { SectionTitle } from "@/components/site/section";
import { site } from "@/lib/site";

function Card({ title, text, children, wide }: { title: string; text: string; children: React.ReactNode; wide?: boolean }) {
  return (
    <TiltCard max={6} className={wide ? "md:col-span-2" : undefined}>
      <div className="metal flex h-full flex-col gap-5 rounded-3xl p-6">
        <div className="flex min-h-[132px] items-center justify-center rounded-2xl border border-white/[0.06] bg-black/40 p-4">
          {children}
        </div>
        <div className="space-y-1.5">
          <h3 className="font-medium">{title}</h3>
          <p className="text-sm leading-relaxed text-muted-foreground">{text}</p>
        </div>
      </div>
    </TiltCard>
  );
}

const pill = "rounded-full border border-white/10 bg-white/[0.04] px-3 py-1 font-mono text-xs text-foreground/90";

export function Features() {
  return (
    <section id="features" className="relative">
      <div aria-hidden className="bg-grid mask-b-fade absolute inset-0 opacity-50" />
      <div className="relative mx-auto max-w-6xl px-4 py-24 sm:px-6">
        <SectionTitle
          eyebrow="Recursos"
          title={
            <>
              Feito para quem está
              <br />
              <span className="text-chrome-dim">começando, sem limitar quem já sabe</span>
            </>
          }
          sub="Tudo que um bot precisa vem pronto. O que você escreve é o que o bot faz."
        />
        <div className="mt-12 grid gap-5 md:grid-cols-3">
          <Card wide title="Erros que ensinam" text="Cada erro mostra o arquivo, a linha, uma seta no lugar exato e a correção. Os seguros o diracord check --fix corrige sozinho.">
            <pre className="w-full overflow-hidden font-mono text-[11px] leading-5 text-muted-foreground">
              <span className="text-danger">erro[E0201]</span>
              <span className="text-foreground">: A função `rpely` não existe</span>
              {"\n  |\n2 |   rpely \"Pong!\"\n  |   "}
              <span className="text-danger">^^^^^ função desconhecida</span>
              {"\n  = ajuda: você quis dizer `reply`?"}
            </pre>
          </Card>
          <Card title="Um comando, dois jeitos" text="O mesmo command atende prefixo e slash. Os modos ligam e desligam no diracord.toml.">
            <div className="flex flex-col items-center gap-2">
              <span className={pill}>!ban @Joe spam</span>
              <span className={pill}>/ban user: @Joe reason: spam</span>
            </div>
          </Card>
          <Card title="Components v2 em peças" text="container, text, row, button: peças que entram umas nas outras, como no discord.js, sem new nem .add.">
            <div className="w-full space-y-2 rounded-xl border-l-2 border-[#5865f2] bg-white/[0.03] p-3 text-left text-xs">
              <p className="font-semibold text-foreground">Welcome, Joe</p>
              <p className="text-muted-foreground">Read the rules before you continue.</p>
              <div className="flex gap-2 pt-1">
                <span className="rounded-md bg-emerald-600/80 px-2 py-1 text-[11px] text-white">I agree</span>
                <span className="rounded-md bg-white/10 px-2 py-1 text-[11px] text-white">Website ↗</span>
              </div>
            </div>
          </Card>
          <Card title="Banco de dados embutido" text="get e set salvam em um arquivo só (Dirabank), por usuário, servidor, membro ou global.">
            <div className="flex flex-wrap justify-center gap-2">
              {["user", "server", "member", "global"].map((s) => (
                <span key={s} className={pill}>
                  scope: {s}
                </span>
              ))}
            </div>
          </Card>
          <Card title="Leve de verdade" text={`O motor é Rust: ${site.engineRamMb} MB de RAM rodando milhares de comandos. Feito para caber num host de 100 MB.`}>
            <p className="text-chrome text-5xl font-semibold tracking-tight">
              {site.engineRamMb}
              <span className="text-2xl"> MB</span>
            </p>
          </Card>
          <Card wide title="Seguro por padrão" text="Todo comando tem limite de passos, tempo, memória e ações. Um while infinito é parado com um erro claro, não derruba o bot.">
            <div className="grid w-full grid-cols-2 gap-2 font-mono text-[11px] sm:grid-cols-4">
              {["100k passos", "3 s de CPU", "4 MB", "50 ações"].map((s) => (
                <span key={s} className="rounded-lg border border-white/10 bg-white/[0.03] px-2 py-2 text-center text-muted-foreground">
                  {s}
                </span>
              ))}
            </div>
          </Card>
          <Card title="Funções próprias" text="Repetiu código? Crie uma fn, com tipos opcionais iguais aos dos comandos, e chame como qualquer função.">
            <pre className="font-mono text-[11px] leading-5 text-muted-foreground">
              <span className="text-foreground">fn</span> giveCoins(who, amount = 1) {"{"}
              {"\n  "}
              <span className="text-foreground">return</span> amount
              {"\n}"}
            </pre>
          </Card>
        </div>
      </div>
    </section>
  );
}
