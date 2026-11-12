//! Keeps `llm.txt` and `docs/` in sync with the code:
//! every function and error code is listed, and every ```dcs example checks clean.

use diracord_engine::project::Analysis;
use diracord_parser::ParseOptions;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// Code blocks fenced with ```dcs.
fn dcs_blocks(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur: Option<String> = None;
    for line in text.lines() {
        match &mut cur {
            None if line.trim_start().starts_with("```dcs") => cur = Some(String::new()),
            Some(_) if line.trim_start().starts_with("```") => {
                out.push(cur.take().unwrap_or_default())
            }
            Some(b) => {
                b.push_str(line);
                b.push('\n');
            }
            None => {}
        }
    }
    out
}

#[test]
fn llm_txt_lists_every_function_and_code() {
    let llm = read("llm.txt");
    let functions_md = read("docs/functions.md");
    for f in diracord_functions::registry().iter() {
        assert!(
            llm.contains(&f.signature()),
            "llm.txt não tem {}",
            f.signature()
        );
        assert!(
            functions_md.contains(&f.signature()),
            "docs/functions.md desatualizado: falta {}",
            f.signature()
        );
    }
    let errors_md = read("docs/errors.md");
    for c in diracord_diagnostics::codes::ALL {
        let code = c.to_string();
        assert!(
            llm.contains(&format!("{code} {}", c.title)),
            "llm.txt não tem {code}"
        );
        assert!(
            errors_md.contains(&format!("{code} — {}", c.title)),
            "docs/errors.md desatualizado: falta {code}"
        );
    }
    for e in diracord_ast::known::EVENTS {
        assert!(llm.contains(e.name), "llm.txt não tem o evento {}", e.name);
    }
}

#[test]
fn every_dcs_example_in_the_docs_is_valid() {
    let mut files = vec!["llm.txt".to_string(), "README.md".to_string()];
    for dir in ["docs", "site/content/docs"] {
        let Ok(entries) = std::fs::read_dir(root().join(dir)) else {
            continue;
        };
        for entry in entries {
            let p = entry.unwrap().path();
            if p.extension().is_some_and(|x| x == "md") {
                files.push(format!(
                    "{dir}/{}",
                    p.file_name().unwrap().to_string_lossy()
                ));
            }
        }
    }
    let mut checked = 0;
    for f in files {
        if !root().join(&f).exists() {
            continue;
        }
        for (i, block) in dcs_blocks(&read(&f)).into_iter().enumerate() {
            // Blocks that show the aoi.js-style `$name[...]` shortcut run with it turned on.
            let dollar = block.split('$').skip(1).any(|t| {
                let name: String = t.chars().take_while(|c| c.is_alphanumeric()).collect();
                !name.is_empty() && t[name.len()..].starts_with('[')
            });
            let a = Analysis::new(
                &[(format!("{f}#{i}"), block.clone())],
                &ParseOptions { dollar },
            );
            assert!(
                a.diagnostics.is_empty(),
                "exemplo inválido em {f} (bloco {i}):\n{block}\n{}",
                a.render(false)
            );
            checked += 1;
        }
    }
    assert!(checked >= 40, "poucos exemplos verificados: {checked}");
}

#[test]
fn site_copies_of_generated_docs_are_up_to_date() {
    if !root().join("site/public/llm.txt").exists() {
        return;
    }
    assert_eq!(
        read("site/public/llm.txt"),
        read("llm.txt"),
        "site/public/llm.txt desatualizado: rode `npm run sync-docs` em site/"
    );
    let functions = read("site/content/docs/reference-functions.md");
    for f in diracord_functions::registry().iter() {
        assert!(
            functions.contains(&f.signature()),
            "site desatualizado: falta {} (rode npm run sync-docs em site/)",
            f.signature()
        );
    }
    let errors = read("site/content/docs/reference-errors.md");
    for c in diracord_diagnostics::codes::ALL {
        assert!(
            errors.contains(&format!("{c} — {}", c.title)),
            "site desatualizado: falta {c} (rode npm run sync-docs em site/)"
        );
    }
}
