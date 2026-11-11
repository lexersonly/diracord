//! A [`Host`] for tests: records actions and answers with fake data.

use crate::host::{BoxFuture, Host, HostError};
use diracord_value::{Action, Value};
use std::collections::HashMap;

/// Records every action. Replies return a fake message; lookups return a
/// fake entity named after the query; `hasRole` returns `false`.
#[derive(Debug, Default)]
pub struct TestHost {
    actions: Vec<Action>,
    failures: HashMap<String, String>,
    responses: HashMap<String, Value>,
    counter: u64,
}

impl TestHost {
    /// An empty host.
    pub fn new() -> Self {
        Self::default()
    }

    /// Makes every `action` fail with `message`.
    pub fn fail_on(mut self, action: &str, message: &str) -> Self {
        self.failures
            .insert(action.to_string(), message.to_string());
        self
    }

    /// Answers `action` with `value`.
    pub fn respond(mut self, action: &str, value: Value) -> Self {
        self.responses.insert(action.to_string(), value);
        self
    }

    /// Every action, in order.
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Text of every `reply` (embeds are shown as text).
    pub fn replies(&self) -> Vec<String> {
        self.actions
            .iter()
            .filter(|a| a.name == "reply")
            .map(|a| a.arg(0).to_string())
            .collect()
    }

    fn answer(&mut self, action: &Action) -> Result<Value, HostError> {
        if let Some(msg) = self.failures.get(&action.name) {
            return Err(HostError::new(msg.clone()));
        }
        if let Some(v) = self.responses.get(&action.name) {
            return Ok(v.clone());
        }
        self.counter += 1;
        Ok(match action.name.as_str() {
            "reply" | "send" | "dm" => Value::object([
                ("id", Value::from(format!("msg-{}", self.counter))),
                (
                    "content",
                    Value::from(
                        action
                            .args
                            .last()
                            .map(|v| v.to_string())
                            .unwrap_or_default(),
                    ),
                ),
            ]),
            "channel" | "member" | "role" => {
                let q = action.arg(0).to_string();
                let mention = match action.name.as_str() {
                    "channel" => format!("<#{q}>"),
                    "role" => format!("<@&{q}>"),
                    _ => format!("<@{q}>"),
                };
                Value::object([
                    ("id", Value::from(q.clone())),
                    ("name", Value::from(q)),
                    ("mention", Value::from(mention)),
                ])
            }
            "hasRole" => Value::Bool(false),
            _ => Value::Null,
        })
    }
}

impl Host for TestHost {
    fn perform(&mut self, action: Action) -> BoxFuture<'_, Result<Value, HostError>> {
        let r = self.answer(&action);
        self.actions.push(action);
        Box::pin(std::future::ready(r))
    }
}
