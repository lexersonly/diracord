Comandos que quase todo bot tem, prontos para copiar. Cada bloco é um arquivo `.dcs` completo, e acima dele está uma **simulação** de como fica no Discord.

## Ping e eco

```preview
{"messages": [{"from": "user", "text": "!ping"}, {"from": "bot", "text": "Pong!"}, {"from": "user", "text": "!say hello everyone"}, {"from": "bot", "text": "hello everyone"}]}
```

```dcs
command("ping") {
  reply "Pong!"
}

command("say", message: text) {
  reply message
}
```

No `!say`, o último argumento `text` pega o resto da linha: `!say hello everyone` responde `hello everyone`.

## Informações do usuário (embed)

```preview
{"messages": [{"from": "user", "text": "!userinfo @Joe"}, {"from": "bot", "embed": {"title": "About Joe", "description": "Joined at 2024-03-10T18:22:05+00:00", "color": "blurple", "fields": [{"name": "Id", "value": "123456789012345678", "inline": true}], "thumbnail": "x"}}]}
```

```dcs
command("userinfo", who: member = null) {
  local target = who
  if not target {
    target = user
  }
  reply {
    title: "About $target.name"
    description: "Joined at $target.joinedAt"
    thumbnail: target.avatar
    color: "blurple"
    fields: [{ name: "Id", value: target.id, inline: true }]
  }
}
```

Um argumento com `= null` é opcional. Se ninguém for mencionado, o comando mostra quem chamou.

## Dado, sorteio e escolha

```preview
{"messages": [{"from": "user", "text": "!dice"}, {"from": "bot", "text": "You rolled 4"}, {"from": "user", "text": "!choose pizza, burger, sushi"}, {"from": "bot", "text": "I choose: sushi"}]}
```

```dcs
command("dice", sides: number = 6) {
  reply "You rolled " + random(1, sides)
}

command("choose", options: text) {
  local list = split(options, ",")
  local pick = list[random(0, len(list) - 1)]
  reply "I choose: " + trim(pick)
}
```

`!choose pizza, burger, sushi` escolhe um dos três.

## Moderação com permissão

O Diracord ainda não tem uma checagem de permissão por comando: **confira o cargo antes de punir**, senão qualquer pessoa usa o `!ban`.

```preview
{"messages": [{"from": "user", "text": "!ban @Joe link spam", "name": "Mod"}, {"from": "bot", "text": "Joe was banned. Reason: link spam"}, {"from": "user", "text": "!ban @Mod"}, {"from": "bot", "text": "You need the Moderator role."}]}
```

```dcs
command("ban", target: member, reason: text = "no reason") {
  if not hasRole(user, "Moderator") {
    reply "You need the Moderator role." ephemeral: true
    stop
  }
  testing {
    ban target reason
    reply "$target.name was banned. Reason: $reason"
  } catch err {
    reply "Could not ban: $err"
  }
}

command("mute", target: member, minutes: number = 10) {
  if not hasRole(user, "Moderator") {
    stop
  }
  timeout target minutes * 60 "muted by $user.name"
  reply "$target.name is muted for $minutes minutes."
}
```

## Moedas e recompensa diária

```preview
{"messages": [{"from": "user", "text": "!daily"}, {"from": "bot", "text": "You got 100 coins!"}, {"from": "user", "text": "!daily"}, {"from": "bot", "text": "Come back tomorrow!"}, {"from": "user", "text": "!balance"}, {"from": "bot", "text": "You have 100 coins."}]}
```

```dcs
command("balance") {
  reply "You have " + get("coins", 0, scope: user) + " coins."
}

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

`now()` é a hora em milissegundos; 86400000 ms é um dia.

## Boas-vindas

```preview
{"messages": [{"from": "bot", "text": "Welcome, @Joe! Read the rules in #rules."}]}
```

```dcs
memberJoin {
  local ch = channel("welcome")
  if ch {
    send ch "Welcome, $user.mention! Read the rules in #rules."
  }
}
```

Crie um canal chamado `welcome` no servidor. Se ele não existir, `channel(...)` devolve `null` e o `if` evita o erro.

## Cargo por botão

```preview
{"messages": [{"from": "user", "text": "!roles"}, {"from": "bot", "layout": [{"container": [{"text": "## Pick your roles"}, {"row": [{"button": "role-games", "label": "Games"}, {"button": "role-art", "label": "Art"}]}], "color": "purple"}]}, {"from": "bot", "text": "You got the Games role.", "ephemeral": true}]}
```

```dcs
command("roles") {
  reply container(
    text("## Pick your roles"),
    row(button("role-games", "Games"), button("role-art", "Art")),
    color: "purple"
  )
}

button("role-games") {
  addRole user "Games"
  reply "You got the Games role." ephemeral: true
}

button("role-art") {
  addRole user "Art"
  reply "You got the Art role." ephemeral: true
}
```

## Dicas rápidas

- Use `npx diracord watch` enquanto escreve: o bot recarrega a cada arquivo salvo, e se tiver erro o código antigo continua rodando.
- `ephemeral: true` faz só quem usou o slash command ver a resposta.
- Envolva ações que podem falhar (ban, kick, addRole) em `testing { } catch err { }`.
- Para XP por servidor, use `scope: member` (o mesmo usuário tem XP diferente em cada servidor).
- Nomes de comando que vão virar slash precisam ser minúsculos, sem espaço (`user-info`, não `UserInfo`).
