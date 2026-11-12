import Link from "next/link";
import { DiscordPreview, type PreviewData } from "@/components/site/discord-preview";
import { Eyebrow } from "@/components/site/section";
import { sections } from "@/lib/nav";

export const metadata = { title: "Componentes" };

/** A tiny live preview for each piece, shown on its card. */
const MINI: Record<string, PreviewData["messages"][number]> = {
  overview: {
    from: "bot",
    layout: [{ container: [{ text: "## Welcome" }, { row: [{ button: "a", label: "I agree", style: "success" }] }], color: "purple" }],
  },
  embed: { from: "bot", embed: { title: "Profile", description: "Level 7", color: "blurple" } },
  container: { from: "bot", layout: [{ container: [{ text: "## News" }, { text: "Event tomorrow." }], color: "blurple" }] },
  section: { from: "bot", layout: [{ section: ["**Ana**", "Level 7"], thumbnail: "x" }] },
  row: { from: "bot", layout: [{ row: [{ button: "a", label: "Yes", style: "success" }, { button: "b", label: "No", style: "danger" }] }] },
  separator: { from: "bot", layout: [{ text: "Above" }, { separator: true }, { text: "Below" }] },
  text: { from: "bot", layout: [{ text: "## Title" }, { text: "Some **bold** text." }] },
  thumbnail: { from: "bot", layout: [{ section: ["Avatar"], thumbnail: "x" }] },
  gallery: { from: "bot", layout: [{ gallery: [1, 2] }] },
  file: { from: "bot", layout: [{ file: "attachment://report.pdf" }] },
  button: { from: "bot", layout: [{ row: [{ button: "a", label: "Click me" }, { button: null, label: "Link", url: "x" }] }] },
  menu: { from: "bot", layout: [{ row: [{ menu: "m", options: ["Red"], placeholder: "Choose a color" }] }] },
};

export default function ComponentsIndex() {
  return (
    <div className="space-y-12">
      <div className="space-y-4">
        <Eyebrow>Componentes</Eyebrow>
        <h1 className="text-chrome pb-1 text-5xl font-semibold tracking-[-0.04em]">Peças de mensagem</h1>
        <p className="max-w-xl text-muted-foreground">
          Tudo que dá para colocar numa mensagem do Discord. Cada página mostra a peça funcionando, o código, os argumentos e os erros
          comuns.
        </p>
      </div>
      {sections.components.groups.map((g) => (
        <div key={g.title} className="space-y-4">
          <h2 className="text-sm font-medium uppercase tracking-wider text-white/40">{g.title}</h2>
          <div className="grid gap-4 sm:grid-cols-2">
            {g.pages.map((p) => (
              <Link
                key={p.slug}
                href={`/components/${p.slug}`}
                className="metal group flex flex-col gap-4 rounded-2xl p-4 transition-colors hover:border-white/20"
              >
                <div className="pointer-events-none max-h-44 overflow-hidden rounded-xl">
                  {MINI[p.slug] && <DiscordPreview data={{ messages: [MINI[p.slug]] }} className="rounded-xl shadow-none" />}
                </div>
                <div>
                  <p className={g.title === "Começar" ? "font-medium" : "font-mono font-medium"}>{p.title}</p>
                  <p className="mt-1 text-sm text-muted-foreground">{p.description}</p>
                </div>
              </Link>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
