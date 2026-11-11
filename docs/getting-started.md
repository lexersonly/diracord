# Começando

## 1. Instalar

Por enquanto, a partir do repositório (o pacote npm `diracord` ainda não foi publicado):

```bash
git clone https://github.com/lexersonly/diracord && cd diracord
cargo build --release -p diracord-engine
npm install
```

O comando `diracord` é `node packages/cli/src/main.ts`. Para encurtar:

```bash
alias diracord="node $PWD/packages/cli/src/main.ts"
```

## 2. Criar um bot

```bash
diracord new meu-bot
cd meu-bot
cp .env.example .env
```

Abra o `.env` e coloque o token do bot (Discord Developer Portal → seu app → Bot → Reset Token):

```text
DISCORD_TOKEN=cole_aqui
```

> O `.env` já está no `.gitignore`. Nunca coloque o token no código.

No portal, em **Bot → Privileged Gateway Intents**, ligue **Message Content** e
**Server Members** (o projeto novo usa as duas).

## 3. Escrever código

O projeto novo já vem com `commands/ping.dcs`:

```dcs
command("ping") {
  reply "Pong!"
}
```

Crie outros arquivos `.dcs` em qualquer pasta. Todos são carregados juntos.

## 4. Conferir e ligar

```bash
diracord check        # mostra erros com o trecho e a dica
diracord check --fix  # corrige sozinho o que for seguro
diracord run          # liga o bot
diracord watch        # liga e recarrega a cada arquivo salvo
```

Sem token, `diracord watch` fica em **modo checagem**: confere o código a cada mudança.

## 5. Próximos passos

- [Guia da linguagem](language.md)
- [Lista de funções](functions.md)
- [Códigos de erro](errors.md)
