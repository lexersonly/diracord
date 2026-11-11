//! A project: many source files parsed, merged and checked together.

use diracord_ast::{FileId, Program};
use diracord_diagnostics::{apply_fixes, render, Diagnostic, Severity, SourceFile};
use diracord_parser::{parse, ParseOptions};

/// The result of reading every file of a project.
#[derive(Debug)]
pub struct Analysis {
    /// Source files, indexed by [`FileId`].
    pub files: Vec<SourceFile>,
    /// All files merged into one program (items keep their `FileId`).
    pub program: Program,
    /// Lexer, parser and checker diagnostics with their file, in order.
    pub diagnostics: Vec<(FileId, Diagnostic)>,
}

impl Analysis {
    /// Parses and checks `(path, source)` pairs.
    pub fn new(sources: &[(String, String)], options: &ParseOptions) -> Analysis {
        let mut files = Vec::new();
        let mut programs = Vec::new();
        let mut diagnostics = Vec::new();
        for (i, (path, text)) in sources.iter().enumerate() {
            let id = FileId(i as u32);
            let out = parse(text, options);
            diagnostics.extend(out.diagnostics.into_iter().map(|d| (id, d)));
            programs.push((id, out.program));
            files.push(SourceFile::new(path.clone(), text.clone()));
        }
        let program = Program::merge(programs);
        diagnostics.extend(diracord_interpreter::check(
            &program,
            diracord_functions::registry(),
        ));
        diagnostics.sort_by_key(|(f, d)| (*f, d.primary_span().map(|s| s.start).unwrap_or(0)));
        Analysis {
            files,
            program,
            diagnostics,
        }
    }

    /// Number of errors.
    pub fn errors(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|(_, d)| d.severity == Severity::Error)
            .count()
    }

    /// Number of warnings.
    pub fn warnings(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|(_, d)| d.severity == Severity::Warning)
            .count()
    }

    /// The source file a diagnostic points into.
    pub fn file(&self, id: FileId) -> &SourceFile {
        &self.files[id.0 as usize]
    }

    /// Every diagnostic rendered with excerpts, plus a summary line.
    pub fn render(&self, color: bool) -> String {
        if self.diagnostics.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for (f, d) in &self.diagnostics {
            out.push_str(&render(d, self.file(*f), color));
            out.push('\n');
        }
        out.push_str(&summary(self.errors(), self.warnings()));
        out.push('\n');
        out
    }

    /// Applies every safe fix. Returns `(path, new text, changed)` per file.
    pub fn fixed_sources(&self) -> Vec<(String, String, bool)> {
        self.files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let fixes: Vec<_> = self
                    .diagnostics
                    .iter()
                    .filter(|(id, _)| id.0 as usize == i)
                    .flat_map(|(_, d)| d.fixes.iter().cloned())
                    .collect();
                // If two fixes collide, apply none rather than guess.
                let text = apply_fixes(f.text(), &fixes).unwrap_or_else(|_| f.text().to_string());
                let changed = text != f.text();
                (f.name().to_string(), text, changed)
            })
            .collect()
    }
}

/// "2 erros, 1 aviso"
pub fn summary(errors: usize, warnings: usize) -> String {
    let n = |k: usize, one: &str, many: &str| format!("{k} {}", if k == 1 { one } else { many });
    match (errors, warnings) {
        (0, 0) => "nenhum problema encontrado".to_string(),
        (0, w) => n(w, "aviso", "avisos"),
        (e, 0) => n(e, "erro", "erros"),
        (e, w) => format!("{}, {}", n(e, "erro", "erros"), n(w, "aviso", "avisos")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn src(path: &str, text: &str) -> (String, String) {
        (path.to_string(), text.to_string())
    }

    #[test]
    fn clean_project() {
        let a = Analysis::new(
            &[src("ping.dcs", "command(\"ping\") { reply \"Pong!\" }")],
            &ParseOptions::default(),
        );
        assert_eq!(a.errors(), 0);
        assert_eq!(a.render(false), "");
    }

    #[test]
    fn errors_point_at_the_right_file() {
        let a = Analysis::new(
            &[
                src("a.dcs", "command(\"a\") { reply \"a\" }"),
                src("b.dcs", "command(\"b\") {\n  rpely \"b\"\n}"),
            ],
            &ParseOptions::default(),
        );
        assert_eq!(a.errors(), 1);
        let text = a.render(false);
        assert!(text.contains("b.dcs:2:3"), "{text}");
        assert!(text.contains("você quis dizer `reply`?"), "{text}");
        assert!(text.ends_with("1 erro\n"));
    }

    #[test]
    fn safe_fixes() {
        let a = Analysis::new(
            &[src("a.dcs", "memberjoin {\n  let x = 1\n}")],
            &ParseOptions::default(),
        );
        let fixed = a.fixed_sources();
        assert!(fixed[0].2);
        assert_eq!(fixed[0].1, "memberJoin {\n  local x = 1\n}");
    }
}
