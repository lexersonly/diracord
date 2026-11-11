//! `diracord-engine`: speaks the JSON-lines protocol on stdin/stdout.
//! Logs and panics go to stderr. See `docs/engine-protocol.md`.

#![forbid(unsafe_code)]

use std::io;

fn main() {
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        Some("--version" | "-V") => {
            println!(
                "diracord-engine {} (protocolo {})",
                env!("CARGO_PKG_VERSION"),
                diracord_engine::server::PROTOCOL
            );
        }
        Some("--help" | "-h") => {
            println!("diracord-engine: motor do Diracord.");
            println!("Fala JSON por linhas no stdin/stdout. Normalmente é iniciado pelo comando `diracord`.");
            println!("Protocolo: docs/engine-protocol.md");
        }
        _ => {
            let stdin = io::stdin();
            let stdout = io::stdout();
            if let Err(e) = diracord_engine::server::serve(stdin.lock(), stdout.lock()) {
                eprintln!("diracord-engine: {e}");
                std::process::exit(1);
            }
        }
    }
}
