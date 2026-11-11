/**
 * Converts Discord API objects into the small objects Diracord code sees
 * (`user.name`, `channel.mention`...). Only useful fields are kept.
 * @module
 */

/** Raw Discord objects are loosely typed here on purpose (we read few fields). */
export type Raw = Record<string, any>;

/** `user` / `author` */
export function user(u: Raw): Raw {
  return {
    id: String(u["id"]),
    name: u["global_name"] ?? u["username"] ?? "",
    username: u["username"] ?? "",
    mention: `<@${u["id"]}>`,
    bot: Boolean(u["bot"]),
    avatar: u["avatar"] ? `https://cdn.discordapp.com/avatars/${u["id"]}/${u["avatar"]}.png` : null,
  };
}

/** A member: a user plus server-specific fields. */
export function member(m: Raw, u: Raw | undefined = m["user"]): Raw {
  const base = u ? user(u) : { id: "", name: "", username: "", mention: "", bot: false, avatar: null };
  return {
    ...base,
    name: m["nick"] ?? base.name,
    nick: m["nick"] ?? null,
    roles: Array.isArray(m["roles"]) ? m["roles"].map(String) : [],
    joinedAt: m["joined_at"] ?? null,
  };
}

const CHANNEL_TYPES: Record<number, string> = { 0: "text", 1: "dm", 2: "voice", 4: "category", 5: "news", 13: "stage", 15: "forum" };

/** `channel` */
export function channel(c: Raw): Raw {
  return {
    id: String(c["id"]),
    name: c["name"] ?? "",
    mention: `<#${c["id"]}>`,
    type: CHANNEL_TYPES[c["type"] as number] ?? "other",
  };
}

/** A role. */
export function role(r: Raw): Raw {
  return { id: String(r["id"]), name: r["name"] ?? "", mention: `<@&${r["id"]}>`, color: r["color"] ?? 0 };
}

/** `server` */
export function server(g: Raw): Raw {
  return { id: String(g["id"]), name: g["name"] ?? "", memberCount: g["member_count"] ?? g["approximate_member_count"] ?? null };
}

/** `message` */
export function message(m: Raw): Raw {
  return {
    id: String(m["id"]),
    content: m["content"] ?? "",
    channelId: String(m["channel_id"] ?? ""),
    authorId: m["author"] ? String(m["author"]["id"]) : null,
  };
}
