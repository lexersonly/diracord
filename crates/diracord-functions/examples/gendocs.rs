//! Prints `docs/functions.md` from the registry, so docs never drift from code.
//!
//! ```text
//! cargo run -q -p diracord-functions --example gendocs > docs/functions.md
//! ```

use diracord_functions::{registry, Category};

/// Categories in documentation order.
const CATEGORIES: [Category; 9] = [
    Category::Messages,
    Category::Moderation,
    Category::Roles,
    Category::Lookup,
    Category::Data,
    Category::Text,
    Category::Math,
    Category::Lists,
    Category::Utility,
];

/// One line per function, for `llm.txt` (`--llm`).
fn llm() {
    let r = registry();
    for c in CATEGORIES {
        println!("[{c}]");
        for s in r.in_category(c) {
            println!("{} -> {} : {}", s.signature(), s.returns, s.summary);
        }
    }
}

fn main() {
    if std::env::args().any(|a| a == "--llm") {
        return llm();
    }
    let r = registry();
    println!("# Funções embutidas\n");
    println!(
        "> Gerado por `cargo run -q -p diracord-functions --example gendocs`. Não edite à mão.\n"
    );
    println!(
        "Total: {} funções. Todas podem ser chamadas com ou sem parênteses:",
        r.len()
    );
    println!("`reply \"oi\"` é igual a `reply(\"oi\")`. Argumentos com `?` são opcionais e");
    println!("também podem ser passados pelo nome: `get(\"coins\", scope: user)`.\n");
    for c in CATEGORIES {
        println!("## {c}\n");
        for s in r.in_category(c) {
            println!("### `{}`\n", s.signature());
            println!("{}\n", s.summary);
            if !s.params.is_empty() {
                println!("| argumento | tipo | descrição |");
                println!("|---|---|---|");
                for p in s.params {
                    let opt = if p.optional { " (opcional)" } else { "" };
                    let kind = p.kind.as_str().replace('|', "\\|");
                    println!("| `{}` | {}{} | {} |", p.name, kind, opt, p.doc);
                }
                println!();
            }
            println!("Devolve: `{}`\n", s.returns);
            println!("```\n{}\n```\n", s.example);
        }
    }
}
