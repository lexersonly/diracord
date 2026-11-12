"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/motion/tabs";
import { Code } from "@/components/site/code";
import { SectionTitle } from "@/components/site/section";
import { banAoiJs, banDiracord, banDiscordJs } from "./samples";

const lines = (s: string) => s.split("\n").filter((l) => l.trim() && !l.trim().startsWith("//")).length;

const NOTES = {
  diracord: [
    "Um arquivo. Funciona como !ban e /ban.",
    "`user: member` já converte menção ou id em membro.",
    "Se o Discord recusar, o catch recebe a mensagem.",
  ],
  djs: [
    "Completo e poderoso, mas exige JavaScript, async/await e builders.",
    "Slash precisa de registro à parte; prefixo é outro handler.",
    "Você monta o Client, o carregador de comandos e os eventos.",
  ],
  aoi: [
    "Tudo vira texto dentro de uma string do JavaScript.",
    "Funções $ aninhadas: $findMember dentro de $ban dentro de $username.",
    "Condições com $onlyIf; slash é configurado separado.",
  ],
};

export function Compare() {
  return (
    <section id="compare" className="mx-auto max-w-6xl px-4 py-24 sm:px-6">
      <SectionTitle
        eyebrow="Comparação"
        title={
          <>
            O mesmo comando,
            <br />
            <span className="text-chrome-dim">três jeitos de escrever</span>
          </>
        }
        sub="Um /ban com motivo opcional e tratamento de erro. Contamos só as linhas com código."
      />
      <Tabs defaultValue="diracord" variant="pill" className="mt-10">
        <TabsList>
          <TabsTrigger value="diracord">Diracord · {lines(banDiracord)} linhas</TabsTrigger>
          <TabsTrigger value="djs">discord.js · {lines(banDiscordJs)}+ linhas</TabsTrigger>
          <TabsTrigger value="aoi">aoi.js · {lines(banAoiJs)}+ linhas</TabsTrigger>
        </TabsList>
        {(
          [
            ["diracord", banDiracord, "dcs", "commands/ban.dcs", NOTES.diracord],
            ["djs", banDiscordJs, "javascript", "commands/ban.js", NOTES.djs],
            ["aoi", banAoiJs, "javascript", "index.js", NOTES.aoi],
          ] as const
        ).map(([value, code, lang, file, notes]) => (
          <TabsContent key={value} value={value} className="mt-6">
            <div className="grid gap-6 md:grid-cols-[1.4fr_0.6fr]">
              <Code code={code} lang={lang} filename={file} lineNumbers />
              <ul className="space-y-3 text-sm text-muted-foreground">
                {notes.map((n) => (
                  <li key={n} className="metal rounded-2xl p-4 leading-relaxed">
                    {n.split("`").map((part, i) =>
                      i % 2 ? (
                        <code key={i} className="rounded bg-white/5 px-1 font-mono text-foreground">
                          {part}
                        </code>
                      ) : (
                        part
                      ),
                    )}
                  </li>
                ))}
              </ul>
            </div>
          </TabsContent>
        ))}
      </Tabs>
    </section>
  );
}
