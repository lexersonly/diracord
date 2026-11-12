/** Facts shown on the site. Numbers are measured on the engine (see README). */
export const site = {
  name: "Diracord",
  url: "https://diracord.vercel.app",
  repo: "https://github.com/lexersonly/diracord",
  version: "0.1.0",
  tagline: "Bots de Discord em texto simples.",
  description:
    "Diracord é uma linguagem para criar bots de Discord: mais fácil que aoi.js, mais completa que um amontoado de $funções, e com erros que dizem exatamente o que corrigir.",
  /** Measured with the release build: `diracord-engine` resident memory while running 5000 commands. */
  engineRamMb: 2.5,
  /** Release binary size (KB). */
  engineBinaryKb: 987,
  /** Commands per second through the JSON protocol (one core, release build). */
  runsPerSecond: 12000,
  functions: 54,
  errorCodes: 47,
} as const;
