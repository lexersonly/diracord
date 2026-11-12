import type { ReactNode } from "react";
import type { SectionId } from "@/lib/nav";
import { Sidebar } from "./sidebar";

/** Sidebar on the left, page on the right (docs and components share it). */
export function Shell({ section, children }: { section: SectionId; children: ReactNode }) {
  return (
    <div className="mx-auto grid max-w-6xl gap-10 px-4 py-12 sm:px-6 lg:grid-cols-[220px_1fr]">
      <aside className="hidden lg:block">
        <div className="scrollbar-hide sticky top-24 max-h-[calc(100vh-7rem)] overflow-y-auto pb-10">
          <Sidebar section={section} />
        </div>
      </aside>
      <div className="min-w-0">{children}</div>
    </div>
  );
}
