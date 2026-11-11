# Diracord — Arquitetura

## 1. Metas que guiam tudo
1. **Pouca RAM**: um host de ~100 MB deve rodar bots com 1000+ usuários.
2. **Velocidade**: o motor (parser + interpretador) é Rust.
3. **Modular**: nada de arquivo gigante. Pastas e crates pequenas; API pública
   separada da privada.
4. **Documentado**: rustdoc e TSDoc em toda API pública, mais `docs/` e `llm.txt`.

## 2. Visão geral

```
 código .dcs ──► [ Rust: lexer → parser → interpretador ] ──► ações
                                   │                           │
                                   ▼                           ▼
                             Dirabank (dados)          TypeScript (Discord)
```

- **Rust** (`crates/`): linguagem, motor, limites, banco de dados.
- **TypeScript** (`packages/`): CLI, config, conexão com o Discord (API direta, sem
  discord.js, para economizar RAM) e a ponte com o motor.
- O motor **não fala com o Discord**. Ele chama um `Host` (trait Rust) para ações
  como `reply` e `ban`. O TypeScript implementa o `Host`. Isso separa "cérebro" de
  "mãos" e permite rodar o motor embutido ou como serviço.

## 3. Crates Rust (workspace)

| Crate | Responsabilidade |
|---|---|
| `diracord-diagnostics` | `Span`, `Diagnostic`, render de erros com trecho e seta, auto-fix |
| `diracord-ast` | Árvore sintática (contrato entre parser e interpretador) |
| `diracord-value` | `Value` de runtime compartilhado |
| `diracord-lexer` | Texto → tokens (aceita `{}` , `$`, strings com `$var`) |
| `diracord-parser` | Tokens → AST. Desfaz o atalho `$ban[...]` conforme a config |
| `diracord-interpreter` | Executa a AST. Limites de tempo/passos/memória. Trait `Host` |
| `diracord-functions` | Funções embutidas: **uma pasta por categoria, um arquivo por função** (`moderation/ban.rs`) |
| `diracord-dirabank` | Banco chave-valor embutido, arquivo único, driver plugável |

Regras de dependência: `diagnostics` não depende de nada; `ast` e `value` só dependem
de `diagnostics`; `lexer`/`parser` nunca dependem do `interpreter`.

## 4. Pacotes TypeScript (planejados)

| Pacote | Responsabilidade |
|---|---|
| `@diracord/cli` | `run`, `check`, `new`, `watch` |
| `@diracord/config` | Lê `diracord.toml` e `.env` |
| `@diracord/discord` | Gateway e REST diretos (cache mínimo), implementa o `Host` |
| `@diracord/engine` | Binding do motor Rust (napi-rs) |

O pacote principal publicado no npm se chama `diracord` (nome livre em 2026-11-11).

## 5. Como o motor roda (modos)
Mesmo código, escolhido por configuração:
1. **Embutido** (napi-rs): o motor roda dentro do processo Node.
2. **Serviço**: processo Rust separado, falando com o TS por socket local. Serve para
   hospedagem, no estilo "cérebro e mãos".
3. **Gateway em Rust** (fase futura): um binário Rust fala com o Discord e roda os bots.
   É o caminho mais leve para 100 MB.

`ram = "100mb"` em `diracord.toml` (ideia `starter_with`): o sistema **calcula
sozinho** o melhor modo, o número de bots por processo e o tamanho de cache.

## 6. Isolamento
Pedido: um processo por bot. Para caber em pouca RAM, o processo por bot deve ser um
binário Rust minúsculo. Com Node, não cabe. O modo é escolhido por `ram`.
Em qualquer modo: limite de tempo, de passos e de memória por execução.

## 7. Dados
O `Host` e o interpretador usam o Dirabank por uma interface (`trait Store`). Escopos:
`user`, `server`, `member`, `global`. Chave = escopo + id + nome.

## 8. Extensões
Plugins WASM registram funções novas. Eles são isolados, com memória e tempo limitados.

## 9. Documentação e IA
- rustdoc / TSDoc em tudo que é público.
- `docs/` em Markdown para usuários.
- Docs de funções **geradas a partir do código**.
- `llm.txt` na raiz: gramática completa, funções e erros comuns, para IAs lerem e
  escreverem Diracord corretamente.

## 10. Fases
1. Contratos (`ast`, `diagnostics`, `value`) e spec.
2. Lexer e parser, com testes e bons erros.
3. Interpretador, limites e `Host`.
4. Dirabank e funções embutidas.
5. TypeScript: config, CLI, Discord direto, binding napi.
6. Components v2, plugins WASM, modo serviço, gateway em Rust.
