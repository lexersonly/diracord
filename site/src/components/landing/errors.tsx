import { Code } from "@/components/site/code";
import { SectionTitle } from "@/components/site/section";
import { brokenSample, errorOutput } from "./samples";

export function ErrorsShowcase() {
  return (
    <section className="mx-auto max-w-6xl px-4 py-24 sm:px-6">
      <div className="grid items-center gap-12 md:grid-cols-2">
        <SectionTitle
          eyebrow="Erros"
          title={
            <>
              O bot não cai.
              <br />
              <span className="text-chrome-dim">O erro aparece antes.</span>
            </>
          }
          sub="diracord check lê o projeto inteiro e mostra todos os problemas de uma vez, com código estável (E0201) para você pesquisar. Nada de descobrir o erro só quando alguém usa o comando."
        />
        <div className="space-y-4">
          <Code code={brokenSample} filename="commands/ping.dcs" lineNumbers />
          <Code code={`$ diracord check\n\n${errorOutput}`} lang="text" filename="terminal" />
        </div>
      </div>
    </section>
  );
}
