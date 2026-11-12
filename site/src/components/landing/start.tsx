import { ButtonLink } from "@/components/motion/button/base";
import { Code } from "@/components/site/code";
import { SectionTitle } from "@/components/site/section";
import { componentsSample, quickstart } from "./samples";

export function Start() {
  return (
    <section className="mx-auto max-w-6xl px-4 py-24 sm:px-6">
      <div className="grid gap-12 md:grid-cols-2">
        <div className="space-y-8">
          <SectionTitle
            eyebrow="Começar"
            title={
              <>
                Do zero ao bot
                <br />
                <span className="text-chrome-dim">em quatro comandos</span>
              </>
            }
            sub="O projeto novo já vem com ping, moedas salvas, regras com botão e boas-vindas."
          />
          <Code code={quickstart} lang="bash" filename="terminal" />
          <ButtonLink href="/docs/getting-started" variant="secondary" className="glass-thin">
            Guia completo →
          </ButtonLink>
        </div>
        <div className="space-y-4">
          <p className="text-sm text-muted-foreground">commands/rules.dcs: Components v2 com botão</p>
          <Code code={componentsSample} filename="commands/rules.dcs" lineNumbers />
        </div>
      </div>
    </section>
  );
}
