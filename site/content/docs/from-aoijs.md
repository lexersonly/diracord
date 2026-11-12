Se você já fez bot com aoi.js, quase tudo tem um equivalente direto, só que com variáveis, `if` e blocos de verdade.

## O atalho $ continua funcionando

Ligue no `diracord.toml`:

```toml
[syntax]
dollar = true
```

E escreva como estava acostumado:

```dcs
command("ban", target: member, reason: text = "no reason") {
  $ban[target;reason]
  $reply["Banned $target.name"]
}
```

Importante: `$nome[a;b]` chama a **função do Diracord** com esse nome. `$ban[target;reason]` é `ban(target, reason)`. Funções que só existem no aoi.js (como `$username`) não existem aqui: use a tabela abaixo. Quando quiser, desligue o atalho e rode `diracord check --fix`, que reescreve tudo para a forma normal.

## Equivalências

| aoi.js | Diracord |
|---|---|
| `client.command({ name: "ping", code: ... })` | `command("ping") { ... }` |
| `$sendMessage[hi]` | `reply "hi"` |
| `$channelSendMessage[id;hi]` | `send channel("general") "hi"` |
| `$username`, `$authorID` | `user.name`, `user.id` |
| `$mentioned[1]` / `$message[1]` | um argumento: `command("x", target: member)` |
| `$getUserVar[coins]` / `$setUserVar[coins;10]` | `get("coins", scope: user)` / `set("coins", 10, scope: user)` |
| `$getServerVar[...]` | `get(..., scope: server)` |
| `$if[...]`, `$onlyIf[...]` | `if ... { }` e `stop` |
| `$title[...]`, `$description[...]`, `$color[...]` | `reply { title: ..., description: ..., color: ... }` |
| `$addButton[...]` | `row(button("id", "Label"))` |
| `$random[1;6]` | `random(1, 6)` |
| `$toUppercase[...]`, `$replaceText[...]` | `upper(...)`, `replace(...)` |
| `$ban`, `$kick`, `$timeout` | `ban`, `kick`, `timeout` |
| interação de botão (`type: "interaction"`) | `button("id") { ... }` |

## O mesmo comando, antes e depois

aoi.js (simplificado):

```text
client.command({
  name: "coins",
  code: `
  $setUserVar[coins;$sum[$getUserVar[coins];1]]
  You have $getUserVar[coins] coins
  `
});
```

Diracord:

```dcs
command("coins") {
  local coins = get("coins", 0, scope: user) + 1
  set("coins", coins, scope: user)
  reply "You have $coins coins"
}
```

## O que muda na prática

- **Ordem:** o código roda de cima para baixo, uma instrução por linha.
- **Variáveis de verdade:** `local x = ...` em vez de repetir a mesma função `$` várias vezes.
- **Blocos:** `if`, `for`, `while`, `chain` e `testing` usam `{ }`, como na maioria das linguagens.
- **Erros antes de rodar:** `diracord check` mostra todos os problemas com seta e sugestão.
- **Slash de graça:** o mesmo `command` vira `/comando` sem configuração extra.
