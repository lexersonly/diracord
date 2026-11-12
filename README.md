<div align="center">

<img src="https://diracord.vercel.app/banner.svg" alt="Diracord: bots de Discord em texto simples" width="100%">

<br><br>

**Uma linguagem de texto para criar bots de Discord.**<br>
Mais fácil que aoi.js, com erros que dizem exatamente o que corrigir.

<br>

[![npm](https://img.shields.io/npm/v/diracord?style=flat-square&color=black&label=npm)](https://www.npmjs.com/package/diracord)
[![node](https://img.shields.io/node/v/diracord?style=flat-square&color=black)](https://nodejs.org)
[![licença MIT](https://img.shields.io/badge/licen%C3%A7a-MIT-black?style=flat-square)](https://github.com/lexersonly/diracord/blob/claude/keen-johnson-qfd683/LICENSE)
[![motor em Rust](https://img.shields.io/badge/motor-Rust-black?style=flat-square&logo=rust)](https://github.com/lexersonly/diracord)
[![RAM do motor](https://img.shields.io/badge/RAM_do_motor-~2.5_MB-black?style=flat-square)](https://github.com/lexersonly/diracord)

[**Site**](https://diracord.vercel.app) &nbsp;·&nbsp; [**Começar**](https://diracord.vercel.app/docs/getting-started) &nbsp;·&nbsp; [**Dicas e comandos**](https://diracord.vercel.app/docs/tips) &nbsp;·&nbsp; [**Vindo do aoi.js**](https://diracord.vercel.app/docs/from-aoijs) &nbsp;·&nbsp; [**GitHub**](https://github.com/lexersonly/diracord)

</div>

## ✨ Em 10 segundos

```dcs
command("ban", user: member, reason: text = "no reason") {
  testing {
    ban user reason
    reply "$user.name was banned. Reason: $reason"
  } catch err {
    reply "Could not ban: $err"
  }
}
```

Esse único arquivo já é **`!ban @Joe spam`** e **`/ban`**, converte a menção no membro,
trata o erro se o Discord recusar e valida tudo antes do bot ligar.

## 🚀 Começar

Precisa de **Node.js 22.18** ou mais novo e do token de um bot ([como criar](https://diracord.vercel.app/docs/getting-started)).

```bash
npx diracord new my-bot
cd my-bot
cp .env.example .env     # cole o token do bot depois do =
npx diracord run
```

Mande `!ping` no Discord e o bot responde `Pong!`. O projeto novo já vem com `ping`, moedas
salvas, mensagem com botão e boas-vindas.

| comando | o que faz |
|---|---|
| `npx diracord new <pasta>` | cria um projeto pronto para rodar |
| `npx diracord check` | procura erros no projeto inteiro, com seta e sugestão |
| `npx diracord check --fix` | corrige sozinho o que é seguro |
| `npx diracord run` | liga o bot |
| `npx diracord watch` | liga e recarrega a cada arquivo salvo |

## 🎯 Por que Diracord

| | Diracord | aoi.js | discord.js |
|---|:---:|:---:|:---:|
| Você escreve | arquivos `.dcs` | `$funções` em strings de JS | JavaScript |
| `!prefixo` e `/slash` no mesmo código | ✅ | ➖ | ➖ |
| Erros **antes** de rodar, com sugestão | ✅ | ❌ | ➖ |
| Correção automática (`--fix`) | ✅ | ❌ | ❌ |
| Variáveis, `if`, `for`, funções de verdade | ✅ | ➖ | ✅ |
| Banco de dados embutido | ✅ | ✅ | ❌ |
| Limites por comando (tempo, memória, ações) | ✅ | ❌ | ❌ |
| API completa do Discord | ➖ | ➖ | ✅ |

> Sem enrolação: se você precisa de voz, música ou sharding gigante, **discord.js** continua
> sendo a escolha certa. [Veja a comparação completa](https://diracord.vercel.app/docs/why).

## 🧩 Components v2 como peças

Caixas, textos, botões e menus montados como no `ContainerBuilder` do discord.js, mas sem `new` nem `.add...`:

```dcs
command("rules") {
  reply container(
    text("## Welcome, $user.name"),
    section("Read the rules before you continue.", thumbnail: user.avatar),
    separator(),
    row(
      button("accept", "I agree", style: "success"),
      button(label: "Website", url: "https://example.com")
    ),
    color: "blurple"
  )
}

button("accept") {
  addRole user "Member"
  reply "Welcome aboard!" ephemeral: true
}
```

Se uma peça estiver no lugar errado, a seta aponta para ela: *"`button` vai dentro de `row(...)`"*.

## 🩺 Erros que ensinam

```text
erro[E0201]: A função `rpely` não existe
 --> commands/ping.dcs:2:3
  |
2 |   rpely "Pong!"
  |   ^^^^^ função desconhecida
  |
  = ajuda: você quis dizer `reply`?
  = correção sugerida: trocar `rpely` por `reply`
```

Todo erro tem um **código estável** (`E0201`) para pesquisar, mostra o arquivo e a linha com
uma seta e quase sempre sugere a correção. Os seguros o `check --fix` aplica sozinho
(`let` → `local`, `=` → `==` no `if`, `try` → `testing`, `memberjoin` → `memberJoin`...).

## 🧰 O que vem pronto

| | |
|---|---|
| 💬 **54 funções** | mensagens, moderação, cargos, buscas, dados, texto, matemática, listas e Components v2 |
| 🗃️ **Dirabank** | banco embutido em **um arquivo só**, com escopos `user`, `server`, `member` e `global` |
| 🔁 **Funções próprias** | `fn giveCoins(who: member, amount: number = 1) { ... }`, com tipos opcionais |
| 🛡️ **Seguro por padrão** | limite de passos, tempo, memória e ações por comando: um `while true { }` não derruba o bot |
| ⚡ **Leve** | motor em Rust: ~2,5 MB de RAM e ~12 mil comandos por segundo (medido) |
| 🔌 **Sem discord.js** | conversa direto com o Gateway e a API do Discord, com cache mínimo |
| 🤖 **Feito para IAs** | [`llm.txt`](https://diracord.vercel.app/llm.txt) com a gramática completa, as funções e os erros comuns |

## 📚 Documentação

- **[Primeiros passos](https://diracord.vercel.app/docs/getting-started)**: do zero ao bot online
- **[Dicas e comandos básicos](https://diracord.vercel.app/docs/tips)**: ban, mute, moedas, boas-vindas, cargo por botão
- **[Bugs e erros comuns](https://diracord.vercel.app/docs/mistakes)**: o que costuma dar errado e como evitar
- **[Sintaxe](https://diracord.vercel.app/docs/syntax)** · **[Funções próprias](https://diracord.vercel.app/docs/functions)** · **[Components v2](https://diracord.vercel.app/docs/components-v2)** · **[Dados salvos](https://diracord.vercel.app/docs/data)**
- **Vindo de outra ferramenta:** [aoi.js](https://diracord.vercel.app/docs/from-aoijs) · [discord.js](https://diracord.vercel.app/docs/from-discordjs)
- **Referência:** [todas as funções](https://diracord.vercel.app/docs/reference-functions) · [códigos de erro](https://diracord.vercel.app/docs/reference-errors)

## 🛠️ Desenvolvimento

O motor é em **Rust** (`crates/`) e o resto em **TypeScript** (`packages/`). O site é um app
Next.js em `site/`.

```
crates/
  diracord-diagnostics   erros com seta, códigos estáveis e auto-fix
  diracord-ast           árvore sintática (todo nó tem posição)
  diracord-lexer         texto → tokens
  diracord-parser        tokens → árvore
  diracord-interpreter   executa a árvore, com limites e a trait Host
  diracord-functions     funções embutidas: um arquivo por função
  diracord-dirabank      banco chave-valor embutido
  diracord-engine        o motor como processo (protocolo JSON por linhas)
packages/                CLI, config, Discord (Gateway + REST) e o cliente do motor
site/                    o site e a documentação
docs/                    SPEC, arquitetura, protocolo e RFCs
```

```bash
cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings
cargo build -p diracord-engine && npm install && npm test && npm run typecheck
```

As listas de funções e de erros são **geradas do código**
(`cargo run -q -p diracord-functions --example gendocs`), e um teste garante que o
`llm.txt` e a documentação citam todas elas e que **todo exemplo ` ```dcs `** passa no `check`.

Mais: [arquitetura](docs/ARCHITECTURE.md) · [especificação](docs/SPEC.md) ·
[protocolo do motor](docs/engine-protocol.md) · [RFCs](docs/rfcs/)

---

<div align="center">

**Diracord** é código aberto sob a licença [MIT](LICENSE). Versão 0.1: a linguagem e o motor são testados,
mas ainda podem mudar. Achou um bug? [Abra uma issue](https://github.com/lexersonly/diracord/issues).

</div>
