//! Text rendering of diagnostics, with the code excerpt and an arrow.
//!
//! Output looks like:
//!
//! ```text
//! erro[E0201]: a função `rpely` não existe
//!  --> commands/ping.dcs:2:3
//!   |
//! 2 |   rpely "Pong!"
//!   |   ^^^^^ função desconhecida
//!   |
//!   = ajuda: você quis dizer `reply`?
//!   = correção automática: trocar `rpely` por `reply`
//! ```

use crate::{Diagnostic, Label, Severity, SourceFile};
use std::fmt::Write;

const RED: &str = "\x1b[1;31m";
const YELLOW: &str = "\x1b[1;33m";
const BLUE: &str = "\x1b[1;34m";
const CYAN: &str = "\x1b[1;36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

struct Palette {
    on: bool,
}

impl Palette {
    fn paint(&self, color: &str, text: &str) -> String {
        if self.on {
            format!("{color}{text}{RESET}")
        } else {
            text.to_string()
        }
    }
}

/// Renders one diagnostic as text. `color` turns ANSI colors on.
pub fn render(diag: &Diagnostic, file: &SourceFile, color: bool) -> String {
    let p = Palette { on: color };
    let mut out = String::new();
    let sev_color = match diag.severity {
        Severity::Error => RED,
        Severity::Warning => YELLOW,
        Severity::Note => CYAN,
    };
    let head = format!("{}[{}]", diag.severity.label(), diag.code);
    let _ = writeln!(out, "{}{}", p.paint(sev_color, &head), p.paint(BOLD, &format!(": {}", diag.message)));

    let mut labels: Vec<&Label> = diag.labels.iter().collect();
    labels.sort_by_key(|l| (l.span.start, !l.primary));
    let lines: Vec<u32> = {
        let mut v: Vec<u32> = labels.iter().map(|l| file.line_col(l.span.start).line).collect();
        v.dedup();
        v
    };
    let gutter = lines.iter().max().map(|n| n.to_string().len()).unwrap_or(1);
    let pad = " ".repeat(gutter);
    let bar = p.paint(BLUE, "|");

    if let Some(span) = diag.primary_span() {
        let lc = file.line_col(span.start);
        let _ = writeln!(out, "{pad}{} {}:{}:{}", p.paint(BLUE, "-->"), file.name(), lc.line, lc.col);
        let _ = writeln!(out, "{pad} {bar}");
        let mut last_line = 0;
        for line in &lines {
            if last_line != 0 && *line > last_line + 1 {
                let _ = writeln!(out, "{}", p.paint(BLUE, "..."));
            }
            last_line = *line;
            let text = file.line_text(*line);
            let num = format!("{line:>gutter$}");
            let _ = writeln!(out, "{} {bar} {}", p.paint(BLUE, &num), text.replace('\t', " "));
            for label in labels.iter().filter(|l| file.line_col(l.span.start).line == *line) {
                let start = file.line_col(label.span.start);
                // Underline up to the end of this line if the span is longer.
                let end_offset = label.span.end.min(file.line_start(*line) + text.len() as u32);
                let end = file.line_col(end_offset.max(label.span.start));
                let width = (end.col.saturating_sub(start.col)).max(1) as usize;
                let mark = if label.primary { "^" } else { "-" };
                let marks = mark.repeat(width);
                let color = if label.primary { sev_color } else { BLUE };
                let caret = format!("{}{}", " ".repeat(start.col as usize - 1), marks);
                let msg = if label.message.is_empty() { String::new() } else { format!(" {}", label.message) };
                let _ = writeln!(out, "{pad} {bar} {}", p.paint(color, &format!("{caret}{msg}")));
            }
        }
        if !diag.help.is_empty() || !diag.notes.is_empty() || !diag.fixes.is_empty() {
            let _ = writeln!(out, "{pad} {bar}");
        }
    } else if !diag.help.is_empty() || !diag.notes.is_empty() || !diag.fixes.is_empty() {
        let _ = writeln!(out, "{pad} {}", p.paint(BLUE, "-->") + " " + file.name());
    }

    let eq = p.paint(BLUE, "=");
    for note in &diag.notes {
        let _ = writeln!(out, "{pad} {eq} {}: {note}", p.paint(BOLD, "nota"));
    }
    for help in &diag.help {
        let _ = writeln!(out, "{pad} {eq} {}: {help}", p.paint(BOLD, "ajuda"));
    }
    for fix in &diag.fixes {
        let kind = if fix.safe { "correção automática" } else { "correção sugerida" };
        let _ = writeln!(out, "{pad} {eq} {}: {}", p.paint(BOLD, kind), fix.message);
    }
    out
}

/// Renders many diagnostics, separated by blank lines, plus a summary line
/// ("2 erros, 1 aviso"). Returns an empty string for an empty slice.
pub fn render_all(diags: &[Diagnostic], file: &SourceFile, color: bool) -> String {
    if diags.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for d in diags {
        out.push_str(&render(d, file, color));
        out.push('\n');
    }
    let errors = diags.iter().filter(|d| d.severity == Severity::Error).count();
    let warnings = diags.iter().filter(|d| d.severity == Severity::Warning).count();
    let _ = writeln!(out, "{}", summary(errors, warnings));
    out
}

fn summary(errors: usize, warnings: usize) -> String {
    let plural = |n: usize, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
    match (errors, warnings) {
        (0, w) => plural(w, "aviso", "avisos"),
        (e, 0) => plural(e, "erro", "erros"),
        (e, w) => format!("{}, {}", plural(e, "erro", "erros"), plural(w, "aviso", "avisos")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{codes, Fix, Span};

    fn sample() -> (SourceFile, Diagnostic) {
        let src = SourceFile::new("commands/ping.dcs", "command(\"ping\") {\n  rpely \"Pong!\"\n}\n");
        let d = Diagnostic::error(codes::E0201, "a função `rpely` não existe")
            .with_label(Span::new(20, 25), "função desconhecida")
            .with_help("você quis dizer `reply`?")
            .with_fix(Fix::replace("trocar `rpely` por `reply`", Span::new(20, 25), "reply", true));
        (src, d)
    }

    #[test]
    fn renders_excerpt_with_arrow() {
        let (src, d) = sample();
        let text = render(&d, &src, false);
        let expected = "\
erro[E0201]: a função `rpely` não existe
 --> commands/ping.dcs:2:3
  |
2 |   rpely \"Pong!\"
  |   ^^^^^ função desconhecida
  |
  = ajuda: você quis dizer `reply`?
  = correção automática: trocar `rpely` por `reply`
";
        assert_eq!(text, expected);
    }

    #[test]
    fn color_adds_escape_codes() {
        let (src, d) = sample();
        assert!(render(&d, &src, true).contains("\x1b["));
    }

    #[test]
    fn empty_span_gets_one_caret() {
        let src = SourceFile::new("a.dcs", "command(\"x\") {");
        let d = Diagnostic::error(codes::E0101, "Faltou fechar a chave").with_label(Span::point(14), "aqui");
        let text = render(&d, &src, false);
        assert!(text.contains("              ^ aqui"), "{text}");
    }

    #[test]
    fn summary_counts() {
        let (src, d) = sample();
        let w = Diagnostic::warning(codes::E0113, "x");
        let text = render_all(&[d.clone(), d, w], &src, false);
        assert!(text.ends_with("2 erros, 1 aviso\n"), "{text}");
        assert_eq!(render_all(&[], &src, false), "");
    }
}
