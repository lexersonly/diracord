Tudo que muda o comportamento do bot fica no `diracord.toml`. O token fica no `.env`.

## diracord.toml

Todas as chaves são opcionais; abaixo estão os valores padrão.

```toml
ram = "100mb"            # meta de memória (informativo por enquanto)

[bot]
name = "my-bot"
prefix = "!"

[commands]
prefix = true            # !ping
slash = true             # /ping

[syntax]
dollar = false           # liga o atalho $ban[user;reason]

[data]
path = "data/bot.dirabank"
default_scope = "user"   # user | server | member | global

[limits]
time_ms = 3000           # tempo de CPU por execução
steps = 100000           # passos por execução
memory_kb = 4096         # memória das variáveis
actions = 50             # ações no Discord por execução

[discord]
intents = ["guilds", "guildMessages", "messageContent", "guildMembers"]
```

Intents disponíveis: `guilds`, `guildMembers`, `guildMessages`, `guildMessageReactions`, `guildVoiceStates`, `messageContent`, `directMessages`. As que dizem *Privileged* no portal do Discord (Message Content e Server Members) precisam ser ligadas lá também.

## .env

```text
DISCORD_TOKEN=your_token_here
```

O `.env` nunca vai para o git (o `.gitignore` do projeto novo já cuida disso). O terminal mostra o token cortado (`abcd…wxyz`), nunca inteiro.

## Arquivos de código

Qualquer arquivo `.dcs`, `.dira` ou `.diracord` dentro da pasta do projeto é carregado, em qualquer subpasta. Pastas que começam com `.`, `node_modules` e a pasta dos dados são ignoradas. Não existe import: comandos, eventos e funções valem no projeto todo.

## Comandos do terminal

| comando | o que faz |
|---|---|
| `diracord new <pasta>` | cria um projeto pronto |
| `diracord check` | mostra todos os erros e avisos, com o trecho e a dica |
| `diracord check --fix` | aplica as correções automáticas seguras e mostra o que sobrou |
| `diracord run` | liga o bot |
| `diracord watch` | liga e recarrega a cada arquivo salvo; sem token, só confere o código |
| `diracord help` | ajuda |

Todos aceitam `--dir <pasta>` para rodar em outra pasta. Com `npx`, escreva `npx diracord check`.

## Códigos de saída

`diracord check` termina com código `0` sem erros e `1` com erros, então dá para usar em CI:

```bash
npx diracord check || exit 1
```
