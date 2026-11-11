# Protocolo do motor (versão 1)

O `diracord-engine` (Rust) fala com o lado TypeScript por **JSON, uma mensagem por
linha**, no stdin/stdout. Logs vão para o stderr. É o modo "serviço" da
[arquitetura](ARCHITECTURE.md); o modo embutido (napi-rs) vai usar as mesmas mensagens.

O protocolo é **sequencial**: um pedido por vez. O cliente TS (`@diracord/engine`)
coloca os pedidos numa fila.

## Ao iniciar

```json
{"op":"hello","protocol":1,"version":"0.1.0"}
```

## Pedidos

### `check` — conferir sem carregar

```json
{"op":"check","files":[{"path":"commands/ping.dcs","source":"..."}],"syntax":{"dollar":false},"color":false}
```
Resposta:
```json
{"op":"checked","ok":false,"errors":1,"warnings":0,"diagnostics":[Diagnostic],"rendered":"erro[E0201]: ..."}
```

`Diagnostic`:
```json
{"code":"E0201","severity":"erro","message":"A função `rpely` não existe",
 "file":"commands/ping.dcs","line":2,"col":3,"endLine":2,"endCol":8,
 "help":["você quis dizer `reply`?"],"notes":[],
 "fixes":[{"message":"trocar `rpely` por `reply`","safe":false,"edits":[{"start":20,"end":25,"replacement":"reply"}]}],
 "rendered":"erro[E0201]: ...\n --> commands/ping.dcs:2:3\n ..."}
```
`start`/`end` são posições em bytes no texto do arquivo.

### `fix` — aplicar correções seguras

Igual ao `check`, com `"op":"fix"`. A resposta (`"op":"fixed"`) traz também
`"files":[{"path","source","changed"}]` com as correções **seguras** aplicadas.

### `load` — carregar o projeto

```json
{"op":"load","files":[...],"syntax":{"dollar":false},
 "data":{"path":"/abs/data/bot.dirabank"},"defaultScope":"user",
 "limits":{"maxSteps":100000,"maxTimeMs":3000,"maxMemoryKb":4096,"maxActions":50}}
```
Sem `data`, os dados ficam só na memória. Se houver erros, `ok` é `false` e o projeto
anterior continua carregado. Resposta:
```json
{"op":"loaded","ok":true,"protocol":1,"diagnostics":[],"rendered":"",
 "commands":[{"name":"ban","params":[{"name":"user","type":"member","optional":false}]}],
 "events":["memberJoin"],"interactions":[{"kind":"button","id":"confirm"}]}
```

### `dispatch` — rodar um comando, evento ou interação

```json
{"op":"dispatch","id":1,
 "trigger":{"kind":"command","name":"ban","args":{"user":{"id":"5","name":"Zé"},"reason":"spam"}},
 "context":{"user":{...},"server":{...},"channel":{...},"message":{...},"isInteraction":false,"extra":{}}}
```
`trigger` pode ser `{"kind":"event","name":"memberJoin"}` ou
`{"kind":"interaction","type":"button","id":"confirm"}`.
Argumentos de comando podem vir como texto: o motor converte para `number`/`bool`
conforme o tipo declarado. Para `member`/`user`/`channel`/`role`, mande o objeto
(ou pelo menos o id/menção).

Durante a execução, o motor pede ações:
```json
{"op":"action","id":1,"seq":1,"action":{"name":"ban","args":["5","spam"],"options":{}}}
```
e espera a resposta com o mesmo `id` e `seq`:
```json
{"op":"result","id":1,"seq":1,"ok":true,"value":null}
{"op":"result","id":1,"seq":1,"ok":false,"error":"Missing Permissions"}
```
Um `ok:false` vira um erro `E0208` que o código pode tratar com `testing { } catch err { }`.

No fim:
```json
{"op":"done","id":1,"ok":true,"steps":12,"actions":1,"value":null}
{"op":"done","id":1,"ok":false,"error":{"code":"E0203","message":"...","file":"a.dcs","rendered":"..."}}
```

### `shutdown`

```json
{"op":"shutdown"}  →  {"op":"bye"}
```
Grava os dados pendentes e encerra.

### Erros do protocolo

Pedido inválido → `{"op":"error","message":"...","id":...}`.

## Ações

| ação | args | o host devolve |
|---|---|---|
| `reply` | `[conteúdo]`, options `{ephemeral}` | mensagem ou `null` |
| `send` | `[canalId, conteúdo]` | mensagem |
| `dm` | `[userId, conteúdo]` | mensagem |
| `react` | `[emoji]` (na mensagem atual) | `null` |
| `deleteMessage` | `[mensagemId ou null]` | `null` |
| `ban` / `kick` | `[userId, motivo ou null]` | `null` |
| `unban` | `[userId, motivo ou null]` | `null` |
| `timeout` | `[userId, segundos, motivo ou null]` | `null` |
| `addRole` / `removeRole` | `[userId, cargoId ou nome]` | `null` |
| `hasRole` | `[userId, cargoId ou nome]` | `true`/`false` |
| `channel` / `member` / `role` | `[busca]` (id, menção ou nome) | objeto ou `null` |

`conteúdo` é texto ou um objeto de embed (`{title, description, color, fields, ...}`).

## Objetos (entidades)

| | campos |
|---|---|
| user | `id, name, username, mention, bot, avatar` |
| member | os de user + `nick, roles, joinedAt` |
| channel | `id, name, mention, type` |
| role | `id, name, mention, color` |
| server | `id, name, memberCount` |
| message | `id, content, channelId, authorId` (+ `values` em menus) |
