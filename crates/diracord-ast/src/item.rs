//! Top-level items: commands, events and interactions.

use crate::{Block, Expr, Ident};
use diracord_diagnostics::{FileId, Span, Spanned};
use std::fmt;

/// One parsed source file.
#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    /// Items in source order.
    pub items: Vec<Item>,
    /// The whole file.
    pub span: Span,
}

impl Program {
    /// All `command(...)` items.
    pub fn commands(&self) -> impl Iterator<Item = &CommandDef> {
        self.items.iter().filter_map(|i| match &i.kind {
            ItemKind::Command(c) => Some(c),
            _ => None,
        })
    }

    /// All event handlers (`memberJoin { }`).
    pub fn events(&self) -> impl Iterator<Item = &EventDef> {
        self.items.iter().filter_map(|i| match &i.kind {
            ItemKind::Event(e) => Some(e),
            _ => None,
        })
    }

    /// All interaction handlers (`button("id") { }`).
    pub fn interactions(&self) -> impl Iterator<Item = &InteractionDef> {
        self.items.iter().filter_map(|i| match &i.kind {
            ItemKind::Interaction(x) => Some(x),
            _ => None,
        })
    }

    /// Finds a command by name.
    pub fn command(&self, name: &str) -> Option<&CommandDef> {
        self.commands().find(|c| c.name.node == name)
    }

    /// Merges several files' programs (a project is many files).
    ///
    /// Each item is stamped with the [`FileId`] given next to its program, so
    /// errors can point at the right file. The merged `span` is meaningless
    /// across files and is left as the default.
    pub fn merge(programs: impl IntoIterator<Item = (FileId, Program)>) -> Program {
        let mut items = Vec::new();
        for (file, p) in programs {
            items.extend(p.items.into_iter().map(|mut i| {
                i.file = file;
                i
            }));
        }
        Program { items, span: Span::default() }
    }
}

/// A top-level item.
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    /// What it is.
    pub kind: ItemKind,
    /// The whole item, header and body.
    pub span: Span,
    /// Which file of the project it came from (the parser sets `FileId(0)`).
    pub file: FileId,
}

/// The kinds of top-level items.
#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind {
    /// `command("ban", user: member) { ... }`
    Command(CommandDef),
    /// `memberJoin { ... }`
    Event(EventDef),
    /// `button("confirm") { ... }`, `menu(...)`, `modal(...)`
    Interaction(InteractionDef),
}

/// A command. The same definition serves prefix and slash commands.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandDef {
    /// Command name (the string literal).
    pub name: Spanned<String>,
    /// Typed arguments, in order.
    pub params: Vec<Param>,
    /// The body.
    pub body: Block,
}

/// A typed command argument: `reason: text = "none"`.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    /// Argument name.
    pub name: Ident,
    /// Declared type.
    pub ty: Spanned<ArgType>,
    /// Default value; `Some` makes the argument optional.
    pub default: Option<Expr>,
    /// The whole `name: type = default`.
    pub span: Span,
}

/// Types allowed in command arguments (the only place with types).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArgType {
    /// Free text.
    Text,
    /// A number.
    Number,
    /// `true` / `false`.
    Bool,
    /// A member of the current server.
    Member,
    /// Any Discord user.
    User,
    /// A channel.
    Channel,
    /// A role.
    Role,
}

impl ArgType {
    /// Every type, in documentation order.
    pub const ALL: [ArgType; 7] = [
        ArgType::Text,
        ArgType::Number,
        ArgType::Bool,
        ArgType::Member,
        ArgType::User,
        ArgType::Channel,
        ArgType::Role,
    ];

    /// The keyword used in source code.
    pub fn as_str(self) -> &'static str {
        match self {
            ArgType::Text => "text",
            ArgType::Number => "number",
            ArgType::Bool => "bool",
            ArgType::Member => "member",
            ArgType::User => "user",
            ArgType::Channel => "channel",
            ArgType::Role => "role",
        }
    }

    /// Parses a type keyword.
    pub fn from_name(name: &str) -> Option<ArgType> {
        ArgType::ALL.into_iter().find(|t| t.as_str() == name)
    }
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An event handler: the event name followed by a block.
#[derive(Clone, Debug, PartialEq)]
pub struct EventDef {
    /// Event name as written (validated against [`crate::known::EVENTS`] by the checker).
    pub name: Ident,
    /// The body.
    pub body: Block,
}

/// Kinds of interaction handlers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractionKind {
    /// `button("id") { }`
    Button,
    /// `menu("id") { }` (select menu)
    Menu,
    /// `modal("id") { }`
    Modal,
}

impl InteractionKind {
    /// The keyword used in source code.
    pub fn as_str(self) -> &'static str {
        match self {
            InteractionKind::Button => "button",
            InteractionKind::Menu => "menu",
            InteractionKind::Modal => "modal",
        }
    }

    /// Parses a keyword.
    pub fn from_name(name: &str) -> Option<InteractionKind> {
        match name {
            "button" => Some(InteractionKind::Button),
            "menu" => Some(InteractionKind::Menu),
            "modal" => Some(InteractionKind::Modal),
            _ => None,
        }
    }
}

/// Code that runs when a component with `id` is used. Linked by id, as in aoi.js.
#[derive(Clone, Debug, PartialEq)]
pub struct InteractionDef {
    /// Button, menu or modal.
    pub kind: InteractionKind,
    /// The custom id (string literal).
    pub id: Spanned<String>,
    /// The body.
    pub body: Block,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_type_roundtrip() {
        for t in ArgType::ALL {
            assert_eq!(ArgType::from_name(t.as_str()), Some(t));
        }
        assert_eq!(ArgType::from_name("string"), None);
    }

    #[test]
    fn interaction_kind_roundtrip() {
        for k in [InteractionKind::Button, InteractionKind::Menu, InteractionKind::Modal] {
            assert_eq!(InteractionKind::from_name(k.as_str()), Some(k));
        }
    }

    #[test]
    fn program_lookup_and_merge() {
        let body = Block { stmts: vec![], span: Span::new(10, 12) };
        let cmd = |name: &str, s: u32| Item {
            kind: ItemKind::Command(CommandDef {
                name: Spanned::new(name.to_string(), Span::new(s, s + 4)),
                params: vec![],
                body: body.clone(),
            }),
            span: Span::new(s, s + 12),
            file: FileId::default(),
        };
        let a = Program { items: vec![cmd("ping", 0)], span: Span::new(0, 12) };
        let b = Program { items: vec![cmd("pong", 0)], span: Span::new(0, 20) };
        let p = Program::merge([(FileId(0), a), (FileId(1), b)]);
        assert_eq!(p.commands().count(), 2);
        assert!(p.command("pong").is_some());
        assert!(p.command("nope").is_none());
        assert_eq!(p.items[1].file, FileId(1));
    }
}
