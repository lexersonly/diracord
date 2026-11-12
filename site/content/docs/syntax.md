Tudo que a linguagem tem, do mais simples ao mais avançado. Palavras-chave são em inglês e curtas; blocos usam `{ }`; comentários começam com `//`.

## Comandos

```dcs
command("ping") {
  reply "Pong!"
}
```

Com argumentos. Os tipos são `text`, `number`, `bool`, `member`, `user`, `channel` e `role`. Argumento com `= valor` é opcional e fica no fim:

```preview
{"messages": [{"from": "user", "text": "!ban @Joe link spam"}, {"from": "bot", "text": "Joe was banned. Reason: link spam"}, {"from": "bot", "command": "/ban user: @Joe", "text": "Joe was banned. Reason: no reason"}]}
```

```dcs
command("ban", user: member, reason: text = "no reason") {
  ban user reason
  reply "$user.name was banned. Reason: $reason"
}
```

O mesmo código atende `!ban @Joe link spam` (o último argumento `text` pega o resto da linha) e `/ban user: @Joe reason: link spam`.

## Eventos

O nome do evento, direto, sem palavra-chave:

```dcs
memberJoin {
  send channel("welcome") "Welcome, $user.mention!"
}
```

| evento | quando |
|---|---|
| `ready` | o bot ligou |
| `messageCreate`, `messageUpdate`, `messageDelete` | mensagem enviada, editada, apagada |
| `memberJoin`, `memberLeave`, `memberUpdate` | alguém entrou, saiu, mudou |
| `reactionAdd`, `reactionRemove` | reação colocada ou tirada (`reaction.emoji`) |
| `channelCreate`, `channelDelete` | canal criado ou apagado |
| `roleCreate`, `roleDelete` | cargo criado ou apagado (`role`) |
| `voiceJoin`, `voiceLeave` | entrou ou saiu de um canal de voz |

Vários arquivos podem ouvir o mesmo evento: todos rodam.

## Botões, menus e modais

O código do clique fica separado, ligado pelo **id**:

```preview
{"messages": [{"from": "bot", "layout": [{"row": [{"button": "confirm", "label": "Confirm", "style": "success"}]}]}, {"from": "bot", "text": "Confirmed!", "ephemeral": true}]}
```

```dcs
button("confirm") {
  reply "Confirmed!" ephemeral: true
}

menu("color") {
  reply "You picked " + join(message.values)
}
```

Para criar os botões na mensagem, veja [Components v2](/components/overview).

## Chamadas com e sem parênteses

`reply "hi"` e `reply("hi")` são a mesma coisa. Sem parênteses, os argumentos são separados por espaço e vão até o fim da linha:

```dcs
command("t", target: member) {
  timeout target 60 "flood"
  reply "Hi " + target.name
  reply upper("a call inside a call needs parentheses")
}
```

Duas regras para lembrar:

- **Chamada dentro de chamada usa parênteses:** `reply upper("hi")`, não `reply upper "hi"`.
- **Em condições, use parênteses:** `if hasRole(user, "VIP") { }`.

Argumentos podem ser passados pelo nome: `get("coins", 0, scope: user)`.

## Texto

Texto usa aspas duplas e aceita variáveis com `$`:

```preview
{"messages": [{"from": "user", "text": "!hi"}, {"from": "bot", "text": "Hi Ana, you have 3 warnings. It costs $5."}]}
```

```dcs
command("hi") {
  local n = 3
  reply "Hi $user.name, you have $n warnings. It costs \$5."
}
```

`$nome` e `$nome.campo` funcionam dentro do texto. Para contas, crie uma variável antes ou use `+`. Escapes: `\n` (nova linha), `\t`, `\"`, `\\` e `\$`.

## Variáveis

```dcs
command("math") {
  local x = 15
  x = x + 1
  local profile = { name: user.name, level: 1 }
  profile.level = 2
  local list = [1, 2, 3]
  list[0] = 10
  reply "$x $profile.level " + list[0]
}
```

- `local` **cria** a variável (ela vale até o fim do bloco `{ }`).
- `x = ...` **muda** uma variável que já existe.
- Tipos: `null`, `true`/`false`, números, texto, listas `[ ]` e objetos `{ }`.

## Condições, laços e chain

```preview
{"messages": [{"from": "user", "text": "!size 2"}, {"from": "bot", "text": "small"}, {"from": "bot", "text": "1"}, {"from": "bot", "text": "3"}, {"from": "bot", "text": "two"}]}
```

```dcs
command("size", n: number) {
  if n > 5 {
    reply "big"
  } else if n > 2 {
    reply "medium"
  } else {
    reply "small"
  }

  for i in range(1, 4) {
    if i == 2 { continue }
    reply i
  }

  local k = 0
  while k < 3 {
    k = k + 1
  }

  chain n {
    1 => reply "one"
    2 => reply "two"
    _ => reply "other"
  }
}
```

- `chain` é o "switch" do Diracord: o primeiro caso igual ganha, `_` é "qualquer outro" e vem por último. Cabe numa linha: `chain x { 1 => reply "one"  _ => stop }`.
- `for x in` funciona com listas, texto (letra por letra), números (`for i in 5` vai de 0 a 4) e objetos (as chaves).
- `stop` encerra o comando. `break` e `continue` só valem dentro de `for`/`while`.
- Falso é: `null`, `false`, `0`, `""` e `[]`. Todo o resto é verdadeiro.

Operadores: `+ - * / %`, `== != < <= > >=`, `and`/`&&`, `or`/`||`, `not`/`!`.

## Erros tratáveis

```preview
{"messages": [{"from": "user", "text": "!kick @Admin"}, {"from": "bot", "text": "Failed: O Discord recusou `kick`: Missing Permissions"}]}
```

```dcs
command("kick", target: member) {
  testing {
    kick target
    reply "Done."
  } catch err {
    reply "Failed: $err"
  }
}
```

`err` é o texto do erro. Erros de limite (tempo, passos, memória, ações) **não** podem ser pegos: eles param o comando de propósito.

## Embeds

```preview
{"messages": [{"from": "user", "text": "!profile"}, {"from": "bot", "embed": {"title": "Profile of Ana", "description": "Member since 2024-03-10T18:22:05+00:00", "color": "blurple", "fields": [{"name": "Coins", "value": "120", "inline": true}], "footer": "Diracord"}}]}
```

```dcs
command("profile") {
  reply {
    title: "Profile of $user.name"
    description: "Member since $user.joinedAt"
    color: "blurple"
    fields: [{ name: "Coins", value: get("coins", 0, scope: user), inline: true }]
    footer: "Diracord"
  }
}
```

Campos: `title`, `description`, `url`, `color` (nome, `"#rrggbb"` ou número), `footer`, `author`, `image`, `thumbnail`, `fields` e `timestamp: true`.

## Variáveis automáticas

Dentro de comandos, eventos e botões já existem:

- `user` (ou `author`): quem usou. Campos: `id`, `name`, `username`, `mention`, `bot`, `avatar`; em servidores também `nick`, `roles`, `joinedAt`.
- `server`: o servidor (`null` em mensagem privada).
- `channel`: o canal.
- `message` (ou `interaction`): a mensagem ou a interação. Em menus, `message.values` tem o que foi escolhido.

## Atalho estilo aoi.js

Com `[syntax] dollar = true` no `diracord.toml`, `$ban[user;reason]` é o mesmo que `ban(user, reason)`. Veja o [guia para quem vem do aoi.js](/docs/from-aoijs).

## Limites

Cada execução tem limite de passos (100 mil), tempo de CPU (3 s, esperar o Discord não conta), memória (4 MB) e ações no Discord (50). Os valores mudam em `[limits]` no `diracord.toml`.
