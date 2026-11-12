Um bloco de texto dentro do layout. Aceita o markdown do Discord.

## Exemplo

```preview
{"messages": [{"from": "user", "text": "!about"}, {"from": "bot", "layout": [{"text": "## About this bot"}, {"text": "Made with **Diracord**. Use `!help` to see the commands."}, {"text": "-# version 1.0"}]}]}
```

```dcs
command("about") {
  reply [
    text("## About this bot"),
    text("Made with **Diracord**. Use `!help` to see the commands."),
    text("-# version 1.0")
  ]
}
```

## Argumentos

| argumento | tipo | o que é |
|---|---|---|
| `content` | texto | até 4000 letras; não pode ser vazio |

## Markdown que funciona

| você escreve | aparece como |
|---|---|
| `# Title`, `## Title`, `### Title` | títulos (grande, médio, pequeno) |
| `**bold**`, `*italic*`, `__underline__` | negrito, itálico, sublinhado |
| `` `code` `` | código |
| `-# small` | texto pequeno e apagado |
| `> quote` | citação |
| `$user.mention` | menção (só usuários são mencionados de verdade) |

## Onde pode ficar

Direto na mensagem ou dentro de [`container`](/components/container). Dentro de [`section`](/components/section), passe o texto direto: `section("my text", ...)`.

## Com variáveis

```preview
{"messages": [{"from": "user", "text": "!balance"}, {"from": "bot", "layout": [{"container": [{"text": "## Ana's wallet"}, {"text": "You have **120** coins."}], "color": "yellow"}]}]}
```

```dcs
command("balance") {
  local coins = get("coins", 0, scope: user)
  reply container(
    text("## $user.name's wallet"),
    text("You have **$coins** coins."),
    color: "yellow"
  )
}
```

## Erros comuns

- **Misturar com texto solto:** `reply "Hi", row(...)` não funciona. Em mensagens com peças, todo texto é um `text(...)`.
- **Contas dentro do texto:** `"${a + b}"` não existe. Crie uma variável antes (`local total = a + b`) e use `"$total"`.
