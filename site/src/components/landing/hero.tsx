import { AnimatedBadge } from "@/components/motion/animated-badge";
import { ButtonLink } from "@/components/motion/button/base";
import { ScrollReveal } from "@/components/motion/scroll-reveal";
import { Code } from "@/components/site/code";
import { Sparkles } from "@/components/site/sparkles";
import { pingSample } from "./samples";

export function Hero() {
  return (
    <section className="relative overflow-hidden">
      <div aria-hidden className="bg-grid mask-radial-fade absolute inset-0 opacity-70" />
      <div
        aria-hidden
        className="absolute -top-40 left-1/2 h-[520px] w-[900px] -translate-x-1/2 rounded-full bg-[radial-gradient(closest-side,rgba(255,255,255,0.12),transparent)]"
      />
      <div className="relative mx-auto grid max-w-6xl items-center gap-12 px-4 pb-20 pt-20 sm:px-6 md:grid-cols-[1.15fr_0.85fr] md:pt-28">
        <div className="space-y-7">
          <AnimatedBadge status="neutral" size="md" showIcon={false} className="glass-thin gap-2.5">
            <span className="rounded-full bg-white px-1.5 py-px text-[10px] font-semibold text-black">Novo</span>
            Components v2 em peças, estilo discord.js
          </AnimatedBadge>
          <ScrollReveal y={18} blur={8}>
            <h1 className="text-chrome pb-2 text-balance text-5xl font-semibold leading-[0.98] tracking-[-0.045em] sm:text-7xl">
              Bots de Discord em texto simples.
            </h1>
          </ScrollReveal>
          <p className="max-w-lg text-pretty text-lg text-muted-foreground">
            Escreva <code className="rounded bg-white/5 px-1.5 py-0.5 font-mono text-[0.9em] text-foreground">reply &quot;Pong!&quot;</code>{" "}
            e pronto. Um comando atende <b className="font-medium text-foreground">!prefixo</b> e{" "}
            <b className="font-medium text-foreground">/slash</b>, e quando algo está errado o Diracord aponta a linha, explica e
            sugere a correção.
          </p>
          <div className="flex flex-wrap items-center gap-3">
            <ButtonLink href="/docs/getting-started" size="lg">
              Criar meu primeiro bot
            </ButtonLink>
            <ButtonLink href="/docs/why" size="lg" variant="secondary" className="glass-thin">
              Ver as diferenças
            </ButtonLink>
          </div>
        </div>
        <div className="relative flex flex-col items-center gap-6">
          <Sparkles className="hidden md:block" />
          <div className="w-full max-w-sm">
            <Code code={pingSample} filename="commands/ping.dcs" />
          </div>
        </div>
      </div>
    </section>
  );
}
