# Funções embutidas

> Gerado por `cargo run -q -p diracord-functions --example gendocs`. Não edite à mão.

Total: 44 funções. Todas podem ser chamadas com ou sem parênteses:
`reply "oi"` é igual a `reply("oi")`. Argumentos com `?` são opcionais e
também podem ser passados pelo nome: `get("coins", scope: user)`.

## messages

### `reply(content: text|embed, ephemeral?: bool)`

Responde à mensagem ou interação atual. Aceita texto ou embed.

| argumento | tipo | descrição |
|---|---|---|
| `content` | text\|embed | texto ou embed { title: ... } |
| `ephemeral` | bool (opcional) | só quem usou o comando vê (slash) |

Devolve: `message`

```
reply "Pong!"
```

### `send(channel: channel, content: text|embed)`

Envia uma mensagem em um canal.

| argumento | tipo | descrição |
|---|---|---|
| `channel` | channel | canal (objeto, id ou #menção) |
| `content` | text\|embed | texto ou embed |

Devolve: `message`

```
send channel("geral") "Olá!"
```

### `dm(user: user, content: text|embed)`

Envia uma mensagem privada para um usuário.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | usuário (objeto, id ou menção) |
| `content` | text\|embed | texto ou embed |

Devolve: `message`

```
dm user "Bem-vindo!"
```

### `react(emoji: text)`

Reage à mensagem atual com um emoji.

| argumento | tipo | descrição |
|---|---|---|
| `emoji` | text | emoji, como "👍" ou <:nome:id> |

Devolve: `null`

```
react "👍"
```

### `deleteMessage(message?: any)`

Apaga a mensagem atual (ou outra, pelo id).

| argumento | tipo | descrição |
|---|---|---|
| `message` | any (opcional) | mensagem (objeto ou id); sem nada, a atual |

Devolve: `null`

```
deleteMessage
```

## moderation

### `ban(user: user, reason?: text)`

Bane um membro do servidor.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | quem será banido |
| `reason` | text (opcional) | motivo (aparece no registro de auditoria) |

Devolve: `null`

```
ban user "spam"
```

### `unban(user: user, reason?: text)`

Tira o banimento de um usuário.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | usuário (id) |
| `reason` | text (opcional) | motivo |

Devolve: `null`

```
unban "123456789"
```

### `kick(user: user, reason?: text)`

Expulsa um membro do servidor.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | quem será expulso |
| `reason` | text (opcional) | motivo |

Devolve: `null`

```
kick user "regras"
```

### `timeout(user: user, seconds: number, reason?: text)`

Deixa um membro de castigo (sem falar) por alguns segundos. 0 tira o castigo.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | quem |
| `seconds` | number | duração em segundos (máximo 28 dias) |
| `reason` | text (opcional) | motivo |

Devolve: `null`

```
timeout user 60 "flood"
```

## roles

### `addRole(user: user, role: role)`

Dá um cargo para um membro.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | o membro |
| `role` | role | cargo (objeto, id, menção ou nome) |

Devolve: `null`

```
addRole user "VIP"
```

### `removeRole(user: user, role: role)`

Tira um cargo de um membro.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | o membro |
| `role` | role | cargo (objeto, id, menção ou nome) |

Devolve: `null`

```
removeRole user "VIP"
```

### `hasRole(user: user, role: role)`

Diz se um membro tem um cargo (true/false).

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | o membro |
| `role` | role | cargo (objeto, id, menção ou nome) |

Devolve: `bool`

```
if hasRole(user, "VIP") { reply "vip!" }
```

## lookup

### `channel(query: any)`

Procura um canal do servidor pelo nome, id ou menção. Devolve null se não achar.

| argumento | tipo | descrição |
|---|---|---|
| `query` | any | nome, id ou menção |

Devolve: `channel`

```
send channel("welcome") "Oi"
```

### `member(query: any)`

Procura um membro do servidor pelo id, menção ou nome. Devolve null se não achar.

| argumento | tipo | descrição |
|---|---|---|
| `query` | any | nome, id ou menção |

Devolve: `member`

```
local m = member("123")
```

### `role(query: any)`

Procura um cargo pelo nome, id ou menção. Devolve null se não achar.

| argumento | tipo | descrição |
|---|---|---|
| `query` | any | nome, id ou menção |

Devolve: `role`

```
local vip = role("VIP")
```

## data

### `get(name: text, default?: any, scope?: scope)`

Lê um valor salvo. Se não existir, devolve o padrão (ou null).

| argumento | tipo | descrição |
|---|---|---|
| `name` | text | nome da variável salva |
| `default` | any (opcional) | valor se nada foi salvo |
| `scope` | scope (opcional) | user, server, member ou global |

Devolve: `any`

```
get("coins", 0, scope: user)
```

### `set(name: text, value: any, scope?: scope)`

Salva um valor (fica guardado mesmo depois de reiniciar o bot).

| argumento | tipo | descrição |
|---|---|---|
| `name` | text | nome da variável salva |
| `value` | any | o valor |
| `scope` | scope (opcional) | user, server, member ou global |

Devolve: `null`

```
set("coins", get("coins", 0) + 1, scope: user)
```

### `delete(name: text, scope?: scope)`

Apaga um valor salvo. Devolve true se existia.

| argumento | tipo | descrição |
|---|---|---|
| `name` | text | nome da variável salva |
| `scope` | scope (opcional) | user, server, member ou global |

Devolve: `bool`

```
delete("coins", scope: user)
```

## text

### `upper(text: text)`

Texto em MAIÚSCULAS.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |

Devolve: `text`

```
reply upper("oi")
```

### `lower(text: text)`

Texto em minúsculas.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |

Devolve: `text`

```
local t = lower(message.content)
```

### `trim(text: text)`

Tira espaços do começo e do fim.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |

Devolve: `text`

```
local t = trim("  oi ")
```

### `len(value: any)`

Tamanho de um texto (em letras) ou de uma lista.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | texto ou lista |

Devolve: `number`

```
if len(reason) > 100 { stop }
```

### `replace(text: text, from: text, to: text)`

Troca todas as ocorrências de um pedaço do texto.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |
| `from` | text | o que procurar |
| `to` | text | pelo que trocar |

Devolve: `text`

```
replace("oi mundo", "mundo", "Diracord")
```

### `split(text: text, separator?: text)`

Divide um texto em uma lista. Sem separador, divide nos espaços.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |
| `separator` | text (opcional) | separador |

Devolve: `list`

```
local words = split(message.content)
```

### `startsWith(text: text, prefix: text)`

Diz se o texto começa com outro.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |
| `prefix` | text | o começo |

Devolve: `bool`

```
if startsWith(message.content, "!") { stop }
```

### `endsWith(text: text, suffix: text)`

Diz se o texto termina com outro.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |
| `suffix` | text | o fim |

Devolve: `bool`

```
endsWith(name, ".png")
```

### `contains(haystack: any, needle: any)`

Diz se um texto contém outro, ou se uma lista contém um valor.

| argumento | tipo | descrição |
|---|---|---|
| `haystack` | any | texto ou lista |
| `needle` | any | o que procurar |

Devolve: `bool`

```
if contains(message.content, "palavrão") { deleteMessage }
```

## math

### `random(min?: number, max?: number)`

Número inteiro aleatório entre min e max (inclusive). Sem argumentos: entre 0 e 1 (decimal).

| argumento | tipo | descrição |
|---|---|---|
| `min` | number (opcional) | menor valor |
| `max` | number (opcional) | maior valor |

Devolve: `number`

```
local dado = random(1, 6)
```

### `round(number: number)`

Arredonda para o inteiro mais próximo.

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```
round(2.5)
```

### `floor(number: number)`

Arredonda para baixo.

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```
floor(2.5)
```

### `ceil(number: number)`

Arredonda para cima.

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```
ceil(2.5)
```

### `abs(number: number)`

Valor absoluto (sem sinal).

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```
abs(-3)
```

### `min(a: number, b: number)`

O menor de dois números.

| argumento | tipo | descrição |
|---|---|---|
| `a` | number | primeiro |
| `b` | number | segundo |

Devolve: `number`

```
min(5, 3)
```

### `max(a: number, b: number)`

O maior de dois números.

| argumento | tipo | descrição |
|---|---|---|
| `a` | number | primeiro |
| `b` | number | segundo |

Devolve: `number`

```
max(5, 3)
```

## lists

### `push(list: list, value: any)`

Devolve uma nova lista com o valor no fim.

| argumento | tipo | descrição |
|---|---|---|
| `list` | list | a lista |
| `value` | any | o valor |

Devolve: `list`

```
list = push(list, "novo")
```

### `join(list: list, separator?: text)`

Junta os itens de uma lista em um texto.

| argumento | tipo | descrição |
|---|---|---|
| `list` | list | a lista |
| `separator` | text (opcional) | separador (padrão ", ") |

Devolve: `text`

```
reply join(nomes, ", ")
```

### `first(list: list)`

Primeiro item da lista (ou null).

| argumento | tipo | descrição |
|---|---|---|
| `list` | list | a lista |

Devolve: `any`

```
first(args)
```

### `last(list: list)`

Último item da lista (ou null).

| argumento | tipo | descrição |
|---|---|---|
| `list` | list | a lista |

Devolve: `any`

```
last(args)
```

### `range(start: number, end?: number)`

Lista de números de start até end (sem incluir end). Com um argumento, de 0 até ele.

| argumento | tipo | descrição |
|---|---|---|
| `start` | number | início (ou fim, se for o único) |
| `end` | number (opcional) | fim (não incluído) |

Devolve: `list`

```
for i in range(1, 4) { reply i }
```

### `reverse(value: any)`

Inverte uma lista ou um texto.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | lista ou texto |

Devolve: `any`

```
reverse([1, 2, 3])
```

## utility

### `typeOf(value: any)`

Nome do tipo de um valor: null, bool, number, text, list ou object.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | o valor |

Devolve: `text`

```
reply typeOf(x)
```

### `toNumber(value: any)`

Converte texto em número. Devolve null se não for número.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | o valor |

Devolve: `number`

```
local n = toNumber("42")
```

### `toText(value: any)`

Converte qualquer valor em texto.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | o valor |

Devolve: `text`

```
toText(42)
```

### `now()`

Hora atual em milissegundos desde 1970 (Unix).

Devolve: `number`

```
local start = now()
```

