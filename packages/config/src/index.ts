/**
 * `@diracord/config`: reads `diracord.toml`, `.env` and the project's source files.
 * @module
 */
export { parseToml, TomlError, type TomlTable, type TomlValue } from "./toml.ts";
export { parseEnv, redact } from "./env.ts";
export {
  ConfigError,
  findRoot,
  getToken,
  listSources,
  loadProject,
  readConfig,
  SOURCE_EXTENSIONS,
  type DiracordConfig,
  type Intent,
  type Project,
  type Scope,
} from "./config.ts";
