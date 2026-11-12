De 1 a 10 imagens grandes, em grade.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!photos"}, {"from": "bot", "layout": [{"text": "## Event photos"}, {"gallery": [1, 2, 3, 4]}]}]}
```

```dcs
command("photos") {
  reply [
    text("## Event photos"),
    gallery(
      "https://example.com/1.png",
      "https://example.com/2.png",
      "https://example.com/3.png",
      "https://example.com/4.png"
    )
  ]
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| imagens... | url, ou `{ url, description, spoiler }` | de 1 a 10 imagens |

Também aceita uma lista pronta: `gallery(urls)`.

## Uma imagem só

```preview
{"messages": [{"from": "user", "text": "!banner"}, {"from": "bot", "layout": [{"container": [{"text": "## Welcome!"}, {"gallery": [1]}], "color": "pink"}]}]}
```

```dcs
command("banner") {
  reply container(
    text("## Welcome!"),
    gallery("https://example.com/banner.png"),
    color: "pink"
  )
}
```

## A partir de uma lista salva

```dcs
command("album") {
  local urls = get("album", [], scope: server)
  if len(urls) == 0 {
    reply "The album is empty."
    stop
  }
  reply gallery(urls)
}
```

## Onde pode ficar

Direto na mensagem ou dentro de [`container`](/components/container).

## Erros comuns

- **Mais de 10 imagens:** divida em duas galerias.
- **Lista vazia:** `gallery([])` dá erro; confira com `len(...)` antes, como no exemplo acima.
- **Endereço sem https:** cada url precisa começar com `https://`.
