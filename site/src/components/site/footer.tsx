import Link from "next/link";
import { site } from "@/lib/site";
import { Logo } from "./logo";

const COLUMNS = [
  {
    title: "Aprender",
    links: [
      { href: "/docs/getting-started", label: "Começar" },
      { href: "/docs/syntax", label: "Sintaxe" },
      { href: "/docs/components-v2", label: "Components v2" },
      { href: "/docs/tips", label: "Dicas e comandos básicos" },
    ],
  },
  {
    title: "Vindo de",
    links: [
      { href: "/docs/from-aoijs", label: "aoi.js" },
      { href: "/docs/from-discordjs", label: "discord.js" },
      { href: "/docs/why", label: "Por que Diracord" },
    ],
  },
  {
    title: "Referência",
    links: [
      { href: "/docs/reference-functions", label: "Funções" },
      { href: "/docs/reference-errors", label: "Códigos de erro" },
      { href: "/docs/mistakes", label: "Erros comuns" },
      { href: "/llm.txt", label: "llm.txt (para IAs)" },
    ],
  },
];

export function Footer() {
  return (
    <footer className="relative mt-24 overflow-hidden border-t border-white/[0.06]">
      <div className="mx-auto grid max-w-6xl gap-10 px-4 py-14 sm:px-6 md:grid-cols-[1.4fr_1fr_1fr_1fr]">
        <div className="space-y-3">
          <div className="flex items-center gap-2 font-semibold">
            <Logo className="size-6" />
            {site.name}
          </div>
          <p className="max-w-xs text-sm text-muted-foreground">{site.tagline} Código aberto, licença MIT.</p>
          <a href={site.repo} className="text-sm text-muted-foreground underline-offset-4 hover:text-foreground hover:underline">
            github.com/lexersonly/diracord
          </a>
        </div>
        {COLUMNS.map((c) => (
          <div key={c.title} className="space-y-3 text-sm">
            <p className="font-medium">{c.title}</p>
            <ul className="space-y-2 text-muted-foreground">
              {c.links.map((l) => (
                <li key={l.href}>
                  <Link href={l.href} className="transition-colors hover:text-foreground">
                    {l.label}
                  </Link>
                </li>
              ))}
            </ul>
          </div>
        ))}
      </div>
      <div
        aria-hidden
        className="text-chrome-dim pointer-events-none -mb-[0.22em] select-none text-center text-[clamp(4rem,19vw,16rem)] font-semibold leading-none tracking-[-0.06em]"
      >
        DIRACORD
      </div>
    </footer>
  );
}
