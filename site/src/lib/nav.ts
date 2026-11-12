/** Pages of the site's two documentation areas (safe to import in the browser). */

export interface PageMeta {
  slug: string;
  title: string;
  description: string;
}

export interface NavGroup {
  title: string;
  pages: PageMeta[];
}

export type SectionId = "docs" | "components";

export interface Section {
  id: SectionId;
  /** Label of the tab in the header. */
  label: string;
  groups: NavGroup[];
}

const docs: NavGroup[] = [
  {
    title: "Começar",
    pages: [
      { slug: "getting-started", title: "Primeiros passos", description: "Instale, crie um bot e coloque online." },
      { slug: "why", title: "Por que Diracord", description: "As diferenças para aoi.js e discord.js, sem enrolação." },
      { slug: "tips", title: "Comandos prontos", description: "Os comandos que todo bot tem, com o resultado no Discord." },
      { slug: "mistakes", title: "Bugs e erros comuns", description: "O que costuma dar errado e como evitar." },
    ],
  },
  {
    title: "Linguagem",
    pages: [
      { slug: "syntax", title: "Sintaxe", description: "Comandos, eventos, variáveis, if, for, chain e mais." },
      { slug: "functions", title: "Funções próprias (fn)", description: "Reaproveite código com fn." },
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

const components: NavGroup[] = [
  {
    title: "Começar",
    pages: [
      { slug: "overview", title: "Visão geral", description: "Como montar mensagens com peças, e onde cada uma pode ficar." },
      { slug: "embed", title: "Embed", description: "O cartão clássico: título, descrição, campos e cor." },
    ],
  },
  {
    title: "Estrutura",
    pages: [
      { slug: "container", title: "container", description: "Caixa com cor de destaque que agrupa outras peças." },
      { slug: "section", title: "section", description: "Texto com uma imagem pequena ou um botão ao lado." },
      { slug: "row", title: "row", description: "Uma linha de botões, ou um menu." },
      { slug: "separator", title: "separator", description: "Espaço ou linha entre peças." },
    ],
  },
  {
    title: "Conteúdo",
    pages: [
      { slug: "text", title: "text", description: "Texto com markdown." },
      { slug: "thumbnail", title: "thumbnail", description: "Imagem pequena ao lado de uma section." },
      { slug: "gallery", title: "gallery", description: "De 1 a 10 imagens grandes." },
      { slug: "file", title: "file", description: "Mostra um arquivo anexado." },
    ],
  },
  {
    title: "Interação",
    pages: [
      { slug: "button", title: "button", description: "Botão que roda código, ou que abre um link." },
      { slug: "menu", title: "menu", description: "Menu de opções para escolher." },
    ],
  },
];

export const sections: Record<SectionId, Section> = {
  docs: { id: "docs", label: "Docs", groups: docs },
  components: { id: "components", label: "Componentes", groups: components },
};

export function pagesOf(section: SectionId): PageMeta[] {
  return sections[section].groups.flatMap((g) => g.pages);
}

export function findPage(section: SectionId, slug: string): PageMeta | undefined {
  return pagesOf(section).find((p) => p.slug === slug);
}

/** Previous and next page inside a section. */
export function neighbours(section: SectionId, slug: string): { prev?: PageMeta; next?: PageMeta } {
  const all = pagesOf(section);
  const i = all.findIndex((p) => p.slug === slug);
  return { prev: all[i - 1], next: all[i + 1] };
}
