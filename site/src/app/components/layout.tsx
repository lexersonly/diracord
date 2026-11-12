import { Shell } from "@/components/docs/shell";

export default function Layout({ children }: LayoutProps<"/components">) {
  return <Shell section="components">{children}</Shell>;
}
