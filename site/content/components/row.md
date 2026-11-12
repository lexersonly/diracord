Uma linha horizontal com até **5 botões**, ou com **um menu**. Botões e menus só aparecem dentro de uma row (ou, no caso do botão, como acessório de uma section).

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!vote"}, {"from": "bot", "layout": [{"text": "Do you like pizza?"}, {"row": [{"button": "yes", "label": "Yes", "style": "success"}, {"button": "no", "label": "No", "style": "danger"}, {"button": "maybe", "label": "Maybe", "style": "secondary"}]}]}]}
```

```dcs
command("vote") {
  reply [
    text("Do you like pizza?"),
    row(
      button("yes", "Yes", style: "success"),
      button("no", "No", style: "danger"),
      button("maybe", "Maybe", style: "secondary")
    )
  ]
}

button("yes") { reply "Great taste!" ephemeral: true }
button("no") { reply "More for us." ephemeral: true }
button("maybe") { reply "Take your time." ephemeral: true }
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| peças... | [`button`](/components/button) ou [`menu`](/components/menu) | até 5 botões, **ou** um menu sozinho |

## Onde pode ficar

Direto na mensagem ou dentro de [`container`](/components/container).

## Com um menu

```preview
{"messages": [{"from": "user", "text": "!language"}, {"from": "bot", "layout": [{"text": "Choose your language:"}, {"row": [{"menu": "lang", "options": ["English", "Português", "Español"], "placeholder": "Language"}]}]}]}
```

```dcs
command("language") {
  reply [
    text("Choose your language:"),
    row(menu("lang", ["English", "Português", "Español"], placeholder: "Language"))
  ]
}

menu("lang") {
  set("lang", message.values[0], scope: user)
  reply "Language saved." ephemeral: true
}
```

## Botões criados com for

Uma row recebe as peças uma por uma, então para criar botões a partir de uma lista monte cada row à mão ou use o [objeto de layout](/components/overview#a-forma-objeto):

```preview
{"messages": [{"from": "user", "text": "!pick"}, {"from": "bot", "layout": [{"row": [{"button": "pick-1", "label": "1"}, {"button": "pick-2", "label": "2"}, {"button": "pick-3", "label": "3"}]}]}]}
```

```dcs
command("pick") {
  local buttons = []
  for n in range(1, 4) {
    buttons = push(buttons, { button: "pick-" + n, label: toText(n) })
  }
  reply { layout: [{ row: buttons }] }
}
```

## Erros comuns

- **Mais de 5 botões:** divida em duas rows.
- **Menu com botões na mesma row:** o menu ocupa a row inteira. Use uma row para o menu e outra para os botões.
- **Texto dentro da row:** `row(text("a"))` não pode; a row só aceita botões e menus.
