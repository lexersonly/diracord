/**
 * `.env` reader. Secrets (like the bot token) live here, never in code.
 * @module
 */

/**
 * Parses `KEY=value` lines. Supports `#` comments, `export KEY=...`, and
 * single/double quotes (double quotes understand `\n`).
 */
export function parseEnv(text: string): Record<string, string> {
  const out: Record<string, string> = {};
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const m = /^(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$/.exec(line);
    if (!m) continue;
    const key = m[1] as string;
    let value = (m[2] as string).trim();
    if (value.startsWith('"') && value.endsWith('"') && value.length >= 2) {
      value = value.slice(1, -1).replaceAll("\\n", "\n").replaceAll('\\"', '"');
    } else if (value.startsWith("'") && value.endsWith("'") && value.length >= 2) {
      value = value.slice(1, -1);
    } else {
      const hash = value.indexOf(" #");
      if (hash >= 0) value = value.slice(0, hash).trim();
    }
    out[key] = value;
  }
  return out;
}

/** Hides a secret for logs: `abcd…wxyz` (never print a full token). */
export function redact(secret: string): string {
  if (secret.length <= 8) return "••••";
  return `${secret.slice(0, 4)}…${secret.slice(-4)}`;
}
