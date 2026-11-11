/**
 * `@diracord/engine`: starts the Rust engine and speaks its protocol.
 *
 * ```ts
 * const engine = await EngineClient.start();
 * await engine.load([{ path: "ping.dcs", source: 'command("ping") { reply "Pong!" }' }]);
 * await engine.dispatch({ kind: "command", name: "ping" }, {}, async (a) => console.log(a));
 * ```
 * @module
 */
export { EngineClient, EngineError } from "./client.ts";
export { findEngine } from "./find.ts";
export type * from "./types.ts";
