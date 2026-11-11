//! Stable error codes.
//!
//! A code never changes meaning once released. Ranges:
//!
//! | Range | Area |
//! |---|---|
//! | `E0001`–`E0099` | lexer (text → tokens) |
//! | `E0100`–`E0199` | parser and static checks |
//! | `E0200`–`E0299` | runtime (interpreter, limits, host) |
//! | `E0300`–`E0399` | function arguments |
//! | `E0400`–`E0499` | data (Dirabank / `get` / `set`) |
//! | `E0500`–`E0599` | project and configuration (CLI) |
//!
//! [`ALL`] lists every code; docs and `llm.txt` are generated from it.

use std::fmt;

/// A stable diagnostic code such as `E0101`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code {
    /// The number (`101` for `E0101`).
    pub number: u16,
    /// Short title in Portuguese, e.g. "chave sem fechar".
    pub title: &'static str,
    /// One-paragraph explanation with the usual fix.
    pub explain: &'static str,
}

impl Code {
    /// The textual form, e.g. `"E0101"`.
    pub fn as_string(&self) -> String {
        format!("E{:04}", self.number)
    }

    /// Looks a code up by its text (`"E0101"`) or number (`"101"`).
    pub fn lookup(text: &str) -> Option<Code> {
        let digits = text.trim().trim_start_matches(['E', 'e']);
        let n: u16 = digits.parse().ok()?;
        ALL.iter().copied().find(|c| c.number == n)
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{:04}", self.number)
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{:04}", self.number)
    }
}

macro_rules! codes {
    ($( $(#[$m:meta])* $name:ident = $num:literal, $title:literal, $explain:literal; )*) => {
        $(
            $(#[$m])*
            pub const $name: Code = Code { number: $num, title: $title, explain: $explain };
        )*
        /// Every known code, in numeric order.
        pub const ALL: &[Code] = &[$($name),*];
    };
}

codes! {
    // ---- lexer ----
    /// A string was opened with `"` and never closed.
    E0001 = 1, "texto sem fechar",
        "Um texto começou com aspas (\") mas não terminou na mesma linha. Feche com outra aspa.";
    /// A character that is not part of the language.
    E0002 = 2, "caractere inesperado",
        "Esse caractere não faz parte do Diracord. Apague-o ou coloque-o dentro de um texto entre aspas.";
    /// A number literal that cannot be read.
    E0003 = 3, "número inválido",
        "Esse número não pode ser lido. Use dígitos, com ponto para decimais: 15 ou 1.5.";
    /// An unknown `\x` escape inside a string.
    E0004 = 4, "escape inválido no texto",
        "Dentro de texto só valem os escapes \\n, \\t, \\\", \\\\ e \\$.";
    /// `$` inside a string not followed by a name.
    E0005 = 5, "variável vazia no texto",
        "Um $ dentro do texto precisa de um nome logo depois, como \"Oi $user.name\". Para escrever o símbolo, use \\$.";

    // ---- parser / static checks ----
    /// The parser found a token it did not expect.
    E0100 = 100, "símbolo inesperado",
        "O código tem algo fora do lugar. Veja a seta e a dica.";
    /// A `{` was never closed.
    E0101 = 101, "chave sem fechar",
        "Toda chave { aberta precisa de uma } para fechar o bloco.";
    /// A `(` was never closed.
    E0102 = 102, "parêntese sem fechar",
        "Todo parêntese ( aberto precisa de um ) para fechar.";
    /// An expression (value) was expected.
    E0103 = 103, "faltou um valor",
        "Aqui era esperado um valor: um texto, um número, uma variável ou uma chamada.";
    /// A name was expected.
    E0104 = 104, "faltou um nome",
        "Aqui era esperado um nome, como `x` ou `user`.";
    /// A `{ ... }` block was expected.
    E0105 = 105, "faltou o bloco",
        "Aqui era esperado um bloco entre chaves: { ... }.";
    /// The left side of `=` cannot receive a value.
    E0106 = 106, "não dá para atribuir aqui",
        "Só dá para usar = com uma variável (x = 1) ou um campo (obj.campo = 1).";
    /// Unknown type in a command argument.
    E0107 = 107, "tipo de argumento desconhecido",
        "Os tipos de argumento são: text, number, bool, member, user, channel, role.";
    /// `$name[...]` used while the dollar syntax is turned off.
    E0108 = 108, "atalho $ desligado",
        "O atalho $nome[...] está desligado. Ligue em diracord.toml ([syntax] dollar = true) ou escreva nome(...).";
    /// `command` without a name.
    E0109 = 109, "comando sem nome",
        "Todo comando precisa de um nome em texto: command(\"ping\") { ... }.";
    /// A bad `chain` arm.
    E0110 = 110, "caso inválido no chain",
        "Cada caso do chain é `valor => ação`. Use `_ =>` para \"qualquer outro\", e só uma vez, no fim.";
    /// A `[` was never closed.
    E0111 = 111, "colchete sem fechar",
        "Todo colchete [ aberto precisa de um ] para fechar.";
    /// A required command argument comes after an optional one.
    E0112 = 112, "argumento obrigatório depois de opcional",
        "Argumentos com valor padrão (= ...) precisam vir depois dos obrigatórios.";
    /// An event name that does not exist.
    E0113 = 113, "evento desconhecido",
        "Esse nome não é um evento conhecido. Veja a lista de eventos na documentação.";
    /// Code outside any command, event or interaction.
    E0114 = 114, "código solto",
        "Todo código precisa estar dentro de um command, de um evento (como memberJoin) ou de um button/menu/modal.";
    /// `break` or `continue` outside a loop.
    E0115 = 115, "break/continue fora de laço",
        "break e continue só funcionam dentro de for ou while.";
    /// Two commands (or handlers) with the same name.
    E0116 = 116, "nome repetido",
        "Já existe outro comando ou interação com esse nome. Renomeie um deles.";

    // ---- runtime ----
    /// A variable that does not exist.
    E0200 = 200, "variável não existe",
        "Essa variável não foi criada. Crie com `local nome = valor` antes de usar.";
    /// A function that does not exist.
    E0201 = 201, "função não existe",
        "Essa função não existe. Veja a dica: pode ser um erro de digitação.";
    /// An operation got the wrong kind of value.
    E0202 = 202, "tipo errado",
        "Essa operação não funciona com esse tipo de valor (por exemplo, número menos texto).";
    /// Division (or remainder) by zero.
    E0203 = 203, "divisão por zero",
        "Não dá para dividir por zero. Confira o valor antes com if.";
    /// The time limit for one execution was reached.
    E0204 = 204, "tempo esgotado",
        "O código demorou demais e foi parado. Procure laços que nunca terminam.";
    /// The step limit for one execution was reached.
    E0205 = 205, "passos demais",
        "O código executou passos demais e foi parado. Procure laços infinitos (while sem fim).";
    /// The memory limit for one execution was reached.
    E0206 = 206, "memória esgotada",
        "O código usou memória demais (listas ou textos muito grandes) e foi parado.";
    /// Something that is not a function was called.
    E0207 = 207, "isso não é uma função",
        "Só dá para chamar funções. Talvez falte um operador entre os valores.";
    /// The host (Discord side) failed to perform an action.
    E0208 = 208, "ação no Discord falhou",
        "O Discord recusou a ação (falta de permissão, usuário saiu, etc.). Use try/catch para tratar.";
    /// A field or index that does not exist.
    E0209 = 209, "campo ou posição não existe",
        "Esse campo (ou posição da lista) não existe nesse valor.";
    /// An error that nobody caught.
    E0210 = 210, "erro não tratado",
        "Aconteceu um erro e nenhum try/catch o tratou.";
    /// Too much nesting.
    E0211 = 211, "aninhamento demais",
        "O código entrou em blocos demais um dentro do outro.";

    // ---- function arguments ----
    /// A required argument is missing.
    E0300 = 300, "faltou argumento",
        "Essa função (ou comando) precisa de mais argumentos. Veja quais na dica.";
    /// More arguments than the function accepts.
    E0301 = 301, "argumentos demais",
        "Essa função recebeu mais argumentos do que aceita.";
    /// An argument with the wrong type or value.
    E0302 = 302, "argumento inválido",
        "Um argumento tem o tipo ou o valor errado. Veja a dica.";
    /// A `name: value` argument the function does not know.
    E0303 = 303, "argumento nomeado desconhecido",
        "Essa função não tem argumento com esse nome.";

    // ---- data ----
    /// The data store failed.
    E0400 = 400, "erro no banco de dados",
        "O Dirabank não conseguiu ler ou gravar. Confira o espaço em disco e as permissões da pasta.";
    /// An unknown data scope.
    E0401 = 401, "escopo inválido",
        "Os escopos são: user, server, member e global.";
    /// Scope needs context that is missing (e.g. `server` in a DM).
    E0402 = 402, "escopo sem contexto",
        "Esse escopo precisa de um servidor (ou usuário) e ele não existe aqui, por exemplo numa mensagem privada.";

    // ---- project / config ----
    /// No diracord.toml found.
    E0500 = 500, "diracord.toml não encontrado",
        "Rode o comando dentro da pasta do bot, ou crie um projeto com `diracord new`.";
    /// diracord.toml cannot be read.
    E0501 = 501, "diracord.toml inválido",
        "O arquivo diracord.toml tem um erro. Veja a linha indicada.";
    /// The bot token is missing.
    E0502 = 502, "token não encontrado",
        "Coloque DISCORD_TOKEN=... no arquivo .env. Nunca coloque o token no código.";
    /// No source files in the project.
    E0503 = 503, "nenhum arquivo de código",
        "Não há arquivos .dcs, .dira ou .diracord no projeto.";
    /// The engine binary could not be started.
    E0504 = 504, "motor não encontrado",
        "O motor do Diracord (diracord-engine) não foi encontrado. Rode `cargo build -p diracord-engine` ou defina DIRACORD_ENGINE.";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_with_padding() {
        assert_eq!(E0001.to_string(), "E0001");
        assert_eq!(E0101.as_string(), "E0101");
    }

    #[test]
    fn numbers_are_unique_and_sorted() {
        for w in ALL.windows(2) {
            assert!(w[0].number < w[1].number, "{:?} >= {:?}", w[0], w[1]);
        }
    }

    #[test]
    fn lookup_works() {
        assert_eq!(Code::lookup("E0201"), Some(E0201));
        assert_eq!(Code::lookup("201"), Some(E0201));
        assert_eq!(Code::lookup("E9999"), None);
    }
}
