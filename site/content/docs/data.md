O Diracord tem um banco de dados embutido, o **Dirabank**: um arquivo só (`data/bot.dirabank`), sem servidor e sem configuração.

## get, set e delete

```preview
{"messages": [{"from": "user", "text": "!coin"}, {"from": "bot", "text": "You have 1 coins."}, {"from": "user", "text": "!coin"}, {"from": "bot", "text": "You have 2 coins."}, {"from": "user", "text": "!reset"}, {"from": "bot", "text": "Coins reset."}]}
```

```dcs
command("coin") {
  local coins = get("coins", 0, scope: user) + 1
  set("coins", coins, scope: user)
  reply "You have $coins coins."
}

command("reset") {
  delete("coins", scope: user)
  reply "Coins reset."
}
```

- `get(nome, padrão?, scope?)` lê um valor; se nada foi salvo, devolve o padrão (ou `null`).
- `set(nome, valor, scope?)` salva (fica guardado mesmo depois de reiniciar o bot).
- `delete(nome, scope?)` apaga e devolve `true` se existia.

## Escopos

O escopo diz **de quem** é o valor:

| escopo | um valor por | exemplo |
|---|---|---|
| `user` | usuário, em todos os servidores | moedas globais |
| `server` | servidor | prefixo ou canal de logs do servidor |
| `member` | usuário dentro de cada servidor | XP e avisos por servidor |
| `global` | o bot inteiro | contador de comandos |

```dcs
command("xp") {
  local xp = get("xp", 0, scope: member) + 10
  set("xp", xp, scope: member)
  reply "You have $xp XP in this server."
}

command("uses") {
  local n = get("uses", 0, scope: global) + 1
  set("uses", n, scope: global)
  reply "This bot was used $n times."
}
```

Sem `scope:`, vale o `default_scope` do `diracord.toml` (padrão: `user`). Em mensagem privada não existe servidor: `scope: server` e `scope: member` dão o erro E0402 ali.

> Atenção: `scope: user` e `scope: member` usam **quem rodou o comando**. Para guardar algo de **outra pessoa** (os avisos de quem foi mencionado, por exemplo), coloque o id dela no nome e use `scope: server`:

```preview
{"messages": [{"from": "user", "text": "!warn @Joe spam", "name": "Mod"}, {"from": "bot", "text": "Joe now has 1 warnings. Reason: spam"}]}
```

```dcs
command("warn", target: member, reason: text = "no reason") {
  local key = "warnings-" + target.id
  local n = get(key, 0, scope: server) + 1
  set(key, n, scope: server)
  reply "$target.name now has $n warnings. Reason: $reason"
}
```

## O que dá para salvar

Qualquer valor: números, texto, `true`/`false`, listas e objetos (até 64 KB por valor).

```dcs
command("save-profile", bio: text) {
  set("profile", { bio: bio, since: now() }, scope: user)
  reply "Saved."
}

command("my-profile") {
  local p = get("profile", null, scope: user)
  if not p {
    reply "No profile yet."
    stop
  }
  reply "Bio: $p.bio"
}
```

## O arquivo

O Dirabank grava cada mudança no fim do arquivo e confere tudo com uma soma de verificação: se a máquina desligar no meio de uma gravação, ao abrir ele descarta só o pedaço cortado. De tempos em tempos o arquivo é compactado sozinho. O caminho fica em `[data] path` no `diracord.toml`.
