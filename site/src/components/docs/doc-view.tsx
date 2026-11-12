import Link from "next/link";
import { notFound } from "next/navigation";
import { Suspense } from "react";
import { headings, Markdown } from "@/components/docs/markdown";
import { Eyebrow } from "@/components/site/section";
import { readContent } from "@/lib/content";
import { findPage, neighbours, sections, type SectionId } from "@/lib/nav";

/** A documentation page: header, markdown, "nesta página" and prev/next links. */
export function DocView({ section, params }: { section: SectionId; params: Promise<{ slug: string }> }) {
  return (
    <Suspense fallback={<Skeleton />}>
      <Content section={section} params={params} />
    </Suspense>
  );
}

function Skeleton() {
  return (
    <div className="animate-pulse space-y-6">
      <div className="h-6 w-28 rounded-full bg-white/5" />
      <div className="h-12 w-2/3 rounded-xl bg-white/5" />
      <div className="h-5 w-1/2 rounded bg-white/5" />
      <div className="h-64 rounded-2xl bg-white/[0.03]" />
    </div>
  );
}

async function Content({ section, params }: { section: SectionId; params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  const page = findPage(section, slug);
  if (!page) notFound();
  const source = await readContent(section, slug);
  const toc = headings(source);
  const group = sections[section].groups.find((g) => g.pages.some((p) => p.slug === slug));
  const { prev, next } = neighbours(section, slug);
  const isPiece = section === "components" && group?.title !== "Começar";
  return (
    <div className="grid gap-10 xl:grid-cols-[1fr_200px]">
      <article className="min-w-0">
        <header className="mb-10 space-y-4 border-b border-white/[0.06] pb-8">
          {group && <Eyebrow>{group.title}</Eyebrow>}
          <h1 className={`text-chrome pb-1 text-4xl font-semibold tracking-[-0.035em] sm:text-5xl ${isPiece ? "font-mono" : ""}`}>
            {page.title}
          </h1>
          <p className="text-lg text-muted-foreground">{page.description}</p>
        </header>
        <Markdown source={source} />
        <nav className="mt-16 grid gap-4 border-t border-white/[0.06] pt-8 sm:grid-cols-2">
          {prev ? (
            <Link href={`/${section}/${prev.slug}`} className="metal rounded-2xl p-4 text-sm">
              <span className="text-muted-foreground">← Anterior</span>
              <p className="mt-1 font-medium">{prev.title}</p>
            </Link>
          ) : (
            <span />
          )}
          {next && (
            <Link href={`/${section}/${next.slug}`} className="metal rounded-2xl p-4 text-right text-sm">
              <span className="text-muted-foreground">Próxima →</span>
              <p className="mt-1 font-medium">{next.title}</p>
            </Link>
          )}
        </nav>
      </article>
      {toc.length > 1 && (
        <aside className="hidden xl:block">
          <div className="sticky top-24 space-y-3 text-sm">
            <p className="text-xs font-medium uppercase tracking-wider text-white/40">Nesta página</p>
            <ul className="space-y-2">
              {toc.map((h) => (
                <li key={h.id}>
                  <a href={`#${h.id}`} className="text-muted-foreground transition-colors hover:text-foreground">
                    {h.text}
                  </a>
                </li>
              ))}
            </ul>
          </div>
        </aside>
      )}
    </div>
  );
}
