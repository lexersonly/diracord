/**
 * `diracord`: the main package. Re-exports the building blocks so a bot can
 * also be started from JavaScript.
 * @module
 */
export { main } from "@diracord/cli";
export { loadProject, getToken, type DiracordConfig, type Project } from "@diracord/config";
export { EngineClient, findEngine } from "@diracord/engine";
export { Bot } from "@diracord/discord";
