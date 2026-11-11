# Diracord

Uma linguagem de texto para criar bots de Discord: **mais fácil** que aoi.js e
discord.js, **mais completa** (variáveis, condições, laços, erros tratáveis) e
**leve** (motor em Rust, meta de rodar em ~100 MB de RAM).

```dcs
command("ban", user: member, reason: text = "sem motivo") {
  try {
    ban user reason
    reply "$user.name foi banido. Motivo: $reason"
  } catch err {
    reply "Não consegui: $err"
  }
}
```

O mesmo comando funciona como `!ban @Zé spam` e como `/ban`.

> Status: **v0.1, em desenvolvimento**. Ainda não publicado no npm.

## Começar

Requisitos: Rust (stable) e Node.js 22.18+.

```bash
cargo build --release -p diracord-engine
npm install
node packages/cli/src/main.ts new meu-bot
cd meu-bot && cp .env.example .env   # coloque o token no .env
node ../packages/cli/src/main.ts run
```

Guia completo: [docs/getting-started.md](docs/getting-started.md).

## Erros que ajudam

```text
erro[E0201]: A função `rpely` não existe
 --> commands/ping.dcs:2:3
  |
2 |   rpely "Pong!"
  |   ^^^^^ função desconhecida
  |
  = ajuda: você quis dizer `reply`?
```

`diracord check --fix` aplica as correções que são seguras. Todos os códigos:
[docs/errors.md](docs/errors.md).

## Documentação

| | |
|---|---|
| [docs/SPEC.md](docs/SPEC.md) | a linguagem (decisões do projeto) |
| [docs/language.md](docs/language.md) | guia da linguagem com exemplos |
| [docs/functions.md](docs/functions.md) | todas as funções (gerado do código) |
| [docs/errors.md](docs/errors.md) | todos os códigos de erro (gerado do código) |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | crates, pacotes e modos do motor |
| [docs/engine-protocol.md](docs/engine-protocol.md) | protocolo entre o motor (Rust) e o TypeScript |
| [docs/rfcs/](docs/rfcs/) | propostas em aberto |
| [llm.txt](llm.txt) | referência compacta para IAs lerem e escreverem Diracord |

## Desenvolvimento

```bash
cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings
cargo build -p diracord-engine && npm test && npm run typecheck
```

Docs geradas: `cargo run -q -p diracord-functions --example gendocs > docs/functions.md`
e `cargo run -q -p diracord-diagnostics --example gencodes > docs/errors.md`. Um teste
garante que `llm.txt` e `docs/` citam toda função e todo código, e que todo exemplo
` ```dcs ` dos docs passa no `check`.

Licença: MIT.
