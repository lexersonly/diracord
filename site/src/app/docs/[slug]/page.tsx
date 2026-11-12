import type { Metadata } from "next";
import { DocView } from "@/components/docs/doc-view";
import { findPage, pagesOf } from "@/lib/nav";

export function generateStaticParams() {
  return pagesOf("docs").map((p) => ({ slug: p.slug }));
}

export async function generateMetadata({ params }: PageProps<"/docs/[slug]">): Promise<Metadata> {
  const { slug } = await params;
  const page = findPage("docs", slug);
  return page ? { title: page.title, description: page.description } : {};
}

export default function Page(props: PageProps<"/docs/[slug]">) {
  return <DocView section="docs" params={props.params} />;
}
