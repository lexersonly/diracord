A caixa que agrupa outras peças, com uma barra de cor do lado. É a peça que mais lembra um embed, mas aceita botões e menus dentro.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!news"}, {"from": "bot", "layout": [{"container": [{"text": "## Server news"}, {"text": "The event starts **tomorrow** at 8 PM."}], "color": "blurple"}]}]}
```

```dcs
command("news") {
  reply container(
    text("## Server news"),
    text("The event starts **tomorrow** at 8 PM."),
    color: "blurple"
  )
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| peças... | peças | uma ou mais: `text`, `section`, `row`, `separator`, `gallery`, `file` |
| `color` | texto (opcional) | cor da barra: `"purple"`, `"#ff8800"`... |
| `spoiler` | bool (opcional) | esconde a caixa até clicar |

As peças vêm primeiro, sem nome; `color` e `spoiler` vão no fim, pelo nome.

## Onde pode ficar

Direto na mensagem: `reply container(...)` ou dentro de uma lista `reply [container(...), ...]`. Um container **não** vai dentro de outro.

## Com botões

```preview
{"messages": [{"from": "user", "text": "!ticket"}, {"from": "bot", "layout": [{"container": [{"text": "## Need help?"}, {"text": "Open a ticket and the staff will answer you."}, {"row": [{"button": "open-ticket", "label": "Open ticket", "emoji": "🎫"}, {"button": null, "label": "FAQ", "url": "x"}]}], "color": "green"}]}]}
```

```dcs
command("ticket") {
  reply container(
    text("## Need help?"),
    text("Open a ticket and the staff will answer you."),
    row(
      button("open-ticket", "Open ticket", emoji: "🎫"),
      button(label: "FAQ", url: "https://example.com/faq")
    ),
    color: "green"
  )
}

button("open-ticket") {
  reply "Ticket opened! The staff was notified." ephemeral: true
}
```

## Duas caixas na mesma mensagem

```preview
{"messages": [{"from": "user", "text": "!plans"}, {"from": "bot", "layout": [{"container": [{"text": "## Free"}, {"text": "Basic commands."}], "color": "gray"}, {"container": [{"text": "## VIP"}, {"text": "Everything, plus custom colors."}], "color": "yellow"}]}]}
```

```dcs
command("plans") {
  reply [
    container(text("## Free"), text("Basic commands."), color: "gray"),
    container(text("## VIP"), text("Everything, plus custom colors."), color: "yellow")
  ]
}
```

## Erros comuns

- **Texto solto:** `container("hi")` → use `container(text("hi"))`.
- **Botão solto:** `container(button("a", "A"))` → coloque em `row(...)`: `container(row(button("a", "A")))`.
- **Container dentro de container** não existe no Discord: use uma lista com dois containers.
- **Vazio:** `container()` precisa de pelo menos uma peça.
