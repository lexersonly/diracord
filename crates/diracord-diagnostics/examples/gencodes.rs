//! Prints `docs/errors.md` from the code registry.
//!
//! ```text
//! cargo run -q -p diracord-diagnostics --example gencodes > docs/errors.md
//! ```

use diracord_diagnostics::codes::ALL;

fn main() {
    println!("# Códigos de erro\n");
    println!("> Gerado por `cargo run -q -p diracord-diagnostics --example gencodes`. Não edite à mão.\n");
    println!("Todo erro do Diracord tem um código estável. O código nunca muda de significado.\n");
    let sections = [
        (1, 99, "Texto do código (lexer)"),
        (100, 199, "Estrutura do código (parser e checagem)"),
        (200, 299, "Execução"),
        (300, 399, "Argumentos de funções"),
        (400, 499, "Dados (Dirabank)"),
        (500, 599, "Projeto e configuração"),
    ];
    for (lo, hi, title) in sections {
        println!("## {title}\n");
        for c in ALL.iter().filter(|c| (lo..=hi).contains(&c.number)) {
            println!("### {c} — {}\n\n{}\n", c.title, c.explain);
        }
    }
}
