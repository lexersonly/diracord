//! End-to-end: source → parser → interpreter → TestHost.

use diracord_diagnostics::{codes, Code};
use diracord_interpreter::{
    block_on, check, testing::TestHost, Context, Interpreter, Limits, Options, Outcome,
    RuntimeError, Trigger,
};
use diracord_parser::{parse, ParseOptions};
use diracord_value::{Action, MemoryStore, Scope, Store, StoreKey, Value};
use std::time::Duration;

fn engine(src: &str) -> Interpreter {
    engine_with(src, Options::default())
}

fn engine_with(src: &str, options: Options) -> Interpreter {
    let out = parse(src, &ParseOptions::default());
    assert!(out.diagnostics.is_empty(), "{:#?}", out.diagnostics);
    Interpreter::new(out.program, options)
}

fn ctx() -> Context {
    Context {
        user: Value::object([
            ("id", Value::from("42")),
            ("name", Value::from("Ana")),
            ("mention", Value::from("<@42>")),
        ]),
        server: Value::object([("id", Value::from("7")), ("name", Value::from("Servidor"))]),
        channel: Value::object([("id", Value::from("100")), ("name", Value::from("geral"))]),
        message: Value::object([("id", Value::from("m1")), ("content", Value::from("!ping"))]),
        ..Context::default()
    }
}

fn run(
    e: &Interpreter,
    t: Trigger,
    host: &mut TestHost,
    store: &mut MemoryStore,
) -> Result<Outcome, RuntimeError> {
    block_on(e.run(&t, ctx(), host, store))
}

fn replies(src: &str, t: Trigger) -> Vec<String> {
    let mut host = TestHost::new();
    run(&engine(src), t, &mut host, &mut MemoryStore::new()).unwrap();
    host.replies()
}

fn err_code(src: &str, t: Trigger) -> Code {
    let mut host = TestHost::new();
    run(&engine(src), t, &mut host, &mut MemoryStore::new())
        .unwrap_err()
        .diagnostic
        .code
}

#[test]
fn ping_end_to_end() {
    assert_eq!(
        replies(
            "command(\"ping\") { reply \"Pong!\" }",
            Trigger::command("ping")
        ),
        ["Pong!"]
    );
}

#[test]
fn ban_command_with_args_and_default() {
    let src = r#"command("ban", user: member, reason: text = "no reason") {
  ban user reason
  reply "$user.name foi banido. Motivo: $reason"
}"#;
    let target = Value::object([("id", Value::from("5")), ("name", Value::from("Zé"))]);
    let mut host = TestHost::new();
    run(
        &engine(src),
        Trigger::command("ban").with_arg("user", target),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap();
    assert_eq!(
        host.actions()[0],
        Action::new("ban", vec!["5".into(), "no reason".into()])
    );
    assert_eq!(host.replies(), ["Zé foi banido. Motivo: no reason"]);
}

#[test]
fn missing_required_argument() {
    let src = "command(\"ban\", user: member) { ban user }";
    assert_eq!(err_code(src, Trigger::command("ban")), codes::E0300);
}

#[test]
fn number_args_are_converted_from_text() {
    let src = "command(\"double\", n: number) { reply n * 2 }";
    assert_eq!(
        replies(src, Trigger::command("double").with_arg("n", "21".into())),
        ["42"]
    );
    assert_eq!(
        err_code(src, Trigger::command("double").with_arg("n", "abc".into())),
        codes::E0302
    );
}

#[test]
fn control_flow() {
    let src = r#"command("t") {
  local total = 0
  for i in range(1, 6) {
    if i == 2 { continue }
    if i == 5 { break }
    total = total + i
  }
  local n = 0
  while n < 3 { n = n + 1 }
  chain total { 8 => reply "oito"  _ => reply "outro" }
  chain "a" { 1 => reply "um"   "a" => reply "letra"   _ => reply "outro" }
  if n > 5 { reply "x" } else if n == 3 { reply "três" } else { reply "y" }
}"#;
    assert_eq!(
        replies(src, Trigger::command("t")),
        ["oito", "letra", "três"]
    );
}

#[test]
fn stop_and_return() {
    let src =
        "command(\"t\") {\n  reply \"a\"\n  stop\n  reply \"b\"\n}\ncommand(\"r\") { return 5 }";
    assert_eq!(replies(src, Trigger::command("t")), ["a"]);
    let mut host = TestHost::new();
    let out = run(
        &engine(src),
        Trigger::command("r"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap();
    assert_eq!(out.value, Value::Int(5));
}

#[test]
fn try_catch_handles_host_failure() {
    let src = "command(\"t\") { testing { ban user } catch err { reply \"Falhou: $err\" } }";
    let mut host = TestHost::new().fail_on("ban", "sem permissão");
    run(
        &engine(src),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap();
    assert_eq!(
        host.replies(),
        ["Falhou: O Discord recusou `ban`: sem permissão"]
    );
}

#[test]
fn uncaught_host_failure_is_e0208() {
    let src = "command(\"t\") {\n  ban user\n}";
    let mut host = TestHost::new().fail_on("ban", "sem permissão");
    let e = run(
        &engine(src),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap_err();
    assert_eq!(e.diagnostic.code, codes::E0208);
    assert_eq!(
        &src[e.diagnostic.primary_span().unwrap().range()],
        "ban user"
    );
}

#[test]
fn data_scopes() {
    let src = r#"command("coin") {
  set("coins", get("coins", 0, scope: user) + 1, scope: user)
  set("total", get("total", 0, scope: global) + 1, scope: global)
  reply get("coins", scope: user)
}"#;
    let e = engine(src);
    let mut store = MemoryStore::new();
    let mut host = TestHost::new();
    run(&e, Trigger::command("coin"), &mut host, &mut store).unwrap();
    run(&e, Trigger::command("coin"), &mut host, &mut store).unwrap();
    assert_eq!(host.replies(), ["1", "2"]);
    assert_eq!(
        store
            .get(&StoreKey::new(Scope::User, "42", "coins"))
            .unwrap(),
        Some(Value::Int(2))
    );
    assert_eq!(
        store
            .get(&StoreKey::new(Scope::Global, "", "total"))
            .unwrap(),
        Some(Value::Int(2))
    );
}

#[test]
fn member_scope_combines_server_and_user() {
    let src = "command(\"x\") { set(\"xp\", 10, scope: member) }";
    let mut store = MemoryStore::new();
    run(
        &engine(src),
        Trigger::command("x"),
        &mut TestHost::new(),
        &mut store,
    )
    .unwrap();
    assert_eq!(
        store
            .get(&StoreKey::new(Scope::Member, "7:42", "xp"))
            .unwrap(),
        Some(Value::Int(10))
    );
}

#[test]
fn default_scope_comes_from_options() {
    let src = "command(\"x\") { set(\"a\", 1) }";
    let mut store = MemoryStore::new();
    let opts = Options {
        default_scope: Scope::Server,
        ..Options::default()
    };
    run(
        &engine_with(src, opts),
        Trigger::command("x"),
        &mut TestHost::new(),
        &mut store,
    )
    .unwrap();
    assert_eq!(
        store.get(&StoreKey::new(Scope::Server, "7", "a")).unwrap(),
        Some(Value::Int(1))
    );
}

#[test]
fn events_and_buttons() {
    let src = r#"memberJoin { send channel("welcome") "Bem-vindo, $user.mention!" }
memberJoin { react "👋" }
button("confirm") { reply "Confirmado!" }"#;
    let e = engine(src);
    let mut host = TestHost::new();
    run(
        &e,
        Trigger::event("memberJoin"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap();
    let names: Vec<&str> = host.actions().iter().map(|a| a.name.as_str()).collect();
    assert_eq!(names, ["channel", "send", "react"]);
    assert_eq!(
        host.actions()[1].args,
        vec![Value::from("welcome"), Value::from("Bem-vindo, <@42>!")]
    );
    let t = Trigger::Interaction {
        kind: diracord_ast::InteractionKind::Button,
        id: "confirm".into(),
    };
    run(&e, t, &mut host, &mut MemoryStore::new()).unwrap();
    assert_eq!(host.replies(), ["Confirmado!"]);
}

#[test]
fn embeds_pass_through() {
    let mut host = TestHost::new();
    run(
        &engine("command(\"e\") { reply { title: \"Oi\", color: \"red\" } }"),
        Trigger::command("e"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap();
    assert_eq!(
        host.actions()[0].args[0],
        Value::object([("color", Value::from("red")), ("title", Value::from("Oi"))])
    );
}

#[test]
fn assignments_into_objects_and_lists() {
    let src = "command(\"t\") {\n  local o = { a: 1 }\n  o.b = 2\n  local l = [1, 2]\n  l[1] = 5\n  reply o.a + o.b + l[1]\n}";
    assert_eq!(replies(src, Trigger::command("t")), ["8"]);
}

#[test]
fn runtime_errors() {
    assert_eq!(
        err_code("command(\"t\") { reply 1 / 0 }", Trigger::command("t")),
        codes::E0203
    );
    assert_eq!(
        err_code("command(\"t\") { reply 1 - \"a\" }", Trigger::command("t")),
        codes::E0202
    );
    assert_eq!(
        err_code("command(\"t\") { reply nada }", Trigger::command("t")),
        codes::E0200
    );
    assert_eq!(
        err_code("command(\"t\") { rpely \"x\" }", Trigger::command("t")),
        codes::E0201
    );
    assert_eq!(
        err_code("command(\"t\") { reply user.nmae }", Trigger::command("t")),
        codes::E0209
    );
    assert_eq!(
        err_code("command(\"t\") { reply [1][5] }", Trigger::command("t")),
        codes::E0209
    );
    assert_eq!(
        err_code("command(\"t\") { ban }", Trigger::command("t")),
        codes::E0300
    );
    assert_eq!(
        err_code("command(\"t\") { }", Trigger::command("nope")),
        codes::E0201
    );
}

#[test]
fn unknown_function_suggests() {
    let mut host = TestHost::new();
    let e = run(
        &engine("command(\"t\") { rpely \"x\" }"),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap_err();
    assert!(
        e.diagnostic.help.iter().any(|h| h.contains("`reply`")),
        "{:?}",
        e.diagnostic.help
    );
}

#[test]
fn step_limit_stops_infinite_loop() {
    let e = err_code("command(\"t\") { while true { } }", Trigger::command("t"));
    assert_eq!(e, codes::E0205);
}

#[test]
fn time_limit() {
    let opts = Options {
        limits: Limits {
            max_steps: u64::MAX,
            max_time: Duration::from_millis(30),
            ..Limits::default()
        },
        ..Options::default()
    };
    let mut host = TestHost::new();
    let r = run(
        &engine_with("command(\"t\") { while true { } }", opts),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new(),
    );
    assert_eq!(r.unwrap_err().diagnostic.code, codes::E0204);
}

#[test]
fn memory_limit() {
    let src = "command(\"t\") { local s = \"xxxxxxxxxx\"\n while true { s = s + s } }";
    let opts = Options {
        limits: Limits {
            max_memory: 64 * 1024,
            ..Limits::default()
        },
        ..Options::default()
    };
    let mut host = TestHost::new();
    let r = run(
        &engine_with(src, opts),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new(),
    );
    assert_eq!(r.unwrap_err().diagnostic.code, codes::E0206);
}

#[test]
fn limits_cannot_be_caught() {
    let src = "command(\"t\") { testing { while true { } } catch { reply \"pego\" } }";
    let mut host = TestHost::new();
    assert!(run(
        &engine(src),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new()
    )
    .is_err());
    assert!(host.replies().is_empty());
}

#[test]
fn action_limit() {
    let src = "command(\"spam\") { for i in 100 { reply i } }";
    assert_eq!(err_code(src, Trigger::command("spam")), codes::E0205);
}

#[test]
fn zero_arg_function_by_name() {
    let mut host = TestHost::new();
    run(
        &engine("command(\"d\") {\n  deleteMessage\n}"),
        Trigger::command("d"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap();
    assert_eq!(host.actions()[0].name, "deleteMessage");
}

// ---- static checks ----

fn check_codes(src: &str) -> Vec<Code> {
    let out = parse(src, &ParseOptions::default());
    assert!(out.diagnostics.is_empty(), "{:#?}", out.diagnostics);
    check(&out.program, diracord_functions::registry())
        .into_iter()
        .map(|(_, d)| d.code)
        .collect()
}

#[test]
fn checker_finds_mistakes() {
    assert!(check_codes("command(\"ping\") { reply \"Pong!\" }").is_empty());
    assert!(check_codes(
        "command(\"a\", u: member) { ban u\n local x = 1\n x = x + 1\n for i in [1] { break } }"
    )
    .is_empty());
    assert_eq!(
        check_codes("command(\"a\") { rpely \"x\" }"),
        [codes::E0201]
    );
    assert_eq!(check_codes("command(\"a\") { reply nada }"), [codes::E0200]);
    assert_eq!(check_codes("command(\"a\") { x = 1 }"), [codes::E0200]);
    assert_eq!(check_codes("command(\"a\") { break }"), [codes::E0115]);
    assert_eq!(check_codes("memberjoin { }"), [codes::E0113]);
    assert_eq!(
        check_codes("command(\"a\") { }\ncommand(\"a\") { }"),
        [codes::E0116]
    );
    assert_eq!(check_codes("command(\"a\") { ban }"), [codes::E0300]);
    assert_eq!(
        check_codes("command(\"a\") { upper(\"a\", \"b\") }"),
        [codes::E0301]
    );
    assert_eq!(
        check_codes("command(\"a\") { get(\"x\", scop: user) }"),
        [codes::E0303]
    );
    assert!(check_codes("command(\"a\") { get(\"x\", scope: member) }").is_empty());
}

#[test]
fn checker_fixes() {
    let src = "memberjoin { reply \"oi\" }";
    let out = parse(src, &ParseOptions::default());
    let diags: Vec<_> = check(&out.program, diracord_functions::registry())
        .into_iter()
        .map(|(_, d)| d)
        .collect();
    let fixes: Vec<_> = diags.iter().flat_map(|d| d.fixes.clone()).collect();
    assert_eq!(
        diracord_diagnostics::apply_fixes(src, &fixes).unwrap(),
        "memberJoin { reply \"oi\" }"
    );
}

// ---- Components v2 ----

fn reply_value(src: &str) -> Value {
    let mut host = TestHost::new();
    run(
        &engine(src),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap();
    host.actions()[0].args[0].clone()
}

#[test]
fn layout_block_builds_the_object_form() {
    let a = r###"command("t") {
  local opcoes = ["sim", "talvez"]
  reply layout {
    container(color: "purple") {
      text "## Oi $user.name"
      section {
        text "Leia as regras."
        thumbnail "https://x/a.png" description: "logo"
      }
      separator(spacing: "large")
      if len(opcoes) > 5 {
        text "muitas"
      }
      row {
        for o in opcoes {
          button(o, upper(o))
        }
        button(label: "Site", url: "https://diracord.dev")
      }
    }
  }
}"###;
    let b = r###"command("t") {
  reply {
    layout: [
      { container: [
          { text: "## Oi Ana" },
          { section: ["Leia as regras."], thumbnail: { url: "https://x/a.png", description: "logo" } },
          { separator: true, spacing: "large" },
          { row: [
              { button: "sim", label: "SIM" },
              { button: "talvez", label: "TALVEZ" },
              { button: null, label: "Site", url: "https://diracord.dev" }
          ] }
        ], color: "purple" }
    ]
  }
}"###;
    assert_eq!(reply_value(a), reply_value(b));
}

#[test]
fn invalid_layouts_are_explained() {
    let src = "command(\"t\") { reply { layout: [{ button: \"x\", label: \"X\" }] } }";
    assert_eq!(err_code(src, Trigger::command("t")), codes::E0117);
    let mut host = TestHost::new();
    let e = run(
        &engine(src),
        Trigger::command("t"),
        &mut host,
        &mut MemoryStore::new(),
    )
    .unwrap_err();
    assert!(
        e.diagnostic.message.contains("dentro de `row`"),
        "{}",
        e.diagnostic.message
    );
    let two = "command(\"t\") { reply layout { section { text \"a\"\n thumbnail \"https://a\"\n button(\"b\", \"B\") } } }";
    assert_eq!(err_code(two, Trigger::command("t")), codes::E0117);
}

#[test]
fn checker_checks_layout_arguments() {
    assert!(check_codes(
        "command(\"a\") { reply layout { separator(spacing: \"large\", line: false) } }"
    )
    .is_empty());
    assert_eq!(
        check_codes("command(\"a\") { reply layout { separator(spacng: \"large\") } }"),
        [codes::E0303]
    );
    assert_eq!(
        check_codes("command(\"a\") { reply layout { text } }"),
        [codes::E0300]
    );
    assert_eq!(
        check_codes("command(\"a\") { reply layout { text nada } }"),
        [codes::E0200]
    );
    assert!(check_codes("command(\"a\") { reply layout { row { for i in range(3) { button(toText(i), \"B\") } } } }").is_empty());
}

#[test]
fn guild_is_not_an_alias_but_gets_a_safe_fix() {
    let src =
        "command(\"a\") {\n  reply \"Servidor: $guild.name\"\n  set(\"x\", 1, scope: guild)\n}";
    let out = parse(src, &ParseOptions::default());
    let diags: Vec<_> = check(&out.program, diracord_functions::registry())
        .into_iter()
        .map(|(_, d)| d)
        .collect();
    assert_eq!(diags.len(), 2, "{diags:#?}");
    assert!(diags
        .iter()
        .all(|d| d.code == codes::E0200 && d.help.iter().any(|h| h.contains("`server`"))));
    let fixes: Vec<_> = diags.iter().flat_map(|d| d.fixes.clone()).collect();
    let fixed = diracord_diagnostics::apply_fixes(src, &fixes).unwrap();
    assert_eq!(
        fixed,
        "command(\"a\") {\n  reply \"Servidor: $server.name\"\n  set(\"x\", 1, scope: server)\n}"
    );
    assert!(check_codes(&fixed).is_empty());
    // A variable the author created called `guild` is fine.
    assert!(check_codes("command(\"a\") { local guild = server\n reply guild.name }").is_empty());
}
