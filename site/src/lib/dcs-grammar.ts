/**
 * TextMate grammar for Diracord (`.dcs`), registered in Shiki so code blocks
 * on the site are highlighted like an editor would.
 */
export const dcsGrammar = {
  name: "dcs",
  scopeName: "source.dcs",
  displayName: "Diracord",
  aliases: ["diracord", "dira"],
  patterns: [
    { include: "#comment" },
    { include: "#string" },
    { include: "#dollar" },
    { include: "#items" },
    { include: "#keywords" },
    { include: "#constants" },
    { include: "#numbers" },
    { include: "#types" },
    { include: "#calls" },
    { include: "#named" },
    { include: "#operators" },
  ],
  repository: {
    comment: { name: "comment.line.double-slash.dcs", match: "//.*$" },
    string: {
      name: "string.quoted.double.dcs",
      begin: '"',
      end: '"',
      patterns: [
        { name: "constant.character.escape.dcs", match: "\\\\[nt\"\\\\$]" },
        { name: "variable.other.interpolated.dcs", match: "\\$[A-Za-z_][A-Za-z0-9_]*(?:\\.[A-Za-z_][A-Za-z0-9_]*)*" },
      ],
    },
    dollar: { name: "support.function.dollar.dcs", match: "\\$[A-Za-z_][A-Za-z0-9_]*" },
    items: {
      patterns: [
        {
          match: "^\\s*(command|button|menu|modal)\\s*(?=\\()",
          captures: { 1: { name: "keyword.declaration.dcs" } },
        },
        {
          match: "\\b(fn)\\s+([A-Za-z_][A-Za-z0-9_]*)",
          captures: { 1: { name: "keyword.declaration.dcs" }, 2: { name: "entity.name.function.dcs" } },
        },
        {
          match:
            "^\\s*(ready|messageCreate|messageUpdate|messageDelete|memberJoin|memberLeave|memberUpdate|reactionAdd|reactionRemove|channelCreate|channelDelete|roleCreate|roleDelete|voiceJoin|voiceLeave)\\s*(?=\\{)",
          captures: { 1: { name: "entity.name.type.event.dcs" } },
        },
      ],
    },
    keywords: {
      name: "keyword.control.dcs",
      match: "\\b(local|if|else|for|in|while|chain|testing|catch|return|stop|break|continue|and|or|not)\\b",
    },
    constants: { name: "constant.language.dcs", match: "\\b(true|false|null|_)\\b" },
    numbers: { name: "constant.numeric.dcs", match: "\\b\\d[\\d_]*(?:\\.\\d+)?\\b" },
    types: {
      match: "(:)\\s*(text|number|bool|member|user|channel|role)\\b",
      captures: { 2: { name: "storage.type.dcs" } },
    },
    calls: {
      patterns: [
        { name: "entity.name.function.call.dcs", match: "\\b[A-Za-z_][A-Za-z0-9_]*(?=\\s*\\()" },
        {
          name: "entity.name.function.call.dcs",
          match: "^\\s*[A-Za-z_][A-Za-z0-9_]*(?=\\s+[\"A-Za-z0-9_\\[{$!])",
        },
      ],
    },
    named: { name: "variable.parameter.dcs", match: "\\b[A-Za-z_][A-Za-z0-9_]*(?=\\s*:(?!:))" },
    operators: { name: "keyword.operator.dcs", match: "=>|==|!=|<=|>=|&&|\\|\\||[-+*/%<>=!]" },
  },
} as const;
