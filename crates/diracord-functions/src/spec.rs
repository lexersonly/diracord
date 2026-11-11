//! Static description of a built-in function.

use crate::{Args, Bound, Effect, FnError};
use std::fmt;

/// Groups used for folders and documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    /// Sending and editing messages (`reply`, `send`).
    Messages,
    /// Moderation (`ban`, `kick`, `timeout`).
    Moderation,
    /// Roles (`addRole`, `removeRole`).
    Roles,
    /// Looking things up on Discord (`channel`, `member`, `role`).
    Lookup,
    /// Saved data (`get`, `set`, `delete`).
    Data,
    /// Text helpers (`upper`, `lower`, `len`).
    Text,
    /// Math (`random`, `round`, `min`, `max`).
    Math,
    /// Lists (`push`, `join`, `contains`).
    Lists,
    /// Anything else (`typeOf`, `now`).
    Utility,
}

impl Category {
    /// Folder name, also used as the docs section.
    pub fn as_str(self) -> &'static str {
        match self {
            Category::Messages => "messages",
            Category::Moderation => "moderation",
            Category::Roles => "roles",
            Category::Lookup => "lookup",
            Category::Data => "data",
            Category::Text => "text",
            Category::Math => "math",
            Category::Lists => "lists",
            Category::Utility => "utility",
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// What a parameter accepts. Used for validation and docs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParamKind {
    /// Anything.
    Any,
    /// Text (numbers and bools are converted to text).
    Text,
    /// A number.
    Number,
    /// `true` / `false` (any value is converted by truthiness).
    Bool,
    /// A list.
    List,
    /// An object.
    Object,
    /// Text or an embed object (what `reply` accepts).
    Content,
    /// A member/user: an object with `id`, an id text or a mention.
    User,
    /// A channel: an object with `id`, an id or `#mention`.
    Channel,
    /// A role: an object with `id`, an id or `@&mention`.
    Role,
    /// A data scope keyword: `user`, `server`, `member`, `global`.
    Scope,
}

impl ParamKind {
    /// Name shown in docs.
    pub fn as_str(self) -> &'static str {
        match self {
            ParamKind::Any => "any",
            ParamKind::Text => "text",
            ParamKind::Number => "number",
            ParamKind::Bool => "bool",
            ParamKind::List => "list",
            ParamKind::Object => "object",
            ParamKind::Content => "text|embed",
            ParamKind::User => "user",
            ParamKind::Channel => "channel",
            ParamKind::Role => "role",
            ParamKind::Scope => "scope",
        }
    }
}

/// One parameter of a built-in function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParamSpec {
    /// Name (also usable as `name: value`).
    pub name: &'static str,
    /// Accepted kind.
    pub kind: ParamKind,
    /// `true` if it can be left out.
    pub optional: bool,
    /// Short description (Portuguese).
    pub doc: &'static str,
}

impl ParamSpec {
    /// A required parameter.
    pub const fn required(name: &'static str, kind: ParamKind, doc: &'static str) -> Self {
        ParamSpec { name, kind, optional: false, doc }
    }

    /// An optional parameter.
    pub const fn optional(name: &'static str, kind: ParamKind, doc: &'static str) -> Self {
        ParamSpec { name, kind, optional: true, doc }
    }
}

/// Everything about one built-in function. Each function file exports one as `SPEC`.
#[derive(Clone, Copy)]
pub struct FunctionSpec {
    /// Name used in code (`reply`).
    pub name: &'static str,
    /// Category (matches the folder).
    pub category: Category,
    /// One sentence in Portuguese.
    pub summary: &'static str,
    /// Parameters in order.
    pub params: &'static [ParamSpec],
    /// What it returns (docs only), e.g. `"message"` or `"text"`.
    pub returns: &'static str,
    /// One line of example code.
    pub example: &'static str,
    /// The implementation. Arguments are already bound to `params` by name/order.
    pub call: fn(&Bound) -> Result<Effect, FnError>,
}

impl FunctionSpec {
    /// Binds raw arguments to this function's parameters and runs it.
    pub fn invoke(&self, args: Args) -> Result<Effect, FnError> {
        let bound = args.bind(self)?;
        (self.call)(&bound)
    }

    /// `name(a, b?)` signature for docs and help messages.
    pub fn signature(&self) -> String {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|p| format!("{}{}: {}", p.name, if p.optional { "?" } else { "" }, p.kind.as_str()))
            .collect();
        format!("{}({})", self.name, params.join(", "))
    }
}

impl fmt::Debug for FunctionSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionSpec").field("name", &self.name).field("category", &self.category).finish()
    }
}
