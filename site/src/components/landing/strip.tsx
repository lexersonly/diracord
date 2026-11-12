import { Marquee } from "@/components/motion/marquee";

const ITEMS = [
  "!prefixo e /slash no mesmo código",
  "Erros com seta e sugestão",
  "diracord check --fix",
  "Components v2 em peças",
  "Banco de dados embutido",
  "Funções próprias com fn",
  "Motor em Rust",
  "Sem discord.js",
  "Limites de segurança",
  "llm.txt para IAs",
];

export function Strip() {
  return (
    <div className="border-y border-white/[0.06] bg-white/[0.015] py-5">
      <Marquee speed={40} gap="3rem" fade pauseOnHover>
        {ITEMS.map((t) => (
          <span key={t} className="flex items-center gap-3 whitespace-nowrap text-sm text-muted-foreground">
            <svg viewBox="0 0 10 10" className="size-2.5 fill-white/70" aria-hidden>
              <path d="M5 0C5.3 3.3 6.7 4.7 10 5C6.7 5.3 5.3 6.7 5 10C4.7 6.7 3.3 5.3 0 5C3.3 4.7 4.7 3.3 5 0Z" />
            </svg>
            {t}
          </span>
        ))}
      </Marquee>
    </div>
  );
}
