import { Sidebar } from "@/components/docs/sidebar";

export default function DocsLayout({ children }: LayoutProps<"/docs">) {
  return (
    <div className="mx-auto grid max-w-6xl gap-10 px-4 py-12 sm:px-6 lg:grid-cols-[220px_1fr]">
      <aside className="hidden lg:block">
        <div className="sticky top-24 max-h-[calc(100vh-7rem)] overflow-y-auto pb-10 scrollbar-hide">
          <Sidebar />
        </div>
      </aside>
      <div className="min-w-0">{children}</div>
    </div>
  );
}
