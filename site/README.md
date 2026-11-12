# Site do Diracord

Next.js 16 + Tailwind 4 + componentes do [beUI](https://beui.dev) (via shadcn CLI).

```bash
npm install
npm run dev      # http://localhost:3000
npm run build
```

- `content/docs/*.md`: páginas da documentação (prosa em português, código em inglês).
  Todo bloco ```dcs é conferido pelo `diracord check` nos testes do repositório
  (`cargo test -p diracord-engine --test docs_sync`).
- `npm run sync-docs` copia as referências geradas (`docs/functions.md`, `docs/errors.md`,
  `llm.txt`) para o site; `dev` e `build` já rodam isso.
- `src/components/motion` e `src/components/agents`: componentes do beUI, como o registro
  publica (fora do lint).
- Deploy: Vercel, com *Root Directory* = `site`.
