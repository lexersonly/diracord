"use client";

import { AnimatedNumber } from "@/components/motion/animated-number";
import { SectionTitle } from "@/components/site/section";
import { site } from "@/lib/site";

const STATS = [
  { value: site.engineRamMb, label: "MB de RAM no motor", format: (n: number) => n.toFixed(1) },
  { value: site.functions, label: "funções embutidas", format: (n: number) => Math.round(n).toString() },
  { value: site.errorCodes, label: "códigos de erro documentados", format: (n: number) => Math.round(n).toString() },
  { value: 0, label: "dependências em runtime", format: () => "0" },
];

export function Stats() {
  return (
    <section className="relative overflow-hidden border-y border-white/[0.06] bg-white/[0.012]">
      <div className="mx-auto grid max-w-6xl gap-12 px-4 py-24 sm:px-6 md:grid-cols-[0.9fr_1.1fr] md:items-center">
        <SectionTitle
          eyebrow="Números"
          title={
            <>
              Pequeno por dentro,
              <br />
              <span className="text-chrome-dim">completo por fora</span>
            </>
          }
          sub={`Medido no motor em release: ${site.runsPerSecond.toLocaleString("pt-BR")} comandos por segundo com ida e volta pelo protocolo, binário de ${site.engineBinaryKb} KB.`}
        />
        <div className="grid grid-cols-2 gap-4">
          {STATS.map((s, i) => (
            <div
              key={s.label}
              className="metal rounded-3xl p-6"
              style={{ transform: `rotate(${i % 2 ? 2 : -2}deg)` }}
            >
              <p className="text-chrome text-5xl font-semibold tracking-tight">
                <AnimatedNumber value={s.value} format={s.format} />
              </p>
              <p className="mt-2 text-sm text-muted-foreground">{s.label}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
