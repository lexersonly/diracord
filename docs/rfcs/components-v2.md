# RFC: sintaxe do layout de Components v2

> Status: **decidido e revisto em 2026-11-11**. A primeira escolha (A+B, blocos com
> `layout { }`) ficou confusa para iniciantes; o dono pediu o estilo do discord.js/aoi.js.
> Decisão final: **peças estilo discord.js** (proposta D abaixo) + forma objeto (B).
> Guia: [docs/language.md](../language.md#components-v2).

## Proposta D — Peças estilo discord.js (escolhida)

Cada peça é uma função comum que devolve um valor; as peças entram umas nas outras,
como `new ContainerBuilder().addTextDisplayComponents(...)` no discord.js, mas sem
`new` nem `.add...`:

```text
reply container(
  text("## Welcome, $user.name"),
  section("Read the rules first.", thumbnail: user.avatar),
  separator(),
  row(button("accept", "Accept", style: "success"),
      button("decline", "Decline", style: "danger")),
  color: "purple"
)
```

- `+` Nada de sintaxe nova: são chamadas de função, que o iniciante já usa (`reply`, `ban`).
- `+` Peças são valores: cabem em variáveis, listas, `if` e `for` normais.
- `+` Erro com seta na peça errada: `button(...)` dentro de `container(...)` → "vai em `row(...)`".
- `-` Muitos parênteses em layouts grandes (resolvido com variáveis e uma peça por linha).

Também foi considerada uma forma estilo aoi.js (`addContainer`, `addText`, `addButton`,
uma por linha, montando a mensagem aos poucos); ficou de fora a pedido do dono.

---

## Contexto

O Components v2 do Discord troca o par "texto + embeds" por uma árvore de componentes,
com a flag `IS_COMPONENTS_V2` na mensagem:

- **Container** (caixa com cor de destaque) contendo os outros;
- **Text Display** (texto em markdown);
- **Section** (até 3 textos + um *accessory*: botão ou miniatura);
- **Thumbnail**, **Media Gallery**, **File**;
- **Separator** (linha/espaço);
- **Action Row** com **botões** e **menus** (selects).

O clique de um botão já tem sintaxe decidida: `button("id") { ... }`, ligado pelo id.
Falta decidir **como escrever o layout** dentro de `reply`/`send`.

O mesmo exemplo nas três propostas: uma caixa roxa com título, um texto ao lado de uma
miniatura, uma linha, e dois botões.

---

## Proposta A — Blocos com palavras (estilo da própria linguagem)

Layout como código: cada componente é uma palavra; quem contém outros usa `{ }`.

```text
reply layout {
  container(color: "purple") {
    text "## Bem-vindo, $user.name"
    section {
      text "Leia as regras antes de continuar."
      thumbnail user.avatar
    }
    separator
    row {
      button("aceitar", "Aceito", style: "success")
      button("recusar", "Não aceito", style: "danger")
    }
  }
}
```

- `+` Parece com o resto do Diracord (blocos `{ }`, chamadas sem parênteses).
- `+` Dá para usar `if`/`for` dentro (`for item in itens { text item }`).
- `+` Erros bons: "`thumbnail` só pode ficar dentro de `section`" com seta.
- `-` Mais palavras novas para aprender (`layout`, `container`, `section`, `row`...).
- `-` Mais trabalho no parser (um modo "layout").

## Proposta B — Objetos (igual ao embed de hoje)

Reaproveita a sintaxe de objeto que já existe para embeds. Sem palavras novas.

```text
reply {
  layout: [
    { container: [
        { text: "## Bem-vindo, $user.name" },
        { section: ["Leia as regras antes de continuar."], thumbnail: user.avatar },
        { separator: true },
        { row: [
            { button: "aceitar", label: "Aceito", style: "success" },
            { button: "recusar", label: "Não aceito", style: "danger" }
        ] }
      ], color: "purple" }
  ]
}
```

- `+` Zero sintaxe nova: só objetos e listas; o parser já aceita hoje.
- `+` Fácil de gerar por código e por IA (parece JSON).
- `-` Muitas chaves e colchetes: difícil para iniciante; fácil esquecer um `]`.
- `-` Não dá para usar `if`/`for` no meio (só montando a lista antes, em variáveis).

## Proposta C — Texto com marcações (estilo markdown)

O layout é um texto, com marcações curtas para caixas, linhas e botões.
Precisa de texto em várias linhas (sintaxe nova: `"""`).

```text
reply layout """
::: container purple
## Bem-vindo, $user.name
Leia as regras antes de continuar. {thumbnail $user.avatar}
---
[Aceito](button:aceitar success) [Não aceito](button:recusar danger)
:::
"""
```

- `+` O mais curto e o mais "visual": parece com a mensagem final.
- `+` Quem já escreve markdown no Discord entende rápido.
- `-` Sintaxe dentro de texto: erros mais difíceis de apontar com precisão.
- `-` Lógica (`if`/`for`) só fora do texto.
- `-` Precisa de strings multilinha, que a linguagem ainda não tem.

---

## Comparação

| | A blocos | B objetos | C texto |
|---|---|---|---|
| Fácil para iniciante | ★★★ | ★ | ★★★ |
| Parecido com o resto da linguagem | ★★★ | ★★ | ★ |
| `if`/`for` dentro do layout | sim | não | não |
| Qualidade das mensagens de erro | ★★★ | ★★ | ★ |
| Trabalho para implementar | médio | pequeno | médio |
| Sintaxe nova | palavras de layout | nenhuma | `"""` + marcações |

## Recomendação

**A**, com **B** aceito como forma "crua" (útil para IAs e para quem vem de JSON):
as duas geram a mesma árvore, e o `check` valida as duas.

## Primeira decisão (substituída pela proposta D)

1. Proposta: **A + B**.
2. Palavra: **`layout`**.
3. Componentes sem bloco aceitam **opções nomeadas**.

Na forma B, a seção usa `section: [textos]` e o acessório vai ao lado
(`thumbnail: url` ou `button: { ... }`); a linha usa `row: [...]`.
