Um menu de opções. Quando a pessoa escolhe, roda o bloco `menu("id") { }` com o mesmo id, e as opções escolhidas ficam em `message.values`.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!color"}, {"from": "bot", "layout": [{"text": "Pick your favorite color:"}, {"row": [{"menu": "color", "options": ["Red", "Green", "Blue"], "placeholder": "Choose a color"}]}]}, {"from": "bot", "text": "You picked Blue.", "ephemeral": true}]}
```

```dcs
command("color") {
  reply [
    text("Pick your favorite color:"),
    row(menu("color", ["Red", "Green", "Blue"], placeholder: "Choose a color"))
  ]
}

menu("color") {
  reply "You picked " + message.values[0] + "." ephemeral: true
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| `id` | texto | liga ao `menu("id") { }` |
| `options` | lista | de 1 a 25 textos, ou objetos `{ label, value, description }` |
| `placeholder` | texto (opcional) | o que aparece antes de escolher |
| `min` | número (opcional) | mínimo de escolhas |
| `max` | número (opcional) | máximo de escolhas |

## Onde pode ficar

Dentro de [`row`](/components/row), **sozinho**: uma row com menu não aceita mais nada.

## Opções com valor e descrição

O `label` é o que a pessoa vê; o `value` é o que chega em `message.values`:

```preview
{"messages": [{"from": "user", "text": "!role"}, {"from": "bot", "layout": [{"row": [{"menu": "pick-role", "options": [{"label": "Gamer"}], "placeholder": "Pick a role"}]}]}]}
```

```dcs
command("role") {
  reply row(menu("pick-role", [
    { label: "Gamer", value: "Games", description: "Get pinged for game nights" },
    { label: "Artist", value: "Art", description: "Access to the art channels" }
  ], placeholder: "Pick a role"))
}

menu("pick-role") {
  addRole user message.values[0]
  reply "You got the " + message.values[0] + " role." ephemeral: true
}
```

## Várias escolhas

```preview
{"messages": [{"from": "user", "text": "!topics"}, {"from": "bot", "layout": [{"row": [{"menu": "topics", "options": ["News"], "placeholder": "Pick up to 3 topics"}]}]}, {"from": "bot", "text": "Saved: News, Events", "ephemeral": true}]}
```

```dcs
command("topics") {
  reply row(menu("topics", ["News", "Events", "Memes", "Art"], placeholder: "Pick up to 3 topics", min: 1, max: 3))
}

menu("topics") {
  set("topics", message.values, scope: user)
  reply "Saved: " + join(message.values) ephemeral: true
}
```

## Erros comuns

- **Menu solto:** `reply menu(...)` → coloque numa row: `reply row(menu(...))`.
- **Menu e botões na mesma row:** use rows separadas.
- **Mais de 25 opções:** o Discord não aceita. Divida em dois menus.
- **`message.values` é uma lista**, mesmo com uma escolha só: use `message.values[0]` ou `join(message.values)`.
