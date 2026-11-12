# Diracord

Uma linguagem de texto para criar bots de Discord: mais fácil que aoi.js, com
variáveis, `if`, `for` e funções de verdade, e erros que dizem exatamente o que corrigir.

```dcs
command("ban", user: member, reason: text = "no reason") {
  testing {
    ban user reason
    reply "$user.name was banned. Reason: $reason"
  } catch err {
    reply "Could not ban: $err"
  }
}
```

O mesmo arquivo atende `!ban @Joe spam` e `/ban`.

## Começar

Requer Node.js 22.18 ou mais novo.

```bash
npx diracord new my-bot
cd my-bot
cp .env.example .env   # coloque o token do bot
npx diracord run
```

| comando | o que faz |
|---|---|
| `diracord new <pasta>` | cria um projeto |
| `diracord check [--fix]` | mostra os erros com seta e sugestão; `--fix` corrige os seguros |
| `diracord run` | liga o bot |
| `diracord watch` | liga e recarrega a cada arquivo salvo |

O pacote já vem com o motor em Rust compilado para Linux (x64, arm64), macOS (Intel e
Apple Silicon) e Windows (x64).

Documentação, guias para quem vem do aoi.js e do discord.js, e todos os códigos de erro:
https://github.com/lexersonly/diracord

Licença MIT.
