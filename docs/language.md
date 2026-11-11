# Guia da linguagem

Este guia mostra o Diracord como ele funciona hoje (v0.1). As decisões de projeto
estão em [SPEC.md](SPEC.md); o que ainda está em aberto está marcado.

## Comandos

```dcs
command("ping") {
  reply "Pong!"
}
```

Com argumentos. Os tipos são `text`, `number`, `bool`, `member`, `user`, `channel`, `role`.
Argumento com `= valor` é opcional e fica no fim:

```dcs
command("ban", user: member, reason: text = "sem motivo") {
  ban user reason
  reply "$user.name foi banido. Motivo: $reason"
}
```

O mesmo código atende `!ban @Zé spam de link` (o último argumento `text` pega o resto
da linha) e `/ban user: @Zé reason: spam`. Os modos ficam em `[commands]` no `diracord.toml`.

## Eventos

O nome do evento, direto:

```dcs
memberJoin {
  local ch = channel("welcome")
  if ch {
    send ch "Bem-vindo, $user.mention!"
  }
}
```

Eventos: `ready`, `messageCreate`, `messageUpdate`, `messageDelete`, `memberJoin`,
`memberLeave`, `memberUpdate`, `reactionAdd`, `reactionRemove`, `channelCreate`,
`channelDelete`, `roleCreate`, `roleDelete`, `voiceJoin`, `voiceLeave`.

## Botões, menus e modais

O código do clique fica separado, ligado pelo id:

```dcs
button("confirm") {
  reply "Confirmado!" ephemeral: true
}

menu("cor") {
  reply "Você escolheu: " + join(message.values)
}
```

Os botões e menus são criados na mensagem com o layout do [Components v2](#components-v2).

## Chamadas com e sem parênteses

`reply "oi"` e `reply("oi")` são iguais. Sem parênteses, os argumentos são separados
por espaço e vão até o fim da linha:

```dcs
command("t", user: member) {
  timeout user 60 "flood"
  reply "Oi " + user.name
  reply upper("chamada dentro de chamada usa parênteses")
}
```

Em condições (`if`, `while`, `for`, `chain`) use parênteses: `if hasRole(user, "VIP") { }`.

## Texto e variáveis no texto

```dcs
command("oi") {
  local n = 3
  reply "Oi $user.name, você tem $n avisos. Custa \$5."
}
```

`$nome` e `$nome.campo` funcionam dentro do texto. Para contas, use uma variável ou `+`.

## Variáveis

```dcs
command("conta") {
  local x = 15
  x = x + 1
  local perfil = { nome: user.name, nivel: 1 }
  perfil.nivel = 2
  local lista = [1, 2, 3]
  lista[0] = 10
  reply "$x $perfil.nivel " + lista[0]
}
```

`local` cria (vale até o fim do bloco). `x = ...` muda uma variável que já existe.

## Condições e laços

```dcs
command("t", n: number) {
  if n > 5 {
    reply "grande"
  } else if n > 2 {
    reply "médio"
  } else {
    reply "pequeno"
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
    1 => reply "um"
    2 => reply "dois"
    _ => reply "outro"
  }
}
```

`chain` é o "switch" do Diracord. Cabe numa linha: `chain x { 1 => reply "um"  _ => stop }`.
`stop` encerra o comando; `break`/`continue` só valem em laços.

Operadores: `+ - * / %`, `== != < <= > >=`, `and`/`&&`, `or`/`||`, `not`/`!`.

## Funções próprias

Para não repetir código, crie funções com `fn`, no topo de qualquer arquivo:

```dcs
fn giveCoins(who: member, amount: number = 1) {
  local total = get("coins", 0, scope: user) + amount
  set("coins", total, scope: user)
  return total
}

fn greet(name) {
  return "Hi $name"
}

command("daily") {
  local total = giveCoins(user, 100)
  reply greet(user.name) + "! You have $total coins."
}
```

- Os tipos são **opcionais** e iguais aos de `command`. Com tipo, `"10"` vira `10`, e o erro
  de tipo aponta para a chamada.
- Chame como qualquer função: `giveCoins(user, 10)`, `giveCoins user 10` ou por nome,
  `giveCoins(who: user, amount: 10)`.
- A função só enxerga os próprios argumentos e as variáveis automáticas (`user`, `server`...),
  não os `local` de quem chamou: passe o que ela precisa como argumento.
- `return` devolve o resultado; `stop` dentro dela encerra o comando inteiro.

## Erros tratáveis

```dcs
command("ban", user: member) {
  testing {
    ban user
  } catch err {
    reply "Falhou: $err"
  }
}
```

> Quem vem do JavaScript costuma escrever `try`: o `check` avisa e o `--fix` troca por `testing`.

## Embeds

```dcs
command("perfil") {
  reply {
    title: "Perfil de $user.name"
    description: "Membro desde $user.joinedAt"
    color: "blurple"
    fields: [{ name: "Moedas", value: get("coins", 0, scope: user), inline: true }]
    footer: "Diracord"
  }
}
```

## Components v2

O layout novo do Discord (caixas, textos, imagens, botões e menus) é montado com
**peças**, no estilo do `ContainerBuilder` do discord.js: cada peça é uma função, e você
coloca uma dentro da outra.

```dcs
command("rules") {
  reply container(
    text("## Welcome, $user.name"),
    section("Read the rules before you continue.", thumbnail: user.avatar),
    separator(spacing: "large"),
    row(
      button("accept", "Accept", style: "success"),
      button("decline", "Decline", style: "danger")
    ),
    color: "purple"
  )
}

button("accept") {
  addRole user "Member"
  reply "Welcome!" ephemeral: true
}

button("decline") {
  reply "No problem." ephemeral: true
}
```

As peças são valores: dá para guardar em variáveis, montar com `if`/`for` e mandar
uma lista inteira:

```dcs
command("colors") {
  local colors = ["Blue", "Green", "Purple"]
  local parts = [text("Pick a color:"), row(menu("color", colors, placeholder: "Click here"))]
  if len(colors) > 2 {
    parts = push(parts, text("-# there are " + len(colors) + " options"))
  }
  reply parts
}

menu("color") {
  reply "You picked " + join(message.values) ephemeral: true
}
```

| peça | vai onde | argumentos |
|---|---|---|
| `container(peças..., color?, spoiler?)` | na mensagem | text, section, row, separator, gallery, file |
| `text(content)` | mensagem ou container | markdown |
| `section(textos..., thumbnail:, button:)` | mensagem ou container | 1–3 textos + **um** `thumbnail:` ou `button:` |
| `row(peças...)` | mensagem ou container | até 5 `button(...)`, ou um `menu(...)` |
| `button(id, label, style?, emoji?, disabled?, url?)` | `row` ou `section(button:)` | style: primary, secondary, success, danger |
| `menu(id, options, placeholder?, min?, max?)` | `row` | options: textos ou `{ label, value }` |
| `thumbnail(url, description?, spoiler?)` | `section(thumbnail:)` | ou passe só a url |
| `separator(spacing?, line?)` | mensagem ou container | spacing: "small" / "large" |
| `gallery(urls...)` | mensagem ou container | 1 a 10 imagens |
| `file(url, spoiler?)` | mensagem ou container | "attachment://nome" |

Dicas:

- `reply` aceita **uma peça**, **uma lista de peças** ou o objeto `{ layout: [...] }`.
- Botão de link: `button(label: "Website", url: "https://...")` (sem id).
- O clique do botão roda o `button("id") { }` com o mesmo id, escrito no topo do arquivo.
- Se uma peça estiver no lugar errado, a seta aponta para ela:
  `button(...)` dentro de `container(...)` → "vai dentro de `row(...)`" (E0117).
- Layout não se mistura com texto ou embed na mesma mensagem: para texto, use `text(...)`.

## Dados salvos

```dcs
command("coins") {
  set("coins", get("coins", 0, scope: user) + 1, scope: user)
  reply "Moedas: " + get("coins", scope: user)
}
```

Escopos: `user`, `server`, `member` (usuário dentro do servidor) e `global`. Sem `scope:`,
vale o `default_scope` do `diracord.toml`. Os dados ficam no **Dirabank**, um arquivo só
(`data/bot.dirabank`).

## Atalho estilo aoi.js

Com `[syntax] dollar = true`, `$ban[user;reason]` é o mesmo que `ban(user, reason)`.
Desligado (padrão), o `check` mostra o erro E0108 e o `--fix` reescreve para a forma normal.

## Limites

Cada execução tem limite de passos, tempo, memória e ações no Discord (veja
`[limits]` no `diracord.toml`). Um `while true { }` é parado com o erro E0205.

## Ainda não existe

Plugins WASM e funções como valor (`local f = giveCoins`). Ver [SPEC.md](SPEC.md) e [rfcs/](rfcs/).
