//! Parser tests. A tiny S-expression printer keeps assertions readable.

use crate::{parse, ParseOptions};
use diracord_ast::*;
use diracord_diagnostics::{apply_fixes, codes, Code};

fn sx(e: &Expr) -> String {
    match &e.kind {
        ExprKind::Literal(Literal::Int(i)) => i.to_string(),
        ExprKind::Literal(Literal::Float(f)) => f.to_string(),
        ExprKind::Literal(Literal::Text(s)) => format!("{s:?}"),
        ExprKind::Literal(Literal::Bool(b)) => b.to_string(),
        ExprKind::Literal(Literal::Null) => "null".into(),
        ExprKind::Name(i) => i.name.clone(),
        ExprKind::Template(parts) => {
            let p: Vec<String> = parts
                .iter()
                .map(|p| match p {
                    TemplatePart::Text(s) => format!("{s:?}"),
                    TemplatePart::Expr(e) => sx(e),
                })
                .collect();
            format!("(tpl {})", p.join(" "))
        }
        ExprKind::List(items) => {
            format!("[{}]", items.iter().map(sx).collect::<Vec<_>>().join(" "))
        }
        ExprKind::Object(fields) => {
            let f: Vec<String> = fields
                .iter()
                .map(|f| format!("{}:{}", f.key.name, sx(&f.value)))
                .collect();
            format!("{{{}}}", f.join(" "))
        }
        ExprKind::Unary { op, expr } => format!("({} {})", op.as_str(), sx(expr)),
        ExprKind::Binary { op, lhs, rhs } => format!("({} {} {})", op.as_str(), sx(lhs), sx(rhs)),
        ExprKind::Member { object, field } => format!("{}.{}", sx(object), field.name),
        ExprKind::Index { object, index } => format!("{}[{}]", sx(object), sx(index)),
        ExprKind::Call { callee, args, .. } => {
            let a: Vec<String> = args
                .iter()
                .map(|a| match &a.name {
                    Some(n) => format!("{}:{}", n.name, sx(&a.value)),
                    None => sx(&a.value),
                })
                .collect();
            if a.is_empty() {
                format!("({})", sx(callee))
            } else {
                format!("({} {})", sx(callee), a.join(" "))
            }
        }
    }
}

fn sx_stmt(s: &Stmt) -> String {
    match &s.kind {
        StmtKind::Local { name, value } => format!("(local {} {})", name.name, sx(value)),
        StmtKind::Assign { target, value } => format!("(= {} {})", sx(target), sx(value)),
        StmtKind::Expr(e) => sx(e),
        StmtKind::If {
            branches,
            otherwise,
        } => {
            let mut out = String::from("(if");
            for b in branches {
                out += &format!(" {} {}", sx(&b.cond), sx_block(&b.body));
            }
            if let Some(o) = otherwise {
                out += &format!(" else {}", sx_block(o));
            }
            out + ")"
        }
        StmtKind::For { var, iter, body } => {
            format!("(for {} {} {})", var.name, sx(iter), sx_block(body))
        }
        StmtKind::While { cond, body } => format!("(while {} {})", sx(cond), sx_block(body)),
        StmtKind::Chain { subject, arms } => {
            let a: Vec<String> = arms
                .iter()
                .map(|a| {
                    let p = match &a.pattern.kind {
                        PatternKind::Wildcard => "_".to_string(),
                        PatternKind::Literal(l) => {
                            sx(&Expr::new(ExprKind::Literal(l.clone()), a.pattern.span))
                        }
                    };
                    format!("{p}=>{}", sx_block(&a.body))
                })
                .collect();
            format!("(chain {} {})", sx(subject), a.join(" "))
        }
        StmtKind::Testing {
            body,
            catch_var,
            handler,
        } => format!(
            "(testing {} {} {})",
            sx_block(body),
            catch_var.as_ref().map(|c| c.name.as_str()).unwrap_or("_"),
            sx_block(handler)
        ),
        StmtKind::Return(e) => format!(
            "(return{})",
            e.as_ref()
                .map(|e| format!(" {}", sx(e)))
                .unwrap_or_default()
        ),
        StmtKind::Stop => "stop".into(),
        StmtKind::Break => "break".into(),
        StmtKind::Continue => "continue".into(),
    }
}

fn sx_block(b: &Block) -> String {
    format!(
        "{{{}}}",
        b.stmts.iter().map(sx_stmt).collect::<Vec<_>>().join("; ")
    )
}

/// Parses `body` inside a command and returns its statements as S-expressions.
fn body(src: &str) -> String {
    let full = format!("command(\"t\") {{\n{src}\n}}");
    let out = parse(&full, &ParseOptions { dollar: true });
    assert!(out.diagnostics.is_empty(), "{:#?}", out.diagnostics);
    let ItemKind::Command(c) = &out.program.items[0].kind else {
        panic!()
    };
    c.body
        .stmts
        .iter()
        .map(sx_stmt)
        .collect::<Vec<_>>()
        .join("\n")
}

fn codes_of(src: &str) -> Vec<Code> {
    parse(src, &ParseOptions::default())
        .diagnostics
        .iter()
        .map(|d| d.code)
        .collect()
}

#[test]
fn ping() {
    let out = parse(
        "command(\"ping\") {\n  reply \"Pong!\"\n}\n",
        &ParseOptions::default(),
    );
    assert!(out.diagnostics.is_empty());
    let c = out.program.command("ping").unwrap();
    assert_eq!(sx_block(&c.body), "{(reply \"Pong!\")}");
    assert_eq!(out.program.items[0].span.start, 0);
}

#[test]
fn bare_and_paren_calls_are_equal() {
    assert_eq!(body("reply \"oi\""), body("reply(\"oi\")"));
}

#[test]
fn command_with_typed_params() {
    let src = "command(\"ban\", user: member, reason: text = \"none\") {\n  ban user reason\n  reply \"$user.name foi banido. Motivo: $reason\"\n}";
    let out = parse(src, &ParseOptions::default());
    assert!(out.diagnostics.is_empty(), "{:#?}", out.diagnostics);
    let c = out.program.command("ban").unwrap();
    assert_eq!(c.params.len(), 2);
    assert_eq!(c.params[0].ty.node, ArgType::Member);
    assert!(c.params[0].default.is_none());
    assert_eq!(c.params[1].ty.node, ArgType::Text);
    assert_eq!(sx(c.params[1].default.as_ref().unwrap()), "\"none\"");
    assert_eq!(
        sx_block(&c.body),
        "{(ban user reason); (reply (tpl user.name \" foi banido. Motivo: \" reason))}"
    );
}

#[test]
fn events_and_interactions() {
    let src = "memberJoin {\n  send channel(\"welcome\") \"Bem-vindo, $user.mention!\"\n}\nbutton(\"confirm\") { reply \"Confirmado!\" }";
    let out = parse(src, &ParseOptions::default());
    assert!(out.diagnostics.is_empty(), "{:#?}", out.diagnostics);
    let e = out.program.events().next().unwrap();
    assert_eq!(e.name.name, "memberJoin");
    assert_eq!(
        sx_block(&e.body),
        "{(send (channel \"welcome\") (tpl \"Bem-vindo, \" user.mention \"!\"))}"
    );
    let b = out.program.interactions().next().unwrap();
    assert_eq!(b.kind, InteractionKind::Button);
    assert_eq!(b.id.node, "confirm");
}

#[test]
fn locals_and_assignment() {
    assert_eq!(
        body("local x = 15\nx = x + 1"),
        "(local x 15)\n(= x (+ x 1))"
    );
    assert_eq!(
        body("local c = get \"coins\" scope: user"),
        "(local c (get \"coins\" scope:user))"
    );
    assert_eq!(body("obj.a = 1\nl[0] = 2"), "(= obj.a 1)\n(= l[0] 2)");
}

#[test]
fn precedence() {
    assert_eq!(
        body("local x = 1 + 2 * 3 == 7 and not false || y"),
        "(local x (|| (&& (== (+ 1 (* 2 3)) 7) (! false)) y))"
    );
    assert_eq!(
        body("local x = -a.b[0] - (1 - 2)"),
        "(local x (- (- a.b[0]) (- 1 2)))"
    );
}

#[test]
fn if_else_chain() {
    assert_eq!(
        body("if x > 5 { reply \"a\" } else if x > 2 { reply \"b\" } else { reply \"c\" }"),
        "(if (> x 5) {(reply \"a\")} (> x 2) {(reply \"b\")} else {(reply \"c\")})"
    );
    assert_eq!(
        body("if x {\n  stop\n}\nelse {\n  break\n}"),
        "(if x {stop} else {break})"
    );
}

#[test]
fn loops() {
    assert_eq!(
        body("for item in list { reply item }"),
        "(for item list {(reply item)})"
    );
    assert_eq!(
        body("while i < 3 { i = i + 1\ncontinue }"),
        "(while (< i 3) {(= i (+ i 1)); continue})"
    );
}

#[test]
fn chain_on_one_line() {
    assert_eq!(
        body("chain x { 1 => reply \"um\"   \"a\" => reply \"letra\"   _ => reply \"outro\" }"),
        "(chain x 1=>{(reply \"um\")} \"a\"=>{(reply \"letra\")} _=>{(reply \"outro\")})"
    );
    assert_eq!(
        body("chain x {\n  -1 => { stop }\n  true => stop,\n}"),
        "(chain x -1=>{stop} true=>{stop})"
    );
}

#[test]
fn testing_catch() {
    assert_eq!(
        body("testing { ban user } catch err { reply \"Falhou: $err\" }"),
        "(testing {(ban user)} err {(reply (tpl \"Falhou: \" err))})"
    );
    assert_eq!(
        body("testing { x() } catch { stop }"),
        "(testing {(x)} _ {stop})"
    );
}

#[test]
fn embeds_and_lists() {
    assert_eq!(
        body("reply { title: \"Oi\", color: \"red\" }"),
        "(reply {title:\"Oi\" color:\"red\"})"
    );
    assert_eq!(
        body("reply {\n  title: \"Oi\"\n  \"cor\": 1\n}"),
        "(reply {title:\"Oi\" cor:1})"
    );
    assert_eq!(body("local l = [1, 2, [3]]"), "(local l [1 2 [3]])");
}

#[test]
fn return_forms() {
    assert_eq!(
        body("return\nreturn 1\nreturn upper \"a\""),
        "(return)\n(return 1)\n(return (upper \"a\"))"
    );
}

#[test]
fn bare_args_are_full_expressions() {
    assert_eq!(
        body("reply \"Oi \" + user.name"),
        "(reply (+ \"Oi \" user.name))"
    );
    assert_eq!(
        body("timeout user 60, \"spam\""),
        "(timeout user 60 \"spam\")"
    );
    assert_eq!(body("ban user reason\nx"), "(ban user reason)\nx");
}

#[test]
fn dollar_shortcut_when_enabled() {
    assert_eq!(body("$ban[user;reason]"), "(ban user reason)");
    assert_eq!(body("$ping"), "(ping)");
}

#[test]
fn dollar_shortcut_disabled_gives_safe_fix() {
    let src = "command(\"x\") {\n  $ban[user;\"spam\"]\n}";
    let out = parse(src, &ParseOptions::default());
    assert_eq!(out.diagnostics.len(), 1);
    let d = &out.diagnostics[0];
    assert_eq!(d.code, codes::E0108);
    let fixed = apply_fixes(src, &d.fixes).unwrap();
    assert_eq!(fixed, "command(\"x\") {\n  ban(user, \"spam\")\n}");
    assert!(parse(&fixed, &ParseOptions::default())
        .diagnostics
        .is_empty());
}

#[test]
fn unclosed_brace() {
    let src = "command(\"ping\") {\n  reply \"Pong!\"\n";
    let out = parse(src, &ParseOptions::default());
    assert_eq!(out.diagnostics.len(), 1);
    assert_eq!(out.diagnostics[0].code, codes::E0101);
    assert_eq!(out.diagnostics[0].primary_span().unwrap().start, 16);
    // The command is still in the tree.
    assert!(out.program.command("ping").is_some());
}

#[test]
fn common_mistakes() {
    assert_eq!(codes_of("reply \"oi\""), [codes::E0114]);
    assert_eq!(codes_of("command(ping) { }"), [codes::E0109]);
    assert_eq!(codes_of("command(\"a\", x: string) { }"), [codes::E0107]);
    assert_eq!(
        codes_of("command(\"a\", x: text = 1, y: text) { }"),
        [codes::E0112]
    );
    assert_eq!(codes_of("command(\"a\" { }"), [codes::E0102]);
    assert_eq!(codes_of("command(\"a\") { local = 1 }"), [codes::E0104]);
    assert_eq!(codes_of("command(\"a\") { if { } }"), [codes::E0103]);
    assert_eq!(
        codes_of("command(\"a\") { if x reply \"a\" }"),
        [codes::E0105]
    );
    assert_eq!(codes_of("command(\"a\") { 1 = 2 }"), [codes::E0106]);
    assert_eq!(
        codes_of("command(\"a\") { chain x { _ => stop  1 => stop } }"),
        [codes::E0110]
    );
    assert_eq!(
        codes_of("command(\"a\") { local l = [1, 2 }"),
        [codes::E0111]
    );
    assert_eq!(codes_of("}"), [codes::E0100]);
}

#[test]
fn safe_fixes_for_beginners() {
    let src = "command(\"a\", n: string) {\n  let x = 1\n  if x = 1 { stop }\n}";
    let out = parse(src, &ParseOptions::default());
    assert_eq!(out.diagnostics.len(), 3, "{:#?}", out.diagnostics);
    let fixes: Vec<_> = out
        .diagnostics
        .iter()
        .flat_map(|d| d.fixes.clone())
        .collect();
    let fixed = apply_fixes(src, &fixes).unwrap();
    assert_eq!(
        fixed,
        "command(\"a\", n: text) {\n  local x = 1\n  if x == 1 { stop }\n}"
    );
    assert!(parse(&fixed, &ParseOptions::default())
        .diagnostics
        .is_empty());
}

#[test]
fn recovers_and_reports_several_errors() {
    let src =
        "command(\"a\") {\n  local = 1\n  reply \"ok\"\n  1 = 2\n}\ncommand(\"b\") { reply \"b\" }";
    let out = parse(src, &ParseOptions::default());
    assert_eq!(out.diagnostics.len(), 2, "{:#?}", out.diagnostics);
    assert_eq!(out.program.commands().count(), 2);
    let a = out.program.command("a").unwrap();
    assert!(a.body.stmts.iter().any(|s| sx_stmt(s) == "(reply \"ok\")"));
}

#[test]
fn two_statements_on_one_line_need_semicolon() {
    assert_eq!(
        codes_of("command(\"a\") { local x = 1 local y = 2 }"),
        [codes::E0100]
    );
    assert!(codes_of("command(\"a\") { local x = 1; local y = 2 }").is_empty());
}

#[test]
fn deep_nesting_does_not_overflow() {
    let src = format!(
        "command(\"a\") {{ local x = {}1{} }}",
        "(".repeat(5000),
        ")".repeat(5000)
    );
    let c = codes_of(&src);
    assert!(c.contains(&codes::E0211));
}

#[test]
fn every_node_has_a_span_inside_the_source() {
    let src = "command(\"ban\", user: member) {\n  if user.bot { stop }\n  reply \"x $user\"\n}";
    let out = parse(src, &ParseOptions::default());
    let c = out.program.command("ban").unwrap();
    for s in &c.body.stmts {
        assert!(s.span.end as usize <= src.len());
        assert!(!s.span.is_empty());
    }
    assert_eq!(&src[c.params[0].span.range()], "user: member");
}

#[test]
fn try_is_an_error_with_a_safe_fix() {
    let src = "command(\"a\") {\n  try { ban user } catch err { stop }\n}";
    let out = parse(src, &ParseOptions::default());
    assert_eq!(out.diagnostics.len(), 1);
    assert_eq!(out.diagnostics[0].code, codes::E0100);
    let fixed = apply_fixes(src, &out.diagnostics[0].fixes).unwrap();
    assert_eq!(
        fixed,
        "command(\"a\") {\n  testing { ban user } catch err { stop }\n}"
    );
    assert!(parse(&fixed, &ParseOptions::default())
        .diagnostics
        .is_empty());
    // The tree is still built, so the rest of the file is checked too.
    let ItemKind::Command(c) = &out.program.items[0].kind else {
        panic!()
    };
    assert!(matches!(c.body.stmts[0].kind, StmtKind::Testing { .. }));
}

#[test]
fn user_functions() {
    let src = "fn giveCoins(who: member, amount: number = 1) {\n  return amount\n}\nfn hi(name) { return \"Hi $name\" }";
    let out = parse(src, &ParseOptions::default());
    assert!(out.diagnostics.is_empty(), "{:#?}", out.diagnostics);
    let fns: Vec<_> = out.program.functions().collect();
    assert_eq!(fns.len(), 2);
    assert_eq!(
        fns[0].signature(),
        "giveCoins(who: member, amount?: number)"
    );
    assert_eq!(fns[1].signature(), "hi(name)");
    assert!(fns[1].params[0].ty.is_none());
}

#[test]
fn function_keyword_mistakes() {
    let src = "function hi(name) { return name }";
    let out = parse(src, &ParseOptions::default());
    assert_eq!(out.diagnostics.len(), 1);
    let fixed = apply_fixes(src, &out.diagnostics[0].fixes).unwrap();
    assert_eq!(fixed, "fn hi(name) { return name }");
    assert_eq!(codes_of("fn hi(a = 1, b) { }"), [codes::E0112]);
    assert_eq!(codes_of("fn hi(a: string) { }"), [codes::E0107]);
    assert_eq!(codes_of("fn hi(a, a) { }"), [codes::E0116]);
}
