"use client";

import { BouncyAccordion } from "@/components/motion/bouncy-accordion";
import { SectionTitle } from "@/components/site/section";

const ITEMS = [
  {
    id: "free",
    title: "É de graça?",
    description: "Sim. O Diracord é código aberto (licença MIT). Você só precisa de um lugar para rodar o bot.",
  },
  {
    id: "learn",
    title: "Preciso saber programar?",
    description:
      "Não. Se você sabe escrever reply \"Pong!\", já fez um comando. O resto (variáveis, if, for, funções) aparece aos poucos no guia, e os erros explicam o que corrigir.",
  },
  {
    id: "aoi",
    title: "Venho do aoi.js. Vou sofrer?",
    description:
      "Não. Ligue [syntax] dollar = true e o atalho $ban[user;reason] funciona. Quando quiser, o diracord check --fix converte para a forma normal. Tem um guia só para quem vem do aoi.js.",
  },
  {
    id: "slash",
    title: "Funciona com slash commands?",
    description:
      "Sim. O mesmo command(\"ban\", user: member) vira /ban com as opções certas e também atende !ban. Você escolhe os modos no diracord.toml.",
  },
  {
    id: "data",
    title: "Onde ficam os dados salvos?",
    description:
      "No Dirabank, um banco embutido em um arquivo só (data/bot.dirabank). Sem servidor de banco, sem configuração. Escopos: user, server, member e global.",
  },
  {
    id: "ai",
    title: "Uma IA consegue escrever Diracord?",
    description:
      "Sim. O site publica o /llm.txt com a gramática completa, todas as funções e os erros comuns, feito para IAs lerem antes de escrever código.",
  },
  {
    id: "ready",
    title: "Já dá para usar em produção?",
    description:
      "O Diracord está na versão 0.1: a linguagem, o motor e a conexão com o Discord funcionam e são testados, mas ainda pode mudar. Para bots pessoais e de comunidade, dá para começar hoje.",
  },
];

export function Faq() {
  return (
    <section className="mx-auto grid max-w-6xl gap-12 px-4 py-24 sm:px-6 md:grid-cols-[0.8fr_1.2fr]">
      <SectionTitle
        eyebrow="Perguntas"
        title={
          <>
            Perguntas
            <br />
            <span className="text-chrome-dim">frequentes</span>
          </>
        }
        sub="Não achou a sua? Abra uma issue no GitHub."
      />
      <BouncyAccordion items={ITEMS} defaultValue="free" />
    </section>
  );
}
