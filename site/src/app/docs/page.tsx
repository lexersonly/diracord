import Link from "next/link";
import { TiltCard } from "@/components/motion/tilt-card";
import { Eyebrow } from "@/components/site/section";
import { sections } from "@/lib/nav";

const groups = sections.docs.groups;

export const metadata = { title: "Documentação" };

export default function DocsIndex() {
  return (
    <div className="space-y-12">
      <div className="space-y-4">
        <Eyebrow>Documentação</Eyebrow>
        <h1 className="text-chrome text-5xl font-semibold tracking-[-0.04em]">Aprenda Diracord</h1>
        <p className="max-w-xl text-muted-foreground">
          Comece pelos primeiros passos. Se você vem do aoi.js ou do discord.js, tem um guia só para você.
        </p>
      </div>
      {groups.map((g) => (
        <div key={g.title} className="space-y-4">
          <h2 className="text-sm font-medium uppercase tracking-wider text-white/40">{g.title}</h2>
          <div className="grid gap-4 sm:grid-cols-2">
            {g.pages.map((p) => (
              <TiltCard key={p.slug} max={5} glare>
                <Link href={`/docs/${p.slug}`} className="metal block h-full rounded-2xl p-5 transition-colors hover:border-white/20">
                  <p className="font-medium">{p.title}</p>
                  <p className="mt-1 text-sm text-muted-foreground">{p.description}</p>
                </Link>
              </TiltCard>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
