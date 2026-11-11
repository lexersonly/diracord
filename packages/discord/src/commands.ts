/**
 * Prefix commands (`!ban @Zé spam`) and slash commands (`/ban user: reason:`).
 * Both end up as the same engine trigger.
 * @module
 */

import type { CommandParam, Manifest } from "@diracord/engine";
import { channel as toChannel, member as toMember, role as toRole, user as toUser, type Raw } from "./entities.ts";

/** Splits `a "b c" d` into `["a", "b c", "d"]`. */
export function splitArgs(text: string): string[] {
  const out: string[] = [];
  const re = /"([^"]*)"|'([^']*)'|(\S+)/g;
  for (let m = re.exec(text); m; m = re.exec(text)) out.push(m[1] ?? m[2] ?? m[3] ?? "");
  return out;
}

/** A prefix command found in a message. */
export interface PrefixInvocation {
  name: string;
  /** Raw text per parameter; the last `text` parameter gets the rest of the line. */
  args: Record<string, string>;
}

/**
 * Reads `!name args...` from `content`. Returns `null` if it is not a known command.
 * Command names are matched case-insensitively.
 */
export function parsePrefix(content: string, prefix: string, manifest: Manifest): PrefixInvocation | null {
  if (!content.startsWith(prefix)) return null;
  const rest = content.slice(prefix.length).trimStart();
  const name = rest.split(/\s+/, 1)[0]?.toLowerCase() ?? "";
  const cmd = manifest.commands.find((c) => c.name.toLowerCase() === name);
  if (!cmd) return null;
  const tail = rest.slice(name.length).trim();
  const parts = splitArgs(tail);
  const args: Record<string, string> = {};
  cmd.params.forEach((p, i) => {
    const isLast = i === cmd.params.length - 1;
    if (isLast && p.type === "text" && parts.length > i) {
      // Rest of the line, keeping the original spacing, for reasons and messages.
      let t = tail;
      for (let k = 0; k < i; k++) t = t.slice(t.indexOf(parts[k] as string) + (parts[k] as string).length).replace(/^["']/, "").trimStart();
      args[p.name] = t.replace(/^["'](.*)["']$/s, "$1");
    } else if (parts[i] !== undefined) args[p.name] = parts[i] as string;
  });
  return { name: cmd.name, args };
}

const OPTION_TYPES: Record<CommandParam["type"], number> = { text: 3, number: 10, bool: 5, member: 6, user: 6, channel: 7, role: 8 };

/** Discord's rule for slash command and option names. */
export function validSlashName(name: string): boolean {
  return /^[-_\p{Ll}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$/u.test(name);
}

/** Application command JSON for a bulk overwrite. Invalid names are skipped (and reported). */
export function slashCommands(manifest: Manifest): { commands: Raw[]; skipped: string[] } {
  const commands: Raw[] = [];
  const skipped: string[] = [];
  for (const c of manifest.commands) {
    if (!validSlashName(c.name) || !c.params.every((p) => validSlashName(p.name))) {
      skipped.push(c.name);
      continue;
    }
    commands.push({
      type: 1,
      name: c.name,
      description: `Comando ${c.name}`,
      options: c.params.map((p) => ({ type: OPTION_TYPES[p.type], name: p.name, description: p.name, required: !p.optional })),
    });
  }
  return { commands, skipped };
}

/** Engine arguments from a slash command interaction, with entities resolved. */
export function slashArgs(interaction: Raw): Record<string, unknown> {
  const data = interaction["data"] ?? {};
  const resolved = data["resolved"] ?? {};
  const out: Record<string, unknown> = {};
  for (const o of (data["options"] ?? []) as Raw[]) {
    const v = o["value"];
    switch (o["type"]) {
      case 6: {
        const u = resolved["users"]?.[v];
        const m = resolved["members"]?.[v];
        out[o["name"]] = m && u ? toMember(m, u) : u ? toUser(u) : String(v);
        break;
      }
      case 7:
        out[o["name"]] = resolved["channels"]?.[v] ? toChannel(resolved["channels"][v]) : String(v);
        break;
      case 8:
        out[o["name"]] = resolved["roles"]?.[v] ? toRole(resolved["roles"][v]) : String(v);
        break;
      default:
        out[o["name"]] = v;
    }
  }
  return out;
}
