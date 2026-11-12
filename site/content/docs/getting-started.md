Em 10 minutos você tem um bot respondendo `!ping` e `/ping`. Você só precisa do Node.js e de uma conta no Discord.

## O que você precisa

- **Node.js 22.18 ou mais novo** ([nodejs.org](https://nodejs.org)). Para conferir: `node --version`.
- **Um bot criado no Discord.** O passo a passo está logo abaixo.

## Criar o bot no Discord

1. Abra o [Discord Developer Portal](https://discord.com/developers/applications) e clique em **New Application**.
2. Na aba **Bot**, clique em **Reset Token** e copie o token. Ele é a senha do seu bot: não mostre para ninguém.
3. Ainda em **Bot**, em **Privileged Gateway Intents**, ligue **Message Content Intent** (para comandos com `!`) e **Server Members Intent** (para `memberJoin`).
4. Na aba **OAuth2 → URL Generator**, marque `bot` e `applications.commands`, escolha as permissões que o bot vai usar (por exemplo *Send Messages*, *Ban Members*) e abra o link gerado para adicionar o bot ao seu servidor.

## Criar o projeto

```bash
npx diracord new my-bot
cd my-bot
```

Isso cria uma pasta pronta:

```text
my-bot/
  diracord.toml        configuração (prefixo, modos, limites)
  .env.example         modelo do arquivo com o token
  .gitignore           já ignora o .env e a pasta data/
  commands/ping.dcs    !ping e /ping
  commands/coins.dcs   moedas salvas por usuário (usa fn)
  commands/rules.dcs   mensagem com botão (Components v2)
  events/join.dcs      boas-vindas
```

## Colocar o token

Copie o modelo e cole o token depois do `=`:

```bash
cp .env.example .env
```

```text
DISCORD_TOKEN=cole_o_token_aqui
```

> O token fica **só** no `.env`. Nunca coloque o token dentro de um arquivo `.dcs` nem envie o `.env` para o GitHub.

## Conferir e ligar

```bash
npx diracord check   # procura erros no projeto inteiro
npx diracord run     # liga o bot
```

No Discord, mande `!ping` em um canal. O bot responde `Pong!`. O `/ping` aparece depois que o bot registra os slash commands (o terminal avisa).

Enquanto você escreve, use o modo que recarrega sozinho a cada arquivo salvo:

```bash
npx diracord watch
```

## Seu primeiro comando

Crie `commands/hello.dcs`:

```dcs
command("hello", name: text = "friend") {
  reply "Hi, $name!"
}
```

- `!hello` responde `Hi, friend!`
- `!hello Joe` responde `Hi, Joe!`
- `/hello name: Joe` também.

O `name: text = "friend"` declara um argumento do tipo texto, com valor padrão. Sem o `= ...`, o argumento é obrigatório.

## Deixar online 24 horas

O bot fica online enquanto `npx diracord run` estiver rodando. Para deixar ligado, use qualquer hospedagem que rode Node.js (uma VPS, um serviço de bots, um Raspberry Pi). O motor do Diracord usa poucos MB de memória, então cabe em planos pequenos.

## Próximos passos

- [Dicas e comandos básicos](/docs/tips): os comandos que todo bot tem, prontos para copiar.
- [Sintaxe](/docs/syntax): tudo que a linguagem tem.
- [Bugs e erros comuns](/docs/mistakes): o que costuma dar errado.
