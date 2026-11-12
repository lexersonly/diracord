De 1 a 3 textos com **um acessório** ao lado: uma imagem pequena ou um botão.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!me"}, {"from": "bot", "layout": [{"container": [{"section": ["## Ana", "Level 7 · 120 coins"], "thumbnail": "x"}], "color": "blurple"}]}]}
```

```dcs
command("me") {
  local coins = get("coins", 0, scope: user)
  reply container(
    section("## $user.name", "Level 7 · $coins coins", thumbnail: user.avatar),
    color: "blurple"
  )
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| textos... | texto | de 1 a 3 textos (markdown) |
| `thumbnail` | url ou [`thumbnail(...)`](/components/thumbnail) | imagem ao lado |
| `button` | [`button(...)`](/components/button) | botão ao lado |

Toda section precisa de **exatamente um** acessório: `thumbnail:` **ou** `button:`.

## Onde pode ficar

Direto na mensagem ou dentro de [`container`](/components/container).

## Com botão ao lado

Bom para listas onde cada linha tem uma ação:

```preview
{"messages": [{"from": "user", "text": "!shop"}, {"from": "bot", "layout": [{"container": [{"text": "## Shop"}, {"section": ["**VIP role**", "500 coins"], "button": {"button": "buy-vip", "label": "Buy", "style": "success"}}, {"section": ["**Custom color**", "200 coins"], "button": {"button": "buy-color", "label": "Buy", "style": "success"}}], "color": "green"}]}]}
```

```dcs
command("shop") {
  reply container(
    text("## Shop"),
    section("**VIP role**", "500 coins", button: button("buy-vip", "Buy", style: "success")),
    section("**Custom color**", "200 coins", button: button("buy-color", "Buy", style: "success")),
    color: "green"
  )
}

button("buy-vip") {
  reply "VIP bought!" ephemeral: true
}

button("buy-color") {
  reply "Color bought!" ephemeral: true
}
```

## Erros comuns

- **Sem acessório:** `section("hi")` dá erro. Se você só quer texto, use [`text("hi")`](/components/text).
- **Dois acessórios:** `thumbnail:` e `button:` juntos não pode. Escolha um.
- **Mais de 3 textos:** divida em duas sections.
- **Botão como texto:** `section(button("a", "A"))` → o botão vai em `button:`: `section("text", button: button("a", "A"))`.
