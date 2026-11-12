Para não repetir código, crie suas próprias funções com `fn`, no topo de qualquer arquivo. Elas valem no projeto inteiro.

## Criando e chamando

```preview
{"messages": [{"from": "user", "text": "!daily"}, {"from": "bot", "text": "Hi Ana! You have 100 coins."}]}
```

```dcs
fn giveCoins(who: member, amount: number = 1) {
  local total = get("coins", 0, scope: user) + amount
  set("coins", total, scope: user)
  return total
}

fn greet(name) {
  return "Hi $name"
}

command("daily") {
  local total = giveCoins(user, 100)
  reply greet(user.name) + "! You have $total coins."
}
```

Uma função própria é chamada igual às embutidas:

- `giveCoins(user, 10)`
- `giveCoins user 10` (sem parênteses, no começo da linha)
- `giveCoins(who: user, amount: 10)` (pelo nome)

## Tipos são opcionais

Os tipos são os mesmos dos argumentos de `command`: `text`, `number`, `bool`, `member`, `user`, `channel` e `role`.

- **Sem tipo** (`fn greet(name)`), o argumento aceita qualquer valor.
- **Com tipo** (`amount: number`), o valor é convertido (`"10"` vira `10`) e, se não der, o erro E0302 aponta a **chamada**, não o meio da função.

## O que a função enxerga

Dentro da função existem só:

- os argumentos dela,
- os `local` criados dentro dela,
- as variáveis automáticas: `user`, `server`, `channel`, `message`.

Ela **não** enxerga os `local` de quem chamou. Se precisa de um valor, passe como argumento:

```dcs
fn label(price) {
  return "Price: $price coins"
}

command("shop") {
  local price = 50
  reply label(price)
}
```

## return e stop

- `return valor` devolve o resultado. Sem `return`, a função devolve `null`.
- `stop` dentro de uma função encerra **o comando inteiro**. É útil para checagens:

```preview
{"messages": [{"from": "user", "text": "!clear-warnings @Joe"}, {"from": "bot", "text": "Only moderators can do that."}]}
```

```dcs
fn requireMod() {
  if not hasRole(user, "Moderator") {
    reply "Only moderators can do that." ephemeral: true
    stop
  }
}

command("clear-warnings", target: member) {
  requireMod()
  delete("warnings-" + target.id, scope: server)
  reply "Warnings cleared."
}
```

## Recursão

Uma função pode chamar ela mesma, até 64 chamadas uma dentro da outra. Passar disso é o erro E0211 (para não travar o bot).

```preview
{"messages": [{"from": "user", "text": "!fact 5"}, {"from": "bot", "text": "120"}]}
```

```dcs
fn fact(n: number) {
  if n <= 1 { return 1 }
  return n * fact(n - 1)
}

command("fact", n: number) {
  reply fact(n)
}
```

## Regras rápidas

- Use `fn`, não `function` (o `check --fix` troca para você).
- O nome não pode ser igual ao de uma função embutida: `fn reply(x)` é o erro E0116.
- Argumentos opcionais (`= valor`) vêm por último.
- Funções não são valores: não dá para guardar `giveCoins` numa variável.
