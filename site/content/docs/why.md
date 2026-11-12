O Diracord foi feito para quem quer criar um bot sem virar programador primeiro, e sem bater no teto depois. Aqui estão as diferenças, inclusive onde as outras opções são melhores.

## Em uma frase

- **discord.js** é uma biblioteca de JavaScript: completa e poderosa, mas você precisa saber programar em JS.
- **aoi.js** coloca funções `$` dentro de uma string do JavaScript: fácil de começar, mas a lógica fica difícil de ler conforme o bot cresce.
- **Diracord** é uma linguagem própria para bots: tão curta quanto aoi.js, com variáveis, `if`, `for` e funções de verdade, e um verificador que acha os erros antes de o bot rodar.

## Lado a lado

| | Diracord | aoi.js | discord.js |
|---|---|---|---|
| O que você escreve | arquivos `.dcs` | `$funções` dentro de JS | JavaScript |
| `!prefixo` e `/slash` | o mesmo `command` | configurados separados | handlers separados + registro |
| Erros antes de rodar | `diracord check`, com seta e sugestão | em geral, ao rodar o comando | do editor/TypeScript, se você usar |
| Correção automática | `check --fix` para os seguros | não | não |
| Variáveis, `if`, `for`, funções | sim, com blocos `{ }` | com funções `$` | sim (é JavaScript) |
| Tratamento de erro | `testing { } catch err { }` | `$onlyIf`, `$suppressErrors` | `try/catch` |
| Banco de dados | embutido (Dirabank), com escopos | variáveis do aoi.db | você escolhe e configura |
| Limites de segurança por comando | sim (tempo, passos, memória, ações) | não | não |
| Components v2 | peças: `container(row(button()))` | funções `$add...` | builders |
| Acesso à API inteira do Discord | o que as funções cobrem | o que as funções cobrem | **tudo** |
| Ecossistema e comunidade | começando agora | grande | **enorme** |

## O que o Diracord faz diferente

### Um comando, dois jeitos de usar

```dcs
command("ban", user: member, reason: text = "no reason") {
  ban user reason
  reply "$user.name was banned. Reason: $reason"
}
```

Esse arquivo já é `!ban @Joe spam` **e** `/ban user: @Joe reason: spam`. O tipo `member` converte a menção ou o id no membro sozinho, e o slash command é registrado automaticamente com as opções certas.

### Erros que dizem o que fazer

```text
erro[E0201]: A função `rpely` não existe
 --> commands/ping.dcs:2:3
  |
2 |   rpely "Pong!"
  |   ^^^^^ função desconhecida
  |
  = ajuda: você quis dizer `reply`?
```

Todo erro tem um código estável (`E0201`) para pesquisar, mostra o arquivo e a linha com uma seta, e quase sempre sugere a correção. Os que são seguros (`let` → `local`, `=` → `==` no `if`, `memberjoin` → `memberJoin`...) o `diracord check --fix` corrige sozinho.

### Código que continua legível

Em aoi.js, uma condição dentro de uma ação dentro de uma mensagem vira funções `$` aninhadas numa string. Em Diracord, é o que parece:

```dcs
command("daily") {
  local last = get("daily", 0, scope: user)
  if now() - last < 86400000 {
    reply "Come back tomorrow!"
    stop
  }
  set("daily", now(), scope: user)
  set("coins", get("coins", 0, scope: user) + 100, scope: user)
  reply "You got 100 coins!"
}
```

### Leve e seguro

O motor é escrito em Rust e usa por volta de 2 MB de memória. Cada execução tem limite de passos, tempo, memória e ações no Discord: um `while true { }` esquecido para com o erro E0205, em vez de travar o bot.

## Quando escolher outra coisa

- Você precisa de algo que só a API completa do Discord tem (voz, música, sharding em bots gigantes): **discord.js**.
- Você já tem um bot grande em aoi.js funcionando e não quer reescrever: continue nele, ou migre aos poucos com o [guia do aoi.js](/docs/from-aoijs).

O Diracord está na versão 0.1. A linguagem e o motor são testados, mas ainda podem mudar.
