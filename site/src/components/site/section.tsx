import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

/** The small pill above section titles ("● Recursos"), as in the reference. */
export function Eyebrow({ children }: { children: ReactNode }) {
  return (
    <span className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/[0.03] px-3 py-1 text-xs text-muted-foreground">
      <span className="size-1.5 rounded-full bg-white/80 shadow-[0_0_10px_rgba(255,255,255,0.8)]" />
      {children}
    </span>
  );
}

export function SectionTitle({
  eyebrow,
  title,
  sub,
  className,
  align = "left",
}: {
  eyebrow: string;
  title: ReactNode;
  sub?: ReactNode;
  className?: string;
  align?: "left" | "center";
}) {
  return (
    <div className={cn("space-y-4", align === "center" && "mx-auto text-center", className)}>
      <Eyebrow>{eyebrow}</Eyebrow>
      <h2 className="text-chrome max-w-2xl text-balance text-4xl font-semibold leading-[1.05] tracking-[-0.03em] sm:text-5xl">
        {title}
      </h2>
      {sub && <p className={cn("max-w-xl text-pretty text-muted-foreground", align === "center" && "mx-auto")}>{sub}</p>}
    </div>
  );
}
