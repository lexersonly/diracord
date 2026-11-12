Mostra, dentro do layout, um arquivo que foi enviado junto com a mensagem.

> **Ainda sem uso prático.** A peça existe e é validada, mas o Diracord 0.1 ainda não envia anexos, então hoje não há um arquivo para ela mostrar. Ela está aqui para a lista de peças ficar completa.

## Forma

```dcs
command("report") {
  reply container(
    text("## Monthly report"),
    file("attachment://report.pdf")
  )
}
```

```preview
{"messages": [{"from": "user", "text": "!report"}, {"from": "bot", "layout": [{"container": [{"text": "## Monthly report"}, {"file": "attachment://report.pdf"}]}]}]}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| `url` | texto | sempre `"attachment://nome-do-arquivo"` |
| `spoiler` | bool (opcional) | escondido até clicar |

## Onde pode ficar

Direto na mensagem ou dentro de [`container`](/components/container).

## Por enquanto

Para compartilhar um arquivo hoje, use um [botão de link](/components/button#botao-de-link) apontando para onde ele está hospedado.
