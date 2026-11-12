Um respiro entre peças: uma linha fina, um espaço, ou os dois.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!info"}, {"from": "bot", "layout": [{"container": [{"text": "## Rules"}, {"text": "Be nice to everyone."}, {"separator": true}, {"text": "## Events"}, {"text": "Game night every Friday."}], "color": "blue"}]}]}
```

```dcs
command("info") {
  reply container(
    text("## Rules"),
    text("Be nice to everyone."),
    separator(),
    text("## Events"),
    text("Game night every Friday."),
    color: "blue"
  )
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| `spacing` | texto (opcional) | `"small"` (padrão) ou `"large"` |
| `line` | bool (opcional) | `false` esconde a linha e deixa só o espaço |

Sem argumentos, escreva `separator()`.

## Espaço grande, sem linha

```preview
{"messages": [{"from": "user", "text": "!spaced"}, {"from": "bot", "layout": [{"container": [{"text": "First block"}, {"separator": true, "spacing": "large", "line": false}, {"text": "Second block, further away"}]}]}]}
```

```dcs
command("spaced") {
  reply container(
    text("First block"),
    separator(spacing: "large", line: false),
    text("Second block, further away")
  )
}
```

## Onde pode ficar

Direto na mensagem ou dentro de [`container`](/components/container).

## Erros comuns

- **`spacing` errado:** só `"small"` e `"large"` existem.
- **Dentro de row ou section:** não pode; o separador fica entre peças do container ou da mensagem.
