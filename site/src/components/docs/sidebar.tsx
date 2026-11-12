"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { groups } from "@/lib/docs-nav";
import { cn } from "@/lib/utils";

export function Sidebar() {
  const pathname = usePathname();
  return (
    <nav className="space-y-8 text-sm">
      {groups.map((g) => (
        <div key={g.title} className="space-y-2">
          <p className="px-3 text-xs font-medium uppercase tracking-wider text-white/40">{g.title}</p>
          <ul className="space-y-0.5">
            {g.pages.map((p) => {
              const href = `/docs/${p.slug}`;
              const active = pathname === href;
              return (
                <li key={p.slug}>
                  <Link
                    href={href}
                    className={cn(
                      "block rounded-lg px-3 py-1.5 transition-colors",
                      active ? "bg-white/[0.07] text-foreground" : "text-muted-foreground hover:text-foreground",
                    )}
                  >
                    {p.title}
                  </Link>
                </li>
              );
            })}
          </ul>
        </div>
      ))}
    </nav>
  );
}
