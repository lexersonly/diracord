import Link from "next/link";
import type { ReactNode } from "react";
import ReactMarkdown, { type Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import { Code } from "@/components/site/code";

/** Slug for heading anchors ("Primeiros passos" → "primeiros-passos"). */
export function slugify(text: string): string {
  return text
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/(^-|-$)/g, "");
}

function textOf(node: ReactNode): string {
  if (typeof node === "string" || typeof node === "number") return String(node);
  if (Array.isArray(node)) return node.map(textOf).join("");
  if (node && typeof node === "object" && "props" in node) {
    return textOf((node as { props: { children?: ReactNode } }).props.children);
  }
  return "";
}

const components: Components = {
  h2: ({ children }) => {
    const id = slugify(textOf(children));
    return (
      <h2 id={id} className="text-chrome mt-14 scroll-mt-24 text-2xl font-semibold tracking-tight first:mt-0">
        <a href={`#${id}`}>{children}</a>
      </h2>
    );
  },
  h3: ({ children }) => {
    const id = slugify(textOf(children));
    return (
      <h3 id={id} className="mt-10 scroll-mt-24 text-lg font-semibold tracking-tight">
        <a href={`#${id}`}>{children}</a>
      </h3>
    );
  },
  p: ({ children }) => <p className="mt-4 leading-7 text-muted-foreground">{children}</p>,
  ul: ({ children }) => <ul className="mt-4 list-disc space-y-2 pl-5 text-muted-foreground marker:text-white/30">{children}</ul>,
  ol: ({ children }) => <ol className="mt-4 list-decimal space-y-2 pl-5 text-muted-foreground marker:text-white/40">{children}</ol>,
  li: ({ children }) => <li className="leading-7">{children}</li>,
  strong: ({ children }) => <strong className="font-semibold text-foreground">{children}</strong>,
  a: ({ href = "", children }) =>
    href.startsWith("/") || href.startsWith("#") ? (
      <Link href={href} className="text-foreground underline decoration-white/30 underline-offset-4 hover:decoration-white">
        {children}
      </Link>
    ) : (
      <a href={href} target="_blank" rel="noreferrer" className="text-foreground underline decoration-white/30 underline-offset-4 hover:decoration-white">
        {children}
      </a>
    ),
  blockquote: ({ children }) => (
    <blockquote className="glass-thin mt-6 rounded-2xl px-5 py-1 text-sm [&_p]:text-foreground/80">{children}</blockquote>
  ),
  hr: () => <hr className="my-12 border-white/10" />,
  table: ({ children }) => (
    <div className="metal mt-6 overflow-x-auto rounded-2xl">
      <table className="w-full text-left text-sm">{children}</table>
    </div>
  ),
  th: ({ children }) => <th className="border-b border-white/10 px-4 py-3 font-medium text-foreground">{children}</th>,
  td: ({ children }) => <td className="border-b border-white/[0.05] px-4 py-3 align-top text-muted-foreground">{children}</td>,
  pre: ({ children }) => <div className="mt-6">{children}</div>,
  code: ({ className, children }) => {
    const lang = /language-([\w-]+)/.exec(className ?? "")?.[1];
    const text = String(children ?? "");
    if (!lang && !text.includes("\n")) {
      return <code className="rounded-md bg-white/[0.06] px-1.5 py-0.5 font-mono text-[0.86em] text-foreground">{children}</code>;
    }
    return <Code code={text} lang={lang ?? "text"} />;
  },
};

export function Markdown({ source }: { source: string }) {
  return (
    <ReactMarkdown remarkPlugins={[remarkGfm]} components={components}>
      {source}
    </ReactMarkdown>
  );
}

/** `## Headings` of a page, for the "nesta página" list. */
export function headings(source: string): { id: string; text: string }[] {
  const out: { id: string; text: string }[] = [];
  let inCode = false;
  for (const line of source.split("\n")) {
    if (line.startsWith("```")) inCode = !inCode;
    const m = !inCode && /^## (.+)$/.exec(line);
    if (m) {
      const text = m[1].replace(/`/g, "");
      out.push({ id: slugify(text), text });
    }
  }
  return out;
}
