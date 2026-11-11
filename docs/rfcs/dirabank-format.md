# RFC: formato do arquivo do Dirabank

> Status: **decidido em 2026-11-11**: proposta **A** (log só de acréscimo).
> É o formato oficial `v1`. Arquivos do `v0` provisório têm o mesmo layout e são
> atualizados (só o número da versão) ao abrir.

## O que o formato precisa garantir

- **Um arquivo só**, sem servidor.
- **Pouca RAM** (meta: host de ~100 MB com vários bots).
- **Não corromper** se a máquina desligar no meio de uma gravação.
- Leitura rápida de `get` (é o caso mais comum) e gravação rápida de `set`.
- Simples de implementar sem dependências.

Todas as propostas usam a mesma chave (`escopo + id do dono + nome`) e a mesma
codificação binária de valores (`null`, bool, número, texto, lista, objeto).

---

## Proposta A — Log só de acréscimo + índice em memória (escolhida, formato v1)

Cada `set`/`delete` vira um registro no fim do arquivo:
`crc32 | op | tamanho da chave | tamanho do valor | chave | valor`.
Ao abrir, o arquivo é lido uma vez e monta um índice `chave → posição`.
Os **valores ficam no disco**; só o índice fica na RAM.
Quando mais da metade do arquivo é lixo (versões antigas), ele é reescrito
num arquivo temporário e trocado de uma vez (`rename`, atômico).

| | |
|---|---|
| RAM | só o índice: ~50–80 bytes por chave (100 mil chaves ≈ 6 MB) |
| `set` | muito rápido (uma escrita no fim) |
| `get` | uma leitura no disco (o SO costuma ter em cache) |
| Queda de energia | o último registro pela metade falha no CRC e é cortado |
| Contras | abrir um arquivo grande precisa ler tudo; compactação usa espaço extra temporário |

## Proposta B — Páginas fixas com árvore B (estilo SQLite simplificado)

O arquivo é dividido em páginas de 4 KB. Uma árvore B ordena as chaves.
Gravações usam *copy-on-write* das páginas e trocam a raiz no fim.

| | |
|---|---|
| RAM | quase nada além de um cache pequeno de páginas (configurável) |
| `set` | mais lento (reescreve algumas páginas) |
| `get` | O(log n) leituras de página |
| Queda de energia | segura por causa do copy-on-write |
| Contras | **bem mais código** e mais difícil de testar; ganha só com milhões de chaves |

## Proposta C — Fotografia periódica + diário (snapshot + journal no mesmo arquivo)

Tudo fica na RAM num `HashMap`. As mudanças vão para um diário no fim do
arquivo; de tempos em tempos (ou ao fechar), o estado inteiro é gravado como
uma fotografia nova e o diário é zerado.

| | |
|---|---|
| RAM | **todos os valores** na memória (100 mil chaves pequenas ≈ 15–30 MB) |
| `set` | rápido |
| `get` | o mais rápido (só memória) |
| Queda de energia | perde no máximo o que não foi para o diário |
| Contras | RAM cresce com os dados; ruim para a meta de 100 MB com muitos bots |

---

## Recomendação

**A** — é a que já está implementada e testada (recuperação de queda, compactação,
reabertura). Equilibra RAM baixa e simplicidade. B só compensa se um bot tiver
milhões de chaves; C gasta RAM demais para a meta do projeto.

## Decisão

**A** vira o formato oficial `v1`. B e C ficam registradas para o caso de um bot
precisar de milhões de chaves no futuro (o driver é plugável).
