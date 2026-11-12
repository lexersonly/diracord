O Components v2 do Discord monta mensagens com caixas, textos, imagens, botões e menus. No Diracord, você monta essas mensagens com **peças**, no estilo do `ContainerBuilder` do discord.js: cada peça é uma função, e uma vai dentro da outra.

## O primeiro exemplo

```preview
{"messages": [{"from": "user", "text": "!rules"}, {"from": "bot", "layout": [{"container": [{"text": "## Welcome, Ana"}, {"section": ["Read the rules before you continue."], "thumbnail": "x"}, {"separator": true, "spacing": "large"}, {"row": [{"button": "accept", "label": "Accept", "style": "success"}, {"button": "decline", "label": "Decline", "style": "danger"}]}], "color": "purple"}]}]}
```

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

Lendo de fora para dentro: uma **caixa roxa** ([`container`](/components/container)) com um **título** ([`text`](/components/text)), um texto com a **foto** ao lado ([`section`](/components/section)), uma **linha** ([`separator`](/components/separator)) e uma **fileira de botões** ([`row`](/components/row) com [`button`](/components/button)). O clique de cada botão roda o `button("id") { }` com o mesmo id.

## Todas as peças

| peça | para quê | vai onde |
|---|---|---|
| [`container`](/components/container) | caixa com cor que agrupa peças | na mensagem |
| [`section`](/components/section) | texto + imagem pequena ou botão ao lado | mensagem ou container |
| [`row`](/components/row) | linha de botões, ou um menu | mensagem ou container |
| [`separator`](/components/separator) | espaço ou linha | mensagem ou container |
| [`text`](/components/text) | texto com markdown | mensagem ou container |
| [`thumbnail`](/components/thumbnail) | imagem pequena | `section(thumbnail: ...)` |
| [`gallery`](/components/gallery) | 1 a 10 imagens grandes | mensagem ou container |
| [`file`](/components/file) | arquivo anexado | mensagem ou container |
| [`button`](/components/button) | botão de ação ou de link | `row` ou `section(button: ...)` |
| [`menu`](/components/menu) | menu de opções | `row` |

O cartão clássico (título, descrição, campos) continua existindo: veja [Embed](/components/embed).

## O que mandar no reply

`reply` (e `send`) aceitam:

- **uma peça**: `reply container(...)`, `reply text("hi")`, `reply row(button(...))`;
- **uma lista de peças**: `reply [text("Pick one:"), row(...)]`;
- o **objeto** `{ layout: [...] }` (útil para montar por código).

O layout não se mistura com texto solto nem com embed na mesma mensagem: para texto, use `text(...)`.

## Peças são valores

Dá para guardar peças em variáveis e montar com `if` e `for`:

```preview
{"messages": [{"from": "user", "text": "!colors"}, {"from": "bot", "layout": [{"text": "Pick a color:"}, {"row": [{"menu": "color", "options": ["Blue", "Green", "Purple"], "placeholder": "Click here"}]}, {"text": "-# there are 3 options"}]}]}
```

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

## Quando algo está no lugar errado

A seta aponta para a peça errada e a ajuda diz onde ela vai:

```text
erro[E0117]: `button(...)` não pode ficar dentro de `container(...)`
 --> commands/rules.dcs:2:31
  |
2 |   reply container(text("a"), button("b", "B"))
  |                              ^^^^^^^^^^^^^^^^^ aqui
  |
  = ajuda: `button` vai dentro de `row(...)` (ou como `button:` de uma `section(...)`)
```

## A forma objeto

Por baixo, toda mensagem vira este objeto. Ele também é aceito direto no `reply`:

```dcs
command("hi") {
  reply {
    layout: [
      { container: [
          { text: "## Hi" },
          { row: [{ button: "accept", label: "Accept", style: "success" }] }
        ], color: "purple" }
    ]
  }
}
```
