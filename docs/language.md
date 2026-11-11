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

## Erros tratáveis

```dcs
command("ban", user: member) {
  try {
    ban user
  } catch err {
    reply "Falhou: $err"
  }
}
```

> O nome `try/catch` é provisório (SPEC: `try/catch` ou `testing`, em aberto).

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

O layout novo do Discord (caixas, textos, imagens, botões e menus) é escrito em blocos,
aberto pela palavra `layout`:

```dcs
command("regras") {
  reply layout {
    container(color: "purple") {
      text "## Bem-vindo, $user.name"
      section {
        text "Leia as regras antes de continuar."
        thumbnail "https://cdn.discordapp.com/embed/avatars/0.png"
      }
      separator(spacing: "large")
      row {
        button("aceitar", "Aceito", style: "success")
        button("recusar", "Não aceito", style: "danger")
      }
    }
  }
}

button("aceitar") {
  addRole user "Membro"
  reply "Bem-vindo!" ephemeral: true
}

button("recusar") {
  reply "Tudo bem." ephemeral: true
}
```

Dentro do layout valem `local`, `if` e `for`:

```dcs
command("cores") {
  local cores = ["Azul", "Verde", "Roxo"]
  reply layout {
    text "Escolha uma cor:"
    row {
      menu("cor", cores, placeholder: "Clique aqui")
    }
    if len(cores) > 2 {
      local n = len(cores)
      separator
      text "-# são $n opções"
    }
  }
}

menu("cor") {
  reply "Você escolheu " + join(message.values) ephemeral: true
}
```

| componente | onde | argumentos |
|---|---|---|
| `container { }` | topo | `color?`, `spoiler?` |
| `section { }` | topo ou container | 1–3 `text` + um `thumbnail` ou `button` |
| `row { }` | topo ou container | até 5 `button`, ou um `menu` |
| `text` | topo, container, section | `content` (markdown) |
| `thumbnail` | section | `url`, `description?`, `spoiler?` |
| `separator` | topo ou container | `spacing?` ("small"/"large"), `line?` |
| `gallery` | topo ou container | `items` (1–10 urls) |
| `file` | topo ou container | `url` ("attachment://..."), `spoiler?` |
| `button` | row ou section | `id?`, `label?`, `style?`, `emoji?`, `disabled?`, `url?` |
| `menu` | row | `id`, `options`, `placeholder?`, `min?`, `max?` |

Componentes sem bloco aceitam opções pelo nome: `separator(spacing: "large", line: false)`.
Com `url:` o botão é um link: `button(label: "Site", url: "https://...")`.

A mesma mensagem também pode ser escrita como objeto (útil para montar por código):

```dcs
command("oi") {
  reply {
    layout: [
      { container: [
          { text: "## Oi" },
          { row: [{ button: "aceitar", label: "Aceito", style: "success" }] }
        ], color: "purple" }
    ]
  }
}
```

Erros comuns: botão fora de `row` (E0117), `section` sem acessório, nome errado
(`txet` → `text`, E0118). Tudo é conferido pelo `check` e antes de enviar.

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

## Ainda em aberto

Funções próprias (`fn`), nome do `try/catch`, alias `guild`,
formato final do Dirabank. Ver [SPEC.md §10](SPEC.md) e [rfcs/](rfcs/).
