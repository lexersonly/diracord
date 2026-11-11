//! Names the language knows about: keywords, events and automatic variables.
//!
//! These lists are part of the language contract. The checker uses them for
//! "você quis dizer ...?" suggestions and the docs/`llm.txt` list them.

/// Reserved words. They cannot be used as variable names.
pub const KEYWORDS: &[&str] = &[
    "command", "button", "menu", "modal", "local", "if", "else", "for", "in", "while", "chain",
    "try", "catch", "return", "stop", "break", "continue", "true", "false", "null", "and", "or",
    "not",
];

/// An event the bot can listen to: `memberJoin { ... }`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventInfo {
    /// Name used in code.
    pub name: &'static str,
    /// What triggers it.
    pub doc: &'static str,
}

/// Every known event.
pub const EVENTS: &[EventInfo] = &[
    EventInfo { name: "ready", doc: "o bot ligou e está conectado" },
    EventInfo { name: "messageCreate", doc: "uma mensagem foi enviada" },
    EventInfo { name: "messageUpdate", doc: "uma mensagem foi editada" },
    EventInfo { name: "messageDelete", doc: "uma mensagem foi apagada" },
    EventInfo { name: "memberJoin", doc: "alguém entrou no servidor" },
    EventInfo { name: "memberLeave", doc: "alguém saiu do servidor" },
    EventInfo { name: "memberUpdate", doc: "um membro mudou (cargos, apelido...)" },
    EventInfo { name: "reactionAdd", doc: "alguém reagiu a uma mensagem" },
    EventInfo { name: "reactionRemove", doc: "alguém tirou uma reação" },
    EventInfo { name: "channelCreate", doc: "um canal foi criado" },
    EventInfo { name: "channelDelete", doc: "um canal foi apagado" },
    EventInfo { name: "roleCreate", doc: "um cargo foi criado" },
    EventInfo { name: "roleDelete", doc: "um cargo foi apagado" },
    EventInfo { name: "voiceJoin", doc: "alguém entrou num canal de voz" },
    EventInfo { name: "voiceLeave", doc: "alguém saiu de um canal de voz" },
];

/// `true` if `name` is a known event.
pub fn is_event(name: &str) -> bool {
    EVENTS.iter().any(|e| e.name == name)
}

/// Variables that exist automatically inside commands, events and interactions.
///
/// `author` is an alias of `user`; `interaction` is an alias of `message` when
/// the code was triggered by an interaction.
pub const AUTO_VARS: &[&str] = &["user", "author", "server", "channel", "message", "interaction"];

/// `true` if `name` is a reserved word.
pub fn is_keyword(name: &str) -> bool {
    KEYWORDS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookups() {
        assert!(is_event("memberJoin"));
        assert!(!is_event("memberjoin"));
        assert!(is_keyword("chain"));
        assert!(!is_keyword("reply"));
    }
}
