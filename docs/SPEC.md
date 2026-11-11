# Diracord — Especificação da linguagem

> Status: rascunho v0.1, baseado nas decisões do dono do projeto.
> Itens marcados **[ABERTO]** ainda não foram decididos. Não invente: pergunte.

## 1. O que é

Diracord é uma linguagem de texto para criar bots de Discord. Ela existe para ser:

- **Mais fácil** que aoi.js e discord.js, para iniciantes e para quem vem do aoi.js.
- **Mais completa**: variáveis, condições, laços, funções próprias, erros tratáveis.
- **Mais rápida** de escrever (menos linhas) e de executar (motor em Rust).
- **Leve**: um servidor com ~100 MB de RAM deve conseguir hospedar bots e atender 1000+ usuários.

Público: iniciantes totais, quem vem do aoi.js e devs de JS, igualmente.

## 2. Escopo da v1

Suportado: comandos de prefixo, slash commands (mesmo código), eventos, botões,
menus e modais, embeds e **Components v2** do Discord.

Fora da v1: testar comandos sem ligar o bot (`diracord test`), editor web, hospedagem
multiusuário. A hospedagem é só uma meta de projeto (ver ARCHITECTURE.md).

## 3. Princípios de sintaxe

1. Palavras-chave em **inglês**, curtas.
2. Blocos com **chaves `{ }`**, como JS/TS. Sem indentação significativa.
3. Parênteses são **opcionais** em chamadas simples: `reply "oi"` e `reply("oi")` são iguais.
4. Texto interpola variáveis com `$`: `"Oi $user.name"`.
5. Tipos só existem nos **argumentos de comando**. O resto é dinâmico.
6. Extensões de arquivo aceitas: `.dcs`, `.dira`, `.diracord`.
7. Comentários com `//`.

## 4. Elementos da linguagem

### 4.1 Comandos
O mesmo comando atende prefixo e slash (cada modo é ligado em `diracord.toml`).

```
command("ping") {
  reply "Pong!"
}

command("ban", user: member, reason: text = "no reason") {
  ban user reason
  reply "$user.name foi banido. Motivo: $reason"
}
```

Tipos de argumento: `text`, `number`, `bool`, `member`, `user`, `channel`, `role`.
Argumento com `= valor` é opcional.

### 4.2 Eventos
Nome do evento direto, sem palavra-chave:

```
memberJoin {
  send channel("welcome") "Bem-vindo, $user.mention!"
}
```

### 4.3 Interações (botão, menu, modal)
O código do clique fica **separado do botão**, ligado por **id** (estilo aoi.js):

```
button("confirm") {
  reply "Confirmado!"
}
```

### 4.4 Variáveis
```
local x = 15          // some quando o comando termina
x = x + 1
```
Dados salvos usam funções `get`/`set`, com escopo configurável:
```
set("coins", get("coins", scope: user) + 1, scope: user)
```
Escopos: `user`, `server`, `member` (usuário dentro de um servidor) e `global`.
A pessoa escolhe e combina os escopos que quiser.

### 4.5 Controle de fluxo
```
if x > 5 { ... } else if x > 2 { ... } else { ... }
for item in list { ... }
while cond { ... }
chain x { 1 => reply "um"   "a" => reply "letra"   _ => reply "outro" }
testing { ban user } catch err { reply "Falhou: $err" }
```
- `chain` é o nome escolhido para o "switch/match".
- Tratamento de erro (decidido em 2026-11-11): `testing { } catch err { }`. Escrever `try`
  é erro com correção automática segura para `testing`.
- Também existem `return`, `stop` (encerra o comando), `break` e `continue`.

### 4.6 Respostas, embeds e Components v2
- Embed como objeto estilo JSON: `reply { title: "Oi", color: "red" }`.
- Components v2 (decidido em 2026-11-11, ver `docs/rfcs/components-v2.md`): layout em
  **blocos** aberto pela palavra `layout` (proposta A), com `if`/`for`/`local` dentro, e a
  forma **objeto** `{ layout: [...] }` (proposta B) aceita também. As duas geram a mesma árvore.
  Componentes sem bloco aceitam opções nomeadas: `separator(spacing: "large")`.
  ```
  reply layout {
    container(color: "purple") {
      text "## Bem-vindo, $user.name"
      separator
      row { button("aceitar", "Aceito", style: "success") }
    }
  }
  ```

### 4.7 Funções próprias
```
fn greet(name: text) { return "Oi $name" }
```
**[ABERTO]** sintaxe exata e se aceita tipos.

### 4.8 Atalho estilo aoi.js
`$ban[user;reason]` equivale a `ban(user, reason)`. Fica **desligado/ligado por
configuração** em `diracord.toml` (`[syntax] dollar = true`). Pode haver outros
"sistemas" de sintaxe plugáveis do mesmo jeito.

### 4.9 Variáveis automáticas
Dentro de comandos e eventos existem: `user` (alias `author`), `server`, `channel`,
`message` (alias `interaction` quando for interação).

## 5. Erros

O sistema de erros é parte central do produto. Todo erro deve:

1. **Explicar em linguagem simples** ("Faltou fechar a chave na linha 5").
2. **Mostrar o trecho** do código, com seta apontando o problema.
3. **Sugerir correção** ("Você quis dizer `reply`?").
4. **Corrigir sozinho** (auto-fix) quando a correção for segura.

Cada erro tem código estável (`E0001`...) para documentação e para IAs.

## 6. Projeto e configuração

```
meu-bot/
  diracord.toml     # configuração
  .env              # token (nunca no código)
  commands/ping.dcs
  events/join.dcs
```
- Projeto é uma **pasta com vários arquivos**, importados automaticamente.
- `diracord.toml` guarda prefixo, modos (prefixo/slash), sintaxes extras,
  escopos de dados, limites e `ram` (ver ARCHITECTURE.md).
- CLI: `diracord run`, `diracord check`, `diracord new`, `diracord watch`.

## 7. Segurança e limites (padrão)

Limite de tempo por execução, limite de memória, cooldown e rate limit,
permissões por comando.

## 8. Dados: Dirabank

`get/set` usam o **Dirabank**, banco chave-valor embutido, escrito em Rust, em arquivo
único e leve. O driver é **plugável**, então outros bancos (como Turso) podem ser
ligados por configuração depois.

Formato do arquivo (decidido em 2026-11-11, `docs/rfcs/dirabank-format.md`, proposta A):
**log só de acréscimo** com CRC32 por registro e índice em memória (os valores ficam no
disco). Versão `1` no cabeçalho; arquivos `v0` são atualizados ao abrir.

## 9. Extensões

Terceiros adicionam funções por **plugins WASM** (isolados e seguros).

## 10. Decisões em aberto (resumo)
- Sintaxe das funções próprias.
- Alias `guild` para `server`.
