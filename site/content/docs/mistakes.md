Os erros mais comuns de quem está começando, e como evitar cada um. Os marcados com **--fix** o `diracord check --fix` corrige sozinho.

## Chamada dentro de chamada sem parênteses

```text
reply upper "hi"          // reply recebe upper e "hi" separados
```

```dcs
command("t") {
  reply upper("hi")
}
```

Sem parênteses, cada argumento é separado por espaço. Uma chamada dentro de outra precisa de parênteses.

## Chamada sem parênteses no if

```text
if hasRole user "VIP" { }
```

```dcs
command("vip") {
  if hasRole(user, "VIP") {
    reply "Hi, VIP!"
  }
}
```

Em `if`, `while`, `for` e `chain`, use parênteses nas chamadas.

## = no lugar de == (--fix)

```text
if x = 5 { }              // E0100: para comparar, use ==
```

`=` guarda um valor; `==` compara.

## let, var, const, try, function (--fix)

O Diracord usa outras palavras: `local` para criar variável, `testing` para tratar erro e `fn` para criar função. O `--fix` troca sozinho.

## Variável que não existe (E0200)

```text
x = 1                     // E0200: x não foi criada (--fix coloca local)
reply nmae                // E0200: você quis dizer name?
```

Crie com `local` antes de mudar. Dentro de `fn`, lembre que a função não enxerga os `local` de quem chamou: passe como argumento.

## guild não existe (--fix)

```text
reply guild.name          // E0200: no Diracord o servidor se chama server
```

## Aspas e $ no texto

```text
reply 'hi'                // E0002: texto usa aspas duplas
reply "costs $5"          // E0005: $ precisa de um nome depois (--fix escreve \$)
reply "total: ${a + b}"   // ${} não existe
```

```dcs
command("price") {
  local a = 2
  local b = 3
  local total = a + b
  reply "Total: $total. It costs \$5."
}
```

## Comando de moderação sem checar permissão

Esse não dá erro, e por isso é perigoso: sem checagem, **qualquer pessoa** usa `!ban`.

```dcs
command("ban", target: member) {
  if not hasRole(user, "Moderator") {
    reply "Only moderators can ban." ephemeral: true
    stop
  }
  ban target
}
```

Lembre também de dar ao **bot** a permissão de banir, e de deixar o cargo do bot acima do cargo de quem ele vai punir.

## Esquecer de tratar ações que falham (E0208)

Banir alguém com cargo maior que o do bot, ou mandar DM para quem bloqueou o bot, falha. Sem tratamento, o comando para com o erro E0208. Com `testing`, você decide o que responder:

```dcs
command("dm-me") {
  testing {
    dm user "Hello!"
  } catch err {
    reply "I can't send you a DM: $err"
  }
}
```

## Laços sem fim e spam

- `while` que nunca termina → para no limite de passos (E0205). Confira se a condição muda dentro do laço.
- `for` que manda mensagem a cada volta → para em 50 ações (E0205). Junte o texto e mande uma vez só com `join(...)`.

## chain com _ no meio

```text
chain x { _ => stop  1 => reply "one" }     // E0110: _ precisa ser o último
```

## Components v2 no lugar errado (E0117)

```text
reply container(button("a", "A"))           // botão precisa de row(...)
reply section("hi")                         // section precisa de thumbnail: ou button:
reply container("hi")                       // texto precisa de text("hi")
```

```dcs
command("ok") {
  reply container(text("hi"), row(button("a", "A")))
}
```

## !comandos não respondem

Quase sempre é a intent: ligue **Message Content Intent** no portal do Discord (aba Bot) e deixe `messageContent` em `[discord] intents`. Confira também o `prefix` no `diracord.toml`.

## /comando não aparece

- O nome do slash precisa ser minúsculo, sem espaço: `user-info`, não `UserInfo` (o terminal avisa quando pula um).
- O bot precisa ter sido convidado com o escopo `applications.commands`.
- O Discord pode levar alguns segundos para mostrar comandos novos; reabra o app.

## Token no código

Nunca coloque o token em um `.dcs` ou no `diracord.toml`. Ele fica no `.env`, que não vai para o git. Se o token vazou, gere outro no portal do Discord (**Reset Token**) na hora.
