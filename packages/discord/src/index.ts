/**
 * `@diracord/discord`: Discord Gateway + REST, without discord.js, with a
 * minimal cache. Implements the engine's host side.
 * @module
 */
export { Bot, type BotOptions } from "./bot.ts";
export { DiscordHost, HostError, type RunState, type RunTarget } from "./host.ts";
export { Gateway, type GatewayOptions, type SocketFactory, type SocketLike } from "./gateway.ts";
export { Rest, DiscordError, API, bucketPath, type FetchLike, type RequestOptions } from "./rest.ts";
export { NameCache, idOf } from "./cache.ts";
export { messageBody, embed, color } from "./content.ts";
export { component, emoji, layoutBody, IS_COMPONENTS_V2 } from "./components.ts";
export { parsePrefix, slashArgs, slashCommands, splitArgs, validSlashName, type PrefixInvocation } from "./commands.ts";
export { intentBits, INTENT_BITS } from "./intents.ts";
export * as entities from "./entities.ts";
