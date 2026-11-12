O Components v2 do Discord monta mensagens com caixas, textos, imagens, botões e menus. No Diracord, você monta essas mensagens com **peças**, no estilo do `ContainerBuilder` do discord.js: cada peça é uma função, e uma vai dentro da outra.

## O primeiro exemplo

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

Lendo de fora para dentro: uma **caixa roxa** (`container`) com um **título** (`text`), um texto com a **foto** ao lado (`section`), uma **linha** (`separator`) e uma **fileira de botões** (`row`). O clique de cada botão roda o `button("id") { }` com o mesmo id.

## As peças

| peça | vai onde | o que recebe |
|---|---|---|
| `container(peças..., color?, spoiler?)` | na mensagem | text, section, row, separator, gallery, file |
| `text(conteúdo)` | mensagem ou container | markdown (`## título`, `**negrito**`) |
| `section(textos..., thumbnail: ou button:)` | mensagem ou container | 1 a 3 textos + **um** acessório |
| `row(peças...)` | mensagem ou container | até 5 botões, **ou** um menu |
| `button(id, label, style?, emoji?, disabled?, url?)` | `row` ou `section(button: ...)` | style: primary, secondary, success, danger |
| `menu(id, opções, placeholder?, min?, max?)` | `row` | lista de textos ou `{ label, value }` |
| `thumbnail(url, description?, spoiler?)` | `section(thumbnail: ...)` | ou passe só a url |
| `separator(spacing?, line?)` | mensagem ou container | spacing: "small" ou "large" |
| `gallery(urls...)` | mensagem ou container | 1 a 10 imagens |
| `file(url, spoiler?)` | mensagem ou container | "attachment://nome" |

## O que mandar no reply

`reply` (e `send`) aceitam:

- **uma peça**: `reply container(...)`, `reply text("hi")`, `reply row(button(...))`;
- **uma lista de peças**: `reply [text("Pick one:"), row(...)]`;
- o **objeto** `{ layout: [...] }` (útil para montar por código).

O layout não se mistura com texto solto nem com embed na mesma mensagem: para texto, use `text(...)`.

## Peças são valores

Dá para guardar peças em variáveis e montar com `if` e `for`:

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

## Botões de link

Com `url:`, o botão abre um endereço e não tem id (não precisa de `button("id") { }`):

```dcs
command("links") {
  reply row(
    button(label: "Website", url: "https://example.com"),
    button(label: "Support", url: "https://discord.gg/example", emoji: "💬")
  )
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

Erros comuns:

- botão solto no container → coloque dentro de `row(...)`;
- `section("texto")` sem acessório → adicione `thumbnail: url` ou `button: button(...)`;
- texto solto no container (`container("hi")`) → use `container(text("hi"))`;
- mais de 5 botões numa `row` → divida em duas `row`.

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
