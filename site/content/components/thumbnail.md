A imagem pequena que fica ao lado de uma [`section`](/components/section). Na maioria das vezes você só passa a url; a função `thumbnail(...)` existe para quando você quer descrição ou spoiler.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!avatar"}, {"from": "bot", "layout": [{"section": ["## Ana", "This is your avatar."], "thumbnail": "x"}]}]}
```

```dcs
command("avatar") {
  reply section("## $user.name", "This is your avatar.", thumbnail: user.avatar)
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| `url` | texto | endereço da imagem (`https://...`) |
| `description` | texto (opcional) | texto alternativo, para leitores de tela |
| `spoiler` | bool (opcional) | borrada até clicar |

## Com descrição e spoiler

```dcs
command("spoiler") {
  reply section(
    "## Surprise",
    "Click the image to reveal it.",
    thumbnail: thumbnail("https://example.com/secret.png", description: "a secret image", spoiler: true)
  )
}
```

## Onde pode ficar

Só no `thumbnail:` de uma section. Para imagens grandes, use [`gallery`](/components/gallery).

## Erros comuns

- **Thumbnail solta:** `reply thumbnail(url)` ou `container(thumbnail(url))` → ela vai em `section(..., thumbnail: url)`.
- **Endereço sem https:** a url precisa começar com `https://`.
- **`user.avatar` vazio:** quem não tem foto tem `avatar` igual a `null`. Confira com `if user.avatar { }` antes, ou use uma imagem padrão.
