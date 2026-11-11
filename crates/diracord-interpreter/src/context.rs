//! What started a run and the automatic variables it gets.

use diracord_ast::InteractionKind;
use diracord_value::{Map, Value};

/// What should run.
#[derive(Clone, Debug, PartialEq)]
pub enum Trigger {
    /// A command, with its arguments by name (already converted by the host;
    /// text is converted to `number`/`bool` when the parameter asks for it).
    Command {
        /// Command name.
        name: String,
        /// Arguments by parameter name.
        args: Vec<(String, Value)>,
    },
    /// An event; every handler with this name runs, in source order.
    Event {
        /// Event name, e.g. `memberJoin`.
        name: String,
    },
    /// A component interaction.
    Interaction {
        /// Button, menu or modal.
        kind: InteractionKind,
        /// The custom id.
        id: String,
    },
}

impl Trigger {
    /// A command with no arguments.
    pub fn command(name: impl Into<String>) -> Self {
        Trigger::Command {
            name: name.into(),
            args: Vec::new(),
        }
    }

    /// Adds a command argument.
    pub fn with_arg(mut self, name: impl Into<String>, value: Value) -> Self {
        if let Trigger::Command { args, .. } = &mut self {
            args.push((name.into(), value));
        }
        self
    }

    /// An event.
    pub fn event(name: impl Into<String>) -> Self {
        Trigger::Event { name: name.into() }
    }
}

/// Automatic variables of a run (see SPEC 4.9).
///
/// Discord entities are objects with at least `id`; see
/// `docs/engine-protocol.md` for the fields hosts should fill.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Context {
    /// Who triggered it. Also available as `author`.
    pub user: Value,
    /// The server (`null` in DMs).
    pub server: Value,
    /// The channel.
    pub channel: Value,
    /// The message, or the interaction (also available as `interaction`).
    pub message: Value,
    /// `true` when triggered by a slash command or component.
    pub is_interaction: bool,
    /// Extra variables for events (e.g. `reaction` in `reactionAdd`).
    pub extra: Map,
}

impl Context {
    /// The variables this context defines, as (name, value) pairs.
    pub(crate) fn variables(&self) -> Vec<(String, Value)> {
        let mut v = vec![
            ("user".to_string(), self.user.clone()),
            ("author".to_string(), self.user.clone()),
            ("server".to_string(), self.server.clone()),
            ("channel".to_string(), self.channel.clone()),
            ("message".to_string(), self.message.clone()),
        ];
        if self.is_interaction {
            v.push(("interaction".to_string(), self.message.clone()));
        }
        v.extend(self.extra.iter().map(|(k, x)| (k.clone(), x.clone())));
        v
    }

    /// The Discord id of the user, if any.
    pub(crate) fn user_id(&self) -> Option<String> {
        self.user.entity_id()
    }

    /// The Discord id of the server, if any.
    pub(crate) fn server_id(&self) -> Option<String> {
        self.server.entity_id()
    }
}
