/** Documentation pages and their order (safe to use in the browser). */

/** One documentation page. The text lives in `content/docs/<slug>.md`. */
export interface DocMeta {
  slug: string;
  title: string;
  description: string;
}

export interface DocGroup {
  title: string;
  pages: DocMeta[];
}

/** Sidebar order. Every slug must have a markdown file. */
export const groups: DocGroup[] = [
  {
    title: "Começar",
    pages: [
      { slug: "getting-started", title: "Primeiros passos", description: "Instale, crie um bot e coloque online." },
      { slug: "why", title: "Por que Diracord", description: "As diferenças para aoi.js e discord.js, sem enrolação." },
      { slug: "tips", title: "Dicas e comandos básicos", description: "Os comandos que todo bot tem, prontos para copiar." },
      { slug: "mistakes", title: "Bugs e erros comuns", description: "O que costuma dar errado e como evitar." },
    ],
  },
  {
    title: "Linguagem",
    pages: [
      { slug: "syntax", title: "Sintaxe", description: "Comandos, eventos, variáveis, if, for, chain e mais." },
      { slug: "functions", title: "Funções próprias (fn)", description: "Reaproveite código com fn." },
      { slug: "components-v2", title: "Components v2", description: "Caixas, botões e menus com peças." },
      { slug: "data", title: "Dados salvos", description: "get, set e escopos no Dirabank." },
      { slug: "config", title: "diracord.toml e CLI", description: "Configuração, .env e os comandos do terminal." },
    ],
  },
  {
    title: "Vindo de",
    pages: [
      { slug: "from-aoijs", title: "aoi.js", description: "Equivalências e o atalho $ban[...]." },
      { slug: "from-discordjs", title: "discord.js", description: "Como as ideias do discord.js viram Diracord." },
    ],
  },
  {
    title: "Referência",
    pages: [
      { slug: "reference-functions", title: "Todas as funções", description: "Lista gerada a partir do código." },
      { slug: "reference-errors", title: "Códigos de erro", description: "Todos os erros, de E0001 a E0504." },
    ],
  },
];

export const allDocs: DocMeta[] = groups.flatMap((g) => g.pages);

export function findDoc(slug: string): DocMeta | undefined {
  return allDocs.find((d) => d.slug === slug);
}

/** Previous and next page, for the links at the bottom. */
export function neighbours(slug: string): { prev?: DocMeta; next?: DocMeta } {
  const i = allDocs.findIndex((d) => d.slug === slug);
  return { prev: allDocs[i - 1], next: allDocs[i + 1] };
}
