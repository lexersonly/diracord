import { ButtonLink } from "@/components/motion/button/base";
import { HeroScene } from "./hero-scene";

const POINTS = ["!prefixo e /slash juntos", "Erros com sugestão", "Components v2 em peças", "Banco embutido", "Motor em Rust"];

export function Hero() {
  return (
    <section className="relative isolate overflow-hidden">
      {/* Fallback glow, also the mood behind the 3D scene. */}
      <div
        aria-hidden
        className="absolute inset-0 -z-20 bg-[radial-gradient(60%_70%_at_75%_40%,rgba(124,58,237,0.35),transparent_70%),radial-gradient(40%_50%_at_60%_80%,rgba(37,99,235,0.25),transparent_70%)]"
      />
      <HeroScene className="absolute inset-0 -z-10" />
      <div aria-hidden className="absolute inset-0 -z-10 bg-gradient-to-r from-background via-background/70 to-transparent md:via-background/40" />
      <div aria-hidden className="absolute inset-x-0 bottom-0 -z-10 h-40 bg-gradient-to-t from-background to-transparent" />

      <div className="mx-auto flex min-h-[calc(100svh-3.5rem)] max-w-6xl flex-col justify-center px-4 pb-32 pt-16 sm:px-6">
        <div className="max-w-2xl space-y-7">
          <span className="glass-thin inline-flex items-center gap-2 rounded-full px-3 py-1 text-xs text-white/80">
            <span className="size-1.5 rounded-full bg-violet shadow-[0_0_10px_var(--violet)]" />
            v0.1 no npm · npx diracord new my-bot
          </span>
          <h1 className="text-balance text-5xl font-medium leading-[1.02] tracking-[-0.04em] text-white sm:text-7xl">
            Seu bot de Discord, em texto simples.
          </h1>
          <p className="max-w-md text-lg text-white/70">
            Uma linguagem feita para criar bots. Escreva o comando, e o Diracord aponta o erro antes de o bot ligar.
          </p>
          <div className="flex flex-wrap gap-3">
            <ButtonLink href="/docs/getting-started" size="lg">
              Começar
            </ButtonLink>
            <ButtonLink href="/components/overview" size="lg" variant="secondary" className="glass-thin">
              Ver componentes
            </ButtonLink>
          </div>
        </div>
      </div>

      <div className="absolute inset-x-0 bottom-8 px-4 sm:px-6">
        <div className="glass mx-auto flex max-w-4xl flex-wrap items-center justify-center gap-x-8 gap-y-2 rounded-2xl px-6 py-4 text-sm text-white/75">
          {POINTS.map((p) => (
            <span key={p} className="whitespace-nowrap">
              {p}
            </span>
          ))}
        </div>
      </div>
    </section>
  );
}
