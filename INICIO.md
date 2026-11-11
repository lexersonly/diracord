# INICIO — leia isto primeiro

Você é um agente de IA trabalhando no **Diracord**: uma linguagem de texto para bots
de Discord, mais fácil e mais completa que aoi.js. Este arquivo diz o que você deve
fazer. Leia depois `docs/SPEC.md` (a linguagem) e `docs/ARCHITECTURE.md` (o projeto).

## Contexto
- O dono começou com ideias soltas e responde melhor a **escolhas concretas**.
  Quando faltar decisão, **pergunte com opções**, não invente.
- O dono fala português informal. Responda em português, direto, sem enrolar.
- O repositório é `lexersonly/diracord`. A branch de trabalho é a que o ambiente indicar.

## O que você deve construir
Um monorepo com **core em Rust** e **o resto em TypeScript**:

1. `crates/diracord-diagnostics`, `-ast`, `-value`: contratos compartilhados.
2. `crates/diracord-lexer` e `-parser`: aceitar `{}` e `$`, strings com `$var`,
   chamadas com e sem parênteses, atalho `$ban[...]` ligado por config.
3. `crates/diracord-interpreter`: executar a AST, com limites de tempo/passos/memória
   e a trait `Host` (o motor nunca fala com o Discord).
4. `crates/diracord-functions`: uma pasta por categoria, **um arquivo por função**.
5. `crates/diracord-dirabank`: chave-valor embutido, arquivo único, driver plugável.
6. `packages/*` em TypeScript: CLI, config, Discord direto (sem discord.js), binding
   napi-rs.
7. Documentação: `docs/`, rustdoc/TSDoc, **`llm.txt`** na raiz e docs de funções geradas.

## Como trabalhar
- **Contratos primeiro.** Defina `ast`, `diagnostics` e `value` antes de paralelizar.
- **Use subagentes em paralelo** nas partes da linguagem (lexer+parser, interpretador,
  Dirabank, TypeScript, docs). Cada agente é dono de **uma crate/pasta** e não edita
  as dos outros. Não construa o bot inteiro de uma vez: entregue a base.
- **Modular sempre.** Arquivos pequenos. Separe API pública (`pub`) da privada. Nada de
  arquivo gigante com tudo.
- **Erros são produto.** Todo erro: mensagem simples, trecho com seta, sugestão,
  auto-fix quando seguro, código estável (`E0001`).
- **Pouca RAM.** Nada de dependência pesada sem motivo. Cache do Discord mínimo.
- **Documente** o que for público, em rustdoc/TSDoc.
- **Teste.** Cada crate com testes. Rode `cargo test` e `cargo clippy` antes de commitar.
- Não faça `unsafe` sem comentar o motivo.

## O que NÃO fazer
- Não invente sintaxe das decisões **[ABERTO]** da SPEC. Pergunte ou escreva um RFC em
  `docs/rfcs/` com 3 propostas.
- Não coloque token, senha ou dados pessoais no repositório ou em pedidos externos.
- Não publique no npm sem o dono confirmar. O nome `diracord` estava livre em
  2026-11-11. O login do npm do dono está na máquina dele, não no container da nuvem.
- Não abra Pull Request sem o dono pedir.

## Ordem sugerida
1. Contratos (`ast`, `diagnostics`, `value`) e testes mínimos.
2. Em paralelo: lexer+parser, interpretador+`Host`, Dirabank, TypeScript, docs/`llm.txt`.
3. Integrar: `parser → interpreter → Host` com um comando `ping` de ponta a ponta.
4. Só então: Discord real, Components v2, plugins WASM, modo serviço.

## Pronto quando
- `cargo test` passa em todo o workspace.
- `diracord check` mostra erros claros em um arquivo com erro de propósito.
- Um `command("ping") { reply "Pong!" }` roda de ponta a ponta com um `Host` de teste.
- `llm.txt` existe e bate com a SPEC.
- Tudo commitado e enviado para a branch de trabalho.
