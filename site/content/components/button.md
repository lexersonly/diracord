Um botão. Clicar nele roda o bloco `button("id") { }` que tem o mesmo id, escrito no topo de qualquer arquivo. Com `url:`, vira um botão de link.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!confirm"}, {"from": "bot", "layout": [{"text": "Delete all your data?"}, {"row": [{"button": "confirm-delete", "label": "Yes, delete", "style": "danger"}, {"button": "cancel", "label": "Cancel", "style": "secondary"}]}]}]}
```

```dcs
command("confirm") {
  reply [
    text("Delete all your data?"),
    row(
      button("confirm-delete", "Yes, delete", style: "danger"),
      button("cancel", "Cancel", style: "secondary")
    )
  ]
}

button("confirm-delete") {
  delete("coins", scope: user)
  reply "Done. Your data was deleted." ephemeral: true
}

button("cancel") {
  reply "Nothing was changed." ephemeral: true
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| `id` | texto | liga ao `button("id") { }`. Não use junto com `url` |
| `label` | texto | o texto do botão (até 80 letras) |
| `style` | texto (opcional) | `primary` (padrão), `secondary`, `success`, `danger` |
| `emoji` | texto (opcional) | um emoji antes do texto |
| `disabled` | bool (opcional) | botão apagado, que não clica |
| `url` | texto (opcional) | vira botão de link |

Um botão precisa de `label` ou de `emoji` (pode ter os dois).

## Estilos

```preview
{"messages": [{"from": "user", "text": "!styles"}, {"from": "bot", "layout": [{"row": [{"button": "a", "label": "primary"}, {"button": "b", "label": "secondary", "style": "secondary"}, {"button": "c", "label": "success", "style": "success"}, {"button": "d", "label": "danger", "style": "danger"}, {"button": null, "label": "link", "url": "x"}]}]}]}
```

```dcs
command("styles") {
  reply row(
    button("a", "primary"),
    button("b", "secondary", style: "secondary"),
    button("c", "success", style: "success"),
    button("d", "danger", style: "danger"),
    button(label: "link", url: "https://example.com")
  )
}
```

## Botão de link

Abre um endereço. Não tem id e não precisa de `button("id") { }`:

```preview
{"messages": [{"from": "user", "text": "!links"}, {"from": "bot", "layout": [{"row": [{"button": null, "label": "Website", "url": "x"}, {"button": null, "label": "Support", "url": "x", "emoji": "💬"}]}]}]}
```

```dcs
command("links") {
  reply row(
    button(label: "Website", url: "https://example.com"),
    button(label: "Support", url: "https://discord.gg/example", emoji: "💬")
  )
}
```

## Emoji e desligado

```preview
{"messages": [{"from": "user", "text": "!queue"}, {"from": "bot", "layout": [{"row": [{"button": "prev", "label": "", "emoji": "⬅️", "style": "secondary", "disabled": true}, {"button": "next", "label": "", "emoji": "➡️", "style": "secondary"}, {"button": "like", "label": "Like", "emoji": "❤️", "style": "success"}]}]}]}
```

```dcs
command("queue") {
  reply row(
    button("prev", emoji: "⬅️", style: "secondary", disabled: true),
    button("next", emoji: "➡️", style: "secondary"),
    button("like", "Like", emoji: "❤️", style: "success")
  )
}
```

## O que acontece no clique

Dentro de `button("id") { }`, `user` é **quem clicou** e `reply` responde ao clique. Com `ephemeral: true`, só quem clicou vê a resposta:

```preview
{"messages": [{"from": "bot", "layout": [{"row": [{"button": "accept", "label": "I agree", "style": "success"}]}]}, {"from": "bot", "text": "Thanks, Ana!", "ephemeral": true}]}
```

```dcs
command("rules") {
  reply row(button("accept", "I agree", style: "success"))
}

button("accept") {
  addRole user "Member"
  reply "Thanks, $user.name!" ephemeral: true
}
```

## Onde pode ficar

Dentro de [`row`](/components/row) (até 5 por row), ou como acessório de uma [`section`](/components/section): `section("text", button: button("id", "Label"))`.

## Erros comuns

- **Botão solto:** `reply button("a", "A")` → coloque numa row: `reply row(button("a", "A"))`.
- **Sem id e sem url:** todo botão precisa de um dos dois.
- **Id e url juntos:** botão de link não tem id.
- **Bloco no lugar errado:** o código do clique fica **fora** do comando, no topo do arquivo, não dentro do `row(...)`.
- **Estilo que não existe:** `style: "red"` → use `danger`. Os estilos são `primary`, `secondary`, `success` e `danger`.
- **Clicou e nada aconteceu:** confira se o id do `button("id") { }` é idêntico ao do botão (maiúsculas contam).
