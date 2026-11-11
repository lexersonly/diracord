# RFC: funções próprias (`fn`)

> Status: **decidido em 2026-11-11**: proposta **B** (tipos opcionais), palavra `fn`,
> com as 9 regras comuns abaixo e sem funções como valor na v1. Implementado.
> Os exemplos usam nomes em inglês (pedido do dono).

## Contexto

Hoje o código só existe dentro de `command`, eventos e `button`/`menu`/`modal`.
Quem repete a mesma lógica em vários comandos precisa copiar e colar. A SPEC já
mostra um esboço:

```text
fn greet(name: text) { return "Hi $name" }
```

e deixa em aberto duas coisas: **a sintaxe exata** e **se aceita tipos**. Lembrete da
SPEC §3.5: "Tipos só existem nos argumentos de comando. O resto é dinâmico."

O mesmo exemplo nas três propostas: uma função que dá moedas e devolve o novo total,
usada por dois comandos.

---

## Regras que valem para as três propostas

Estas regras não dependem da sintaxe; listo para o dono confirmar ou mudar.

1. **Globais no projeto.** Uma função definida em qualquer arquivo pode ser usada em
   todos (como os comandos). Não existe import.
2. **Chamada igual às funções embutidas.** Com ou sem parênteses e com argumentos por
   nome: `giveCoins user 10` = `giveCoins(user, 10)` = `giveCoins(who: user, amount: 10)`.
3. **Não pode ter o nome de uma função embutida** (`reply`, `ban`...) nem de outra
   função própria: erro novo, com o lugar das duas definições.
4. **Variáveis isoladas.** Dentro da função só existem os argumentos, os `local` dela e
   as variáveis automáticas da execução (`user`, `server`, `channel`, `message`). Não
   enxerga os `local` de quem chamou.
5. **Podem agir no Discord** (`reply`, `ban`, `get`/`set`): as ações contam no mesmo
   limite da execução que chamou.
6. **`return` devolve o valor** e sai da função; sem `return`, devolve `null`.
   `stop` dentro da função encerra o comando inteiro (como hoje).
7. **Recursão permitida, com limite** de profundidade (ex.: 64 chamadas aninhadas;
   passar disso é o erro E0211 "aninhamento demais").
8. **Argumento faltando** é E0300 e **a mais** é E0301, com o `uso:` na dica, igual às
   funções embutidas. O `check` já confere isso antes de rodar.
9. Ficam **no topo do arquivo**, como `command` (não dentro de outro bloco).

---

## Proposta A — `fn` sem tipos (o mais simples)

```text
fn giveCoins(who, n = 1) {
  local total = get("coins", 0, scope: user) + n
  set("coins", total, scope: user)
  return total
}

command("daily") {
  local t = giveCoins(user, 100)
  reply "You got 100 coins. Total: $t"
}

command("coin") {
  reply "Total: " + giveCoins(user)
}
```

- `+` Segue a SPEC §3.5 à risca: tipos só nos comandos.
- `+` O menor jeito de escrever; nada novo para aprender além de `fn` e `return`.
- `+` Valor padrão com `= valor` igual ao de `command`.
- `-` Passar o tipo errado só dá erro lá dentro (ex.: `"abc" + 1` vira texto), mais
  longe de onde está o engano.
- `-` A documentação gerada e o `llm.txt` não conseguem dizer o que cada argumento espera.

## Proposta B — `fn` com tipos opcionais (igual aos argumentos de `command`)

```text
fn giveCoins(who: member, n: number = 1) {
  local total = get("coins", 0, scope: user) + n
  set("coins", total, scope: user)
  return total
}

fn greet(name) {            // type is optional: without it, any value is accepted
  return "Hi $name"
}
```

Os tipos são os mesmos dos comandos (`text`, `number`, `bool`, `member`, `user`,
`channel`, `role`) e funcionam do mesmo jeito: `"10"` vira `10` para `number`; um texto
qualquer para `member` é erro E0302 apontando **a chamada**.

- `+` Mesma escrita de `command(... user: member, reason: text = "x")`: quem sabe um
  sabe o outro.
- `+` Erro no lugar certo (na chamada), e o `check` pode avisar antes de rodar quando o
  valor é fixo (`giveCoins(user, "muito")`).
- `+` Documentação e `llm.txt` mostram o que cada argumento espera.
- `-` Muda a regra "tipos só nos comandos" da SPEC §3.5 (passa a ser "nos comandos e
  nas funções, e opcionais nas funções").
- `-` Um pouco mais de coisa para o iniciante ver (mas ele pode ignorar os tipos).

## Proposta C — `fn("nome", ...)`, espelho exato de `command`

```text
fn("giveCoins", who: member, n: number = 1) {
  local total = get("coins", 0, scope: user) + n
  set("coins", total, scope: user)
  return total
}

command("daily") {
  reply "Total: " + giveCoins(user, 100)
}
```

- `+` Idêntico a `command("nome", ...) { }` e a `button("id") { }`: uma forma só para
  tudo que fica no topo do arquivo.
- `+` O nome entre aspas deixa claro que é uma definição, não uma chamada.
- `-` O nome entre aspas é estranho na hora de chamar (`fn("giveCoins")`, mas
  `giveCoins(...)`): o iniciante pode tentar chamar `"giveCoins"(...)`.
- `-` Diferente do esboço que já está na SPEC e do que quem vem de JS/Lua espera.
- `-` Nomes com espaço ou acento no texto precisam de regra extra (comandos já
  exigem "uma palavra só").

---

## Comparação

| | A sem tipos | B tipos opcionais | C `fn("nome")` |
|---|---|---|---|
| Fácil para iniciante | ★★★ | ★★★ (tipos podem ficar de fora) | ★★ |
| Igual ao resto da linguagem | ★★ | ★★★ (igual aos args de `command`) | ★★★ (igual ao cabeçalho de `command`) |
| Erros perto do engano | ★ | ★★★ | ★★★ |
| Bom para docs e IA (`llm.txt`) | ★ | ★★★ | ★★★ |
| Segue a SPEC §3.5 como está | sim | não (precisa ajustar) | não (precisa ajustar) |
| Trabalho para implementar | pequeno | pequeno (reaproveita a conversão de `command`) | pequeno |

## Recomendação

**B.** A escrita é a mesma dos argumentos de `command`, que o usuário já conhece, e
os tipos são opcionais: o iniciante escreve `fn greet(name) { }` como na proposta A,
e quem quer erros melhores põe `name: text`. Precisa só ajustar a SPEC §3.5 para
"tipos existem nos argumentos de comandos e (opcionais) de funções".

## Perguntas para o dono

1. Qual proposta? (A / B / C)
2. A palavra é `fn` (como no esboço da SPEC) ou `function`?
3. As 9 regras comuns acima estão boas? Em especial:
   - a 4: a função **não** enxerga os `local` de quem chamou;
   - a 6: `stop` dentro da função encerra o comando inteiro.
4. Funções podem ser guardadas em variáveis e passadas como valor
   (`local f = giveCoins`)? Recomendo **não** na v1 (fica mais simples e os erros
   ficam melhores); dá para adicionar depois.
