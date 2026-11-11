//! The protocol end to end, in memory and through the real binary.

use diracord_engine::{json, server::serve};
use diracord_value::Value;
use std::io::{BufRead, BufReader, Cursor, Write};
use std::process::{Command, Stdio};

fn lines(out: &[u8]) -> Vec<Value> {
    String::from_utf8_lossy(out)
        .lines()
        .map(|l| json::parse(l).unwrap())
        .collect()
}

fn op(v: &Value) -> &str {
    v.field("op").and_then(Value::as_str).unwrap_or("")
}

const PING: &str = r#"{"op":"load","files":[{"path":"commands/ping.dcs","source":"command(\"ping\") {\n  reply \"Pong!\"\n}\n"}]}"#;

#[test]
fn ping_through_the_protocol() {
    let input = format!(
        "{PING}\n{}\n{}\n{}\n",
        r#"{"op":"dispatch","id":1,"trigger":{"kind":"command","name":"ping"},"context":{"user":{"id":"42","name":"Ana"}}}"#,
        r#"{"op":"result","id":1,"seq":1,"ok":true,"value":{"id":"m9"}}"#,
        r#"{"op":"shutdown"}"#
    );
    let mut out = Vec::new();
    serve(Cursor::new(input), &mut out).unwrap();
    let msgs = lines(&out);
    let ops: Vec<&str> = msgs.iter().map(op).collect();
    assert_eq!(ops, ["hello", "loaded", "action", "done", "bye"]);
    let loaded = &msgs[1];
    assert_eq!(loaded.field("ok"), Some(&Value::Bool(true)));
    let cmds = loaded.field("commands").unwrap();
    assert_eq!(json::to_string(cmds), r#"[{"name":"ping","params":[]}]"#);
    let action = msgs[2].field("action").unwrap();
    assert_eq!(
        json::to_string(action),
        r#"{"args":["Pong!"],"name":"reply","options":{}}"#
    );
    assert_eq!(msgs[3].field("ok"), Some(&Value::Bool(true)));
}

#[test]
fn check_reports_clear_errors() {
    let req = r#"{"op":"check","files":[{"path":"commands/ping.dcs","source":"command(\"ping\") {\n  rpely \"Pong!\"\n}\n"}]}"#;
    let mut out = Vec::new();
    serve(Cursor::new(format!("{req}\n")), &mut out).unwrap();
    let msgs = lines(&out);
    let checked = &msgs[1];
    assert_eq!(checked.field("ok"), Some(&Value::Bool(false)));
    let rendered = checked.field("rendered").and_then(Value::as_str).unwrap();
    assert!(rendered.contains("erro[E0201]"), "{rendered}");
    assert!(rendered.contains("commands/ping.dcs:2:3"), "{rendered}");
    assert!(rendered.contains("^^^^^"), "{rendered}");
    assert!(rendered.contains("você quis dizer `reply`?"), "{rendered}");
}

#[test]
fn host_failure_and_runtime_error() {
    let input = format!(
        "{}\n{}\n{}\n",
        r#"{"op":"load","files":[{"path":"a.dcs","source":"command(\"b\") {\n  ban user\n}"}]}"#,
        r#"{"op":"dispatch","id":7,"trigger":{"kind":"command","name":"b"},"context":{"user":{"id":"1"}}}"#,
        r#"{"op":"result","id":7,"seq":1,"ok":false,"error":"Missing Permissions"}"#
    );
    let mut out = Vec::new();
    serve(Cursor::new(input), &mut out).unwrap();
    let msgs = lines(&out);
    let done = msgs.iter().find(|m| op(m) == "done").unwrap();
    assert_eq!(done.field("ok"), Some(&Value::Bool(false)));
    let err = done.field("error").unwrap();
    assert_eq!(err.field("code"), Some(&Value::from("E0208")));
    assert!(err
        .field("rendered")
        .and_then(Value::as_str)
        .unwrap()
        .contains("a.dcs:2:3"));
}

#[test]
fn bad_requests_get_errors() {
    let mut out = Vec::new();
    serve(
        Cursor::new("nope\n{\"op\":\"dispatch\",\"id\":1}\n{\"op\":\"zzz\"}\n"),
        &mut out,
    )
    .unwrap();
    let msgs = lines(&out);
    assert_eq!(msgs.iter().filter(|m| op(m) == "error").count(), 3);
}

#[test]
fn real_binary_runs_ping() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_diracord-engine"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let mut read = || {
        let mut l = String::new();
        stdout.read_line(&mut l).unwrap();
        json::parse(l.trim()).unwrap()
    };
    assert_eq!(op(&read()), "hello");
    writeln!(stdin, "{PING}").unwrap();
    assert_eq!(op(&read()), "loaded");
    writeln!(
        stdin,
        r#"{{"op":"dispatch","id":1,"trigger":{{"kind":"command","name":"ping"}}}}"#
    )
    .unwrap();
    let action = read();
    assert_eq!(op(&action), "action");
    assert_eq!(
        action.field("action").and_then(|a| a.field("args")),
        Some(&Value::List(vec!["Pong!".into()]))
    );
    writeln!(
        stdin,
        r#"{{"op":"result","id":1,"seq":1,"ok":true,"value":null}}"#
    )
    .unwrap();
    assert_eq!(read().field("ok"), Some(&Value::Bool(true)));
    writeln!(stdin, r#"{{"op":"shutdown"}}"#).unwrap();
    assert_eq!(op(&read()), "bye");
    assert!(child.wait().unwrap().success());
}
