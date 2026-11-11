/**
 * Gateway intents.
 * @module
 */

/** Intent bits by the names used in `diracord.toml`. */
export const INTENT_BITS = {
  guilds: 1 << 0,
  guildMembers: 1 << 1,
  guildVoiceStates: 1 << 7,
  guildMessages: 1 << 9,
  guildMessageReactions: 1 << 10,
  directMessages: 1 << 12,
  messageContent: 1 << 15,
} as const;

/** Combines intent names into the bitfield sent in IDENTIFY. Unknown names are ignored. */
export function intentBits(names: readonly string[]): number {
  let bits = 0;
  for (const n of names) bits |= (INTENT_BITS as Record<string, number>)[n] ?? 0;
  return bits;
}
