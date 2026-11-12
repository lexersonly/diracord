Total: 54 funções. Todas podem ser chamadas com ou sem parênteses:
`reply "hi"` é igual a `reply("hi")`. Argumentos com `?` são opcionais, `...` aceita vários, e
também podem ser passados pelo nome: `get("coins", scope: user)`.

## messages

### `reply(content: text|embed, ephemeral?: bool)`

Responde à mensagem ou interação atual. Aceita texto ou embed.

| argumento | tipo | descrição |
|---|---|---|
| `content` | text\|embed | texto ou embed { title: ... } |
| `ephemeral` | bool (opcional) | só quem usou o comando vê (slash) |

Devolve: `message`

```diracord
reply "Pong!"
```

### `send(channel: channel, content: text|embed)`

Envia uma mensagem em um canal.

| argumento | tipo | descrição |
|---|---|---|
| `channel` | channel | canal (objeto, id ou #menção) |
| `content` | text\|embed | texto ou embed |

Devolve: `message`

```diracord
send channel("general") "Hello!"
```

### `dm(user: user, content: text|embed)`

Envia uma mensagem privada para um usuário.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | usuário (objeto, id ou menção) |
| `content` | text\|embed | texto ou embed |

Devolve: `message`

```diracord
dm user "Welcome!"
```

### `react(emoji: text)`

Reage à mensagem atual com um emoji.

| argumento | tipo | descrição |
|---|---|---|
| `emoji` | text | emoji, como "👍" ou <:nome:id> |

Devolve: `null`

```diracord
react "👍"
```

### `deleteMessage(message?: any)`

Apaga a mensagem atual (ou outra, pelo id).

| argumento | tipo | descrição |
|---|---|---|
| `message` | any (opcional) | mensagem (objeto ou id); sem nada, a atual |

Devolve: `null`

```diracord
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

```diracord
ban user "spam"
```

### `unban(user: user, reason?: text)`

Tira o banimento de um usuário.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | usuário (id) |
| `reason` | text (opcional) | motivo |

Devolve: `null`

```diracord
unban "123456789"
```

### `kick(user: user, reason?: text)`

Expulsa um membro do servidor.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | quem será expulso |
| `reason` | text (opcional) | motivo |

Devolve: `null`

```diracord
kick user "rules"
```

### `timeout(user: user, seconds: number, reason?: text)`

Deixa um membro de castigo (sem falar) por alguns segundos. 0 tira o castigo.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | quem |
| `seconds` | number | duração em segundos (máximo 28 dias) |
| `reason` | text (opcional) | motivo |

Devolve: `null`

```diracord
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

```diracord
addRole user "VIP"
```

### `removeRole(user: user, role: role)`

Tira um cargo de um membro.

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | o membro |
| `role` | role | cargo (objeto, id, menção ou nome) |

Devolve: `null`

```diracord
removeRole user "VIP"
```

### `hasRole(user: user, role: role)`

Diz se um membro tem um cargo (true/false).

| argumento | tipo | descrição |
|---|---|---|
| `user` | user | o membro |
| `role` | role | cargo (objeto, id, menção ou nome) |

Devolve: `bool`

```diracord
if hasRole(user, "VIP") { reply "vip!" }
```

## lookup

### `channel(query: any)`

Procura um canal do servidor pelo nome, id ou menção. Devolve null se não achar.

| argumento | tipo | descrição |
|---|---|---|
| `query` | any | nome, id ou menção |

Devolve: `channel`

```diracord
send channel("welcome") "Hi"
```

### `member(query: any)`

Procura um membro do servidor pelo id, menção ou nome. Devolve null se não achar.

| argumento | tipo | descrição |
|---|---|---|
| `query` | any | nome, id ou menção |

Devolve: `member`

```diracord
local m = member("123")
```

### `role(query: any)`

Procura um cargo pelo nome, id ou menção. Devolve null se não achar.

| argumento | tipo | descrição |
|---|---|---|
| `query` | any | nome, id ou menção |

Devolve: `role`

```diracord
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

```diracord
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

```diracord
set("coins", get("coins", 0) + 1, scope: user)
```

### `delete(name: text, scope?: scope)`

Apaga um valor salvo. Devolve true se existia.

| argumento | tipo | descrição |
|---|---|---|
| `name` | text | nome da variável salva |
| `scope` | scope (opcional) | user, server, member ou global |

Devolve: `bool`

```diracord
delete("coins", scope: user)
```

## text

### `upper(text: text)`

Texto em MAIÚSCULAS.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |

Devolve: `text`

```diracord
reply upper("hi")
```

### `lower(text: text)`

Texto em minúsculas.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |

Devolve: `text`

```diracord
local t = lower(message.content)
```

### `trim(text: text)`

Tira espaços do começo e do fim.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |

Devolve: `text`

```diracord
local t = trim("  hi ")
```

### `len(value: any)`

Tamanho de um texto (em letras) ou de uma lista.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | texto ou lista |

Devolve: `number`

```diracord
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

```diracord
replace("hello world", "world", "Diracord")
```

### `split(text: text, separator?: text)`

Divide um texto em uma lista. Sem separador, divide nos espaços.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |
| `separator` | text (opcional) | separador |

Devolve: `list`

```diracord
local words = split(message.content)
```

### `startsWith(text: text, prefix: text)`

Diz se o texto começa com outro.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |
| `prefix` | text | o começo |

Devolve: `bool`

```diracord
if startsWith(message.content, "!") { stop }
```

### `endsWith(text: text, suffix: text)`

Diz se o texto termina com outro.

| argumento | tipo | descrição |
|---|---|---|
| `text` | text | o texto |
| `suffix` | text | o fim |

Devolve: `bool`

```diracord
endsWith(name, ".png")
```

### `contains(haystack: any, needle: any)`

Diz se um texto contém outro, ou se uma lista contém um valor.

| argumento | tipo | descrição |
|---|---|---|
| `haystack` | any | texto ou lista |
| `needle` | any | o que procurar |

Devolve: `bool`

```diracord
if contains(message.content, "badword") { deleteMessage }
```

## math

### `random(min?: number, max?: number)`

Número inteiro aleatório entre min e max (inclusive). Sem argumentos: entre 0 e 1 (decimal).

| argumento | tipo | descrição |
|---|---|---|
| `min` | number (opcional) | menor valor |
| `max` | number (opcional) | maior valor |

Devolve: `number`

```diracord
local dice = random(1, 6)
```

### `round(number: number)`

Arredonda para o inteiro mais próximo.

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```diracord
round(2.5)
```

### `floor(number: number)`

Arredonda para baixo.

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```diracord
floor(2.5)
```

### `ceil(number: number)`

Arredonda para cima.

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```diracord
ceil(2.5)
```

### `abs(number: number)`

Valor absoluto (sem sinal).

| argumento | tipo | descrição |
|---|---|---|
| `number` | number | o número |

Devolve: `number`

```diracord
abs(-3)
```

### `min(a: number, b: number)`

O menor de dois números.

| argumento | tipo | descrição |
|---|---|---|
| `a` | number | primeiro |
| `b` | number | segundo |

Devolve: `number`

```diracord
min(5, 3)
```

### `max(a: number, b: number)`

O maior de dois números.

| argumento | tipo | descrição |
|---|---|---|
| `a` | number | primeiro |
| `b` | number | segundo |

Devolve: `number`

```diracord
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

```diracord
list = push(list, "new")
```

### `join(list: list, separator?: text)`

Junta os itens de uma lista em um texto.

| argumento | tipo | descrição |
|---|---|---|
| `list` | list | a lista |
| `separator` | text (opcional) | separador (padrão ", ") |

Devolve: `text`

```diracord
reply join(names, ", ")
```

### `first(list: list)`

Primeiro item da lista (ou null).

| argumento | tipo | descrição |
|---|---|---|
| `list` | list | a lista |

Devolve: `any`

```diracord
first(args)
```

### `last(list: list)`

Último item da lista (ou null).

| argumento | tipo | descrição |
|---|---|---|
| `list` | list | a lista |

Devolve: `any`

```diracord
last(args)
```

### `range(start: number, end?: number)`

Lista de números de start até end (sem incluir end). Com um argumento, de 0 até ele.

| argumento | tipo | descrição |
|---|---|---|
| `start` | number | início (ou fim, se for o único) |
| `end` | number (opcional) | fim (não incluído) |

Devolve: `list`

```diracord
for i in range(1, 4) { reply i }
```

### `reverse(value: any)`

Inverte uma lista ou um texto.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | lista ou texto |

Devolve: `any`

```diracord
reverse([1, 2, 3])
```

## utility

### `typeOf(value: any)`

Nome do tipo de um valor: null, bool, number, text, list ou object.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | o valor |

Devolve: `text`

```diracord
reply typeOf(x)
```

### `toNumber(value: any)`

Converte texto em número. Devolve null se não for número.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | o valor |

Devolve: `number`

```diracord
local n = toNumber("42")
```

### `toText(value: any)`

Converte qualquer valor em texto.

| argumento | tipo | descrição |
|---|---|---|
| `value` | any | o valor |

Devolve: `text`

```diracord
toText(42)
```

### `now()`

Hora atual em milissegundos desde 1970 (Unix).

Devolve: `number`

```diracord
local start = now()
```

## components

### `container(items...: component, color?: text, spoiler?: bool)`

Caixa com cor de destaque que agrupa outras peças.

| argumento | tipo | descrição |
|---|---|---|
| `items` | component | peças: text, section, row, separator, gallery, file |
| `color` | text (opcional) | cor da barra: "purple", "#ff0000"... |
| `spoiler` | bool (opcional) | esconde até clicar |

Devolve: `component`

```diracord
reply container(text("## Hello"), row(button("ok", "Ok")), color: "purple")
```

### `text(content: text)`

Texto (com markdown) dentro do layout.

| argumento | tipo | descrição |
|---|---|---|
| `content` | text | o texto; aceita markdown (## título, **negrito**) |

Devolve: `component`

```diracord
reply text("## Hello, $user.name")
```

### `section(texts...: text, thumbnail?: any, button?: component)`

1 a 3 textos com uma imagem pequena (thumbnail) ou um botão ao lado.

| argumento | tipo | descrição |
|---|---|---|
| `texts` | text | de 1 a 3 textos (ou text(...)) |
| `thumbnail` | any (opcional) | imagem ao lado: url ou thumbnail(...) |
| `button` | component (opcional) | ou um botão ao lado: button(...) |

Devolve: `component`

```diracord
reply container(section("Read the rules first.", thumbnail: user.avatar))
```

### `thumbnail(url: text, description?: text, spoiler?: bool)`

Imagem pequena ao lado de uma section (use em thumbnail: quando precisar de opções).

| argumento | tipo | descrição |
|---|---|---|
| `url` | text | endereço da imagem (https://...) |
| `description` | text (opcional) | texto alternativo |
| `spoiler` | bool (opcional) | esconde até clicar |

Devolve: `component`

```diracord
reply container(section("Hi", thumbnail: thumbnail(user.avatar, description: "avatar")))
```

### `separator(spacing?: text, line?: bool)`

Espaço ou linha entre peças.

| argumento | tipo | descrição |
|---|---|---|
| `spacing` | text (opcional) | "small" ou "large" |
| `line` | bool (opcional) | mostra a linha (padrão: true) |

Devolve: `component`

```diracord
reply container(text("a"), separator(spacing: "large"), text("b"))
```

### `gallery(images...: any)`

De 1 a 10 imagens grandes.

| argumento | tipo | descrição |
|---|---|---|
| `images` | any | de 1 a 10 urls (ou { url, description, spoiler }) |

Devolve: `component`

```diracord
reply gallery("https://example.com/a.png", "https://example.com/b.png")
```

### `file(url: text, spoiler?: bool)`

Mostra um arquivo enviado junto (attachment://nome).

| argumento | tipo | descrição |
|---|---|---|
| `url` | text | "attachment://nome-do-arquivo" |
| `spoiler` | bool (opcional) | esconde até clicar |

Devolve: `component`

```diracord
reply file("attachment://report.pdf")
```

### `row(items...: component)`

Uma linha com até 5 botões, ou com um menu.

| argumento | tipo | descrição |
|---|---|---|
| `items` | component | até 5 button(...), ou um menu(...) |

Devolve: `component`

```diracord
reply row(button("yes", "Yes"), button("no", "No", style: "danger"))
```

### `button(id?: text, label?: text, style?: text, emoji?: text, disabled?: bool, url?: text)`

Um botão. O clique roda o button("id") { } com o mesmo id; com url, vira link.

| argumento | tipo | descrição |
|---|---|---|
| `id` | text (opcional) | id que liga ao button("id") { } (não use com url) |
| `label` | text (opcional) | texto do botão |
| `style` | text (opcional) | primary, secondary, success, danger |
| `emoji` | text (opcional) | emoji, como "👍" |
| `disabled` | bool (opcional) | botão desligado |
| `url` | text (opcional) | vira botão de link (abre o endereço) |

Devolve: `component`

```diracord
reply row(button("accept", "Accept", style: "success"), button(label: "Site", url: "https://example.com"))
```

### `menu(id: text, options: list, placeholder?: text, min?: number, max?: number)`

Menu de opções. A escolha roda o menu("id") { }; os valores ficam em message.values.

| argumento | tipo | descrição |
|---|---|---|
| `id` | text | id que liga ao menu("id") { } |
| `options` | list | lista de textos ou { label, value, description } |
| `placeholder` | text (opcional) | texto antes de escolher |
| `min` | number (opcional) | mínimo de escolhas |
| `max` | number (opcional) | máximo de escolhas |

Devolve: `component`

```diracord
reply row(menu("color", ["Red", "Blue"], placeholder: "Pick one"))
```

