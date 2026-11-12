import Link from "next/link";
import { ButtonLink } from "@/components/motion/button/base";
import { site } from "@/lib/site";
import { Logo } from "./logo";

const NAV = [
  { href: "/docs", label: "Docs" },
  { href: "/components", label: "Componentes" },
  { href: "/docs/tips", label: "Comandos prontos" },
  { href: "/docs/why", label: "Por que Diracord" },
];

export function Header() {
  return (
    <header className="sticky top-0 z-50 border-b border-white/[0.06] bg-background/70 backdrop-blur-xl">
      <div className="mx-auto flex h-14 max-w-6xl items-center justify-between gap-4 px-4 sm:px-6">
        <Link href="/" className="flex items-center gap-2 text-sm font-semibold tracking-tight">
          <Logo className="size-6" />
          {site.name}
        </Link>
        <nav className="hidden items-center gap-6 text-[13px] text-muted-foreground md:flex">
          {NAV.map((n) => (
            <Link key={n.href} href={n.href} className="transition-colors hover:text-foreground">
              {n.label}
            </Link>
          ))}
        </nav>
        <div className="flex items-center gap-2">
          <ButtonLink href={site.repo} variant="ghost" size="sm" target="_blank" rel="noreferrer">
            GitHub
          </ButtonLink>
          <ButtonLink href="/docs/getting-started" size="sm">
            Começar
          </ButtonLink>
        </div>
      </div>
      {/* Small screens: the same links in a row that scrolls sideways. */}
      <nav className="scrollbar-hide flex gap-5 overflow-x-auto border-t border-white/[0.06] px-4 py-2.5 text-[13px] text-muted-foreground md:hidden">
        {NAV.map((n) => (
          <Link key={n.href} href={n.href} className="whitespace-nowrap hover:text-foreground">
            {n.label}
          </Link>
        ))}
      </nav>
    </header>
  );
}
