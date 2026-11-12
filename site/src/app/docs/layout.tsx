import { Shell } from "@/components/docs/shell";

export default function Layout({ children }: LayoutProps<"/docs">) {
  return <Shell section="docs">{children}</Shell>;
}
