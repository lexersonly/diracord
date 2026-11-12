O embed é o cartão clássico do Discord. No Diracord ele é um **objeto**: você passa `{ ... }` para o `reply`. Não é uma peça do Components v2, e não se mistura com elas na mesma mensagem.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!profile"}, {"from": "bot", "embed": {"title": "Profile of Ana", "description": "Member since 2024-03-10T18:22:05+00:00", "color": "blurple", "fields": [{"name": "Coins", "value": "120", "inline": true}, {"name": "Level", "value": "7", "inline": true}], "footer": "Diracord", "thumbnail": "x"}}]}
```

```dcs
command("profile") {
  reply {
    title: "Profile of $user.name"
    description: "Member since $user.joinedAt"
    color: "blurple"
    thumbnail: user.avatar
    fields: [
      { name: "Coins", value: get("coins", 0, scope: user), inline: true },
      { name: "Level", value: get("level", 1, scope: user), inline: true }
    ]
    footer: "Diracord"
  }
}
```

## Campos

| campo | o que é |
|---|---|
| `title` | título em destaque |
| `description` | texto principal (aceita markdown) |
| `url` | link do título |
| `color` | `"red"`, `"blurple"`, `"#ff8800"` ou um número |
| `fields` | lista de `{ name, value, inline }` |
| `thumbnail` | imagem pequena à direita (url) |
| `image` | imagem grande embaixo (url) |
| `author` | texto pequeno acima do título |
| `footer` | texto pequeno no fim |
| `timestamp` | `true` para a hora atual |

Cores com nome: `red`, `green`, `blue`, `blurple`, `yellow`, `orange`, `purple`, `pink`, `white`, `black`, `gray`.

## Aviso simples

```preview
{"messages": [{"from": "user", "text": "!warn-me"}, {"from": "bot", "embed": {"title": "Careful!", "description": "This action cannot be undone.", "color": "red"}}]}
```

```dcs
command("warn-me") {
  reply { title: "Careful!", description: "This action cannot be undone.", color: "red" }
}
```

## Embed ou Components v2?

- **Embed**: rápido para um cartão com título, campos e cor.
- **Components v2**: quando você quer botões e menus dentro da caixa, várias seções ou galeria. Comece pela [visão geral](/components/overview).
