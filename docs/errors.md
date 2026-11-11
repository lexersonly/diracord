# Códigos de erro

> Gerado por `cargo run -q -p diracord-diagnostics --example gencodes`. Não edite à mão.

Todo erro do Diracord tem um código estável. O código nunca muda de significado.

## Texto do código (lexer)

### E0001 — texto sem fechar

Um texto começou com aspas (") mas não terminou na mesma linha. Feche com outra aspa.

### E0002 — caractere inesperado

Esse caractere não faz parte do Diracord. Apague-o ou coloque-o dentro de um texto entre aspas.

### E0003 — número inválido

Esse número não pode ser lido. Use dígitos, com ponto para decimais: 15 ou 1.5.

### E0004 — escape inválido no texto

Dentro de texto só valem os escapes \n, \t, \", \\ e \$.

### E0005 — variável vazia no texto

Um $ dentro do texto precisa de um nome logo depois, como "Oi $user.name". Para escrever o símbolo, use \$.

## Estrutura do código (parser e checagem)

### E0100 — símbolo inesperado

O código tem algo fora do lugar. Veja a seta e a dica.

### E0101 — chave sem fechar

Toda chave { aberta precisa de uma } para fechar o bloco.

### E0102 — parêntese sem fechar

Todo parêntese ( aberto precisa de um ) para fechar.

### E0103 — faltou um valor

Aqui era esperado um valor: um texto, um número, uma variável ou uma chamada.

### E0104 — faltou um nome

Aqui era esperado um nome, como `x` ou `user`.

### E0105 — faltou o bloco

Aqui era esperado um bloco entre chaves: { ... }.

### E0106 — não dá para atribuir aqui

Só dá para usar = com uma variável (x = 1) ou um campo (obj.campo = 1).

### E0107 — tipo de argumento desconhecido

Os tipos de argumento são: text, number, bool, member, user, channel, role.

### E0108 — atalho $ desligado

O atalho $nome[...] está desligado. Ligue em diracord.toml ([syntax] dollar = true) ou escreva nome(...).

### E0109 — comando sem nome

Todo comando precisa de um nome em texto: command("ping") { ... }.

### E0110 — caso inválido no chain

Cada caso do chain é `valor => ação`. Use `_ =>` para "qualquer outro", e só uma vez, no fim.

### E0111 — colchete sem fechar

Todo colchete [ aberto precisa de um ] para fechar.

### E0112 — argumento obrigatório depois de opcional

Argumentos com valor padrão (= ...) precisam vir depois dos obrigatórios.

### E0113 — evento desconhecido

Esse nome não é um evento conhecido. Veja a lista de eventos na documentação.

### E0114 — código solto

Todo código precisa estar dentro de um command, de um evento (como memberJoin) ou de um button/menu/modal.

### E0115 — break/continue fora de laço

break e continue só funcionam dentro de for ou while.

### E0116 — nome repetido

Já existe outro comando ou interação com esse nome. Renomeie um deles.

## Execução

### E0200 — variável não existe

Essa variável não foi criada. Crie com `local nome = valor` antes de usar.

### E0201 — função não existe

Essa função não existe. Veja a dica: pode ser um erro de digitação.

### E0202 — tipo errado

Essa operação não funciona com esse tipo de valor (por exemplo, número menos texto).

### E0203 — divisão por zero

Não dá para dividir por zero. Confira o valor antes com if.

### E0204 — tempo esgotado

O código demorou demais e foi parado. Procure laços que nunca terminam.

### E0205 — passos demais

O código executou passos demais e foi parado. Procure laços infinitos (while sem fim).

### E0206 — memória esgotada

O código usou memória demais (listas ou textos muito grandes) e foi parado.

### E0207 — isso não é uma função

Só dá para chamar funções. Talvez falte um operador entre os valores.

### E0208 — ação no Discord falhou

O Discord recusou a ação (falta de permissão, usuário saiu, etc.). Use try/catch para tratar.

### E0209 — campo ou posição não existe

Esse campo (ou posição da lista) não existe nesse valor.

### E0210 — erro não tratado

Aconteceu um erro e nenhum try/catch o tratou.

### E0211 — aninhamento demais

O código entrou em blocos demais um dentro do outro.

## Argumentos de funções

### E0300 — faltou argumento

Essa função (ou comando) precisa de mais argumentos. Veja quais na dica.

### E0301 — argumentos demais

Essa função recebeu mais argumentos do que aceita.

### E0302 — argumento inválido

Um argumento tem o tipo ou o valor errado. Veja a dica.

### E0303 — argumento nomeado desconhecido

Essa função não tem argumento com esse nome.

## Dados (Dirabank)

### E0400 — erro no banco de dados

O Dirabank não conseguiu ler ou gravar. Confira o espaço em disco e as permissões da pasta.

### E0401 — escopo inválido

Os escopos são: user, server, member e global.

### E0402 — escopo sem contexto

Esse escopo precisa de um servidor (ou usuário) e ele não existe aqui, por exemplo numa mensagem privada.

## Projeto e configuração

### E0500 — diracord.toml não encontrado

Rode o comando dentro da pasta do bot, ou crie um projeto com `diracord new`.

### E0501 — diracord.toml inválido

O arquivo diracord.toml tem um erro. Veja a linha indicada.

### E0502 — token não encontrado

Coloque DISCORD_TOKEN=... no arquivo .env. Nunca coloque o token no código.

### E0503 — nenhum arquivo de código

Não há arquivos .dcs, .dira ou .diracord no projeto.

### E0504 — motor não encontrado

O motor do Diracord (diracord-engine) não foi encontrado. Rode `cargo build -p diracord-engine` ou defina DIRACORD_ENGINE.

