import type { Metadata } from "next";
import { DocView } from "@/components/docs/doc-view";
import { findPage, pagesOf } from "@/lib/nav";

export function generateStaticParams() {
  return pagesOf("components").map((p) => ({ slug: p.slug }));
}

export async function generateMetadata({ params }: PageProps<"/components/[slug]">): Promise<Metadata> {
  const { slug } = await params;
  const page = findPage("components", slug);
  return page ? { title: page.title, description: page.description } : {};
}

export default function Page(props: PageProps<"/components/[slug]">) {
  return <DocView section="components" params={props.params} />;
}
