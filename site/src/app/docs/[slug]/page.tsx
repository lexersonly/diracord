import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { Suspense } from "react";
import { headings, Markdown } from "@/components/docs/markdown";
import { Eyebrow } from "@/components/site/section";
import { allDocs, findDoc, groups, neighbours, readDoc } from "@/lib/docs";

export function generateStaticParams() {
  return allDocs.map((d) => ({ slug: d.slug }));
}

export async function generateMetadata({ params }: PageProps<"/docs/[slug]">): Promise<Metadata> {
  const { slug } = await params;
  const doc = findDoc(slug);
  return doc ? { title: doc.title, description: doc.description } : {};
}

export default function DocPage(props: PageProps<"/docs/[slug]">) {
  return (
    <Suspense fallback={<DocSkeleton />}>
      <DocContent params={props.params} />
    </Suspense>
  );
}

function DocSkeleton() {
  return (
    <div className="animate-pulse space-y-6">
      <div className="h-6 w-28 rounded-full bg-white/5" />
      <div className="h-12 w-2/3 rounded-xl bg-white/5" />
      <div className="h-5 w-1/2 rounded bg-white/5" />
      <div className="h-64 rounded-2xl bg-white/[0.03]" />
    </div>
  );
}

async function DocContent({ params }: { params: PageProps<"/docs/[slug]">["params"] }) {
  const { slug } = await params;
  const doc = findDoc(slug);
  if (!doc) notFound();
  const source = await readDoc(slug);
  const toc = headings(source);
  const group = groups.find((g) => g.pages.some((p) => p.slug === slug));
  const { prev, next } = neighbours(slug);
  return (
    <div className="grid gap-10 xl:grid-cols-[1fr_200px]">
      <article className="min-w-0">
        <header className="mb-10 space-y-4 border-b border-white/[0.06] pb-8">
          {group && <Eyebrow>{group.title}</Eyebrow>}
          <h1 className="text-chrome text-4xl font-semibold tracking-[-0.035em] sm:text-5xl">{doc.title}</h1>
          <p className="text-lg text-muted-foreground">{doc.description}</p>
        </header>
        <Markdown source={source} />
        <nav className="mt-16 grid gap-4 border-t border-white/[0.06] pt-8 sm:grid-cols-2">
          {prev ? (
            <Link href={`/docs/${prev.slug}`} className="metal rounded-2xl p-4 text-sm">
              <span className="text-muted-foreground">← Anterior</span>
              <p className="mt-1 font-medium">{prev.title}</p>
            </Link>
          ) : (
            <span />
          )}
          {next && (
            <Link href={`/docs/${next.slug}`} className="metal rounded-2xl p-4 text-right text-sm">
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
