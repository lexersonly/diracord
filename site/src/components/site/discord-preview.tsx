import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

/**
 * A simulated Discord conversation, used across the docs to show what the
 * code does. Layouts use the same object form as the language
 * (`{ container: [...], color }`, `{ row: [{ button, label }] }`...).
 */
export interface PreviewData {
  messages: PreviewMessage[];
}

export interface PreviewMessage {
  from: "user" | "bot";
  /** Display name (default: "Ana" for users, "My Bot" for the bot). */
  name?: string;
  text?: string;
  /** Shown above a bot reply: "Ana usou /ban". */
  command?: string;
  ephemeral?: boolean;
  embed?: Obj;
  layout?: Obj[];
  /** Reactions under the message. */
  reactions?: string[];
}

type Obj = Record<string, unknown>;

const COLORS: Record<string, string> = {
  red: "#ed4245",
  green: "#57f287",
  blue: "#3498db",
  blurple: "#5865f2",
  yellow: "#fee75c",
  orange: "#e67e22",
  purple: "#9b59b6",
  pink: "#eb459e",
  white: "#ffffff",
  black: "#23272a",
  gray: "#95a5a6",
};

const BUTTON: Record<string, string> = {
  primary: "bg-[#5865f2] text-white",
  secondary: "bg-[#4e5058] text-white",
  success: "bg-[#248046] text-white",
  danger: "bg-[#da373c] text-white",
  link: "bg-[#4e5058] text-white",
};

const color = (c: unknown) => (typeof c === "string" ? (COLORS[c] ?? (c.startsWith("#") ? c : undefined)) : undefined);

/** Tiny Discord-flavoured markdown: **bold**, `code`, @mentions and #channels. */
function inline(text: string): ReactNode[] {
  return text.split(/(\*\*[^*]+\*\*|`[^`]+`|<?[@#][\w-]+>?)/g).map((part, i) => {
    if (part.startsWith("**")) return <strong key={i} className="font-semibold text-white">{part.slice(2, -2)}</strong>;
    if (part.startsWith("`")) return <code key={i} className="rounded bg-[#1e1f22] px-1 py-0.5 font-mono text-[0.85em]">{part.slice(1, -1)}</code>;
    if (/^<?[@#]/.test(part)) return <span key={i} className="rounded bg-[#5865f2]/30 px-0.5 font-medium text-[#c9cdfb]">{part.replace(/[<>]/g, "")}</span>;
    return part;
  });
}

function Markdown({ text }: { text: string }) {
  return (
    <>
      {text.split("\n").map((line, i) => {
        if (line.startsWith("## ")) return <p key={i} className="text-[17px] font-bold leading-snug text-white">{inline(line.slice(3))}</p>;
        if (line.startsWith("# ")) return <p key={i} className="text-xl font-bold leading-snug text-white">{inline(line.slice(2))}</p>;
        if (line.startsWith("-# ")) return <p key={i} className="text-xs text-[#949ba4]">{inline(line.slice(3))}</p>;
        return <p key={i} className="min-h-[1.375em]">{inline(line)}</p>;
      })}
    </>
  );
}

/** A soft gradient tile standing in for an image. */
function Picture({ className, label }: { className?: string; label?: string }) {
  return (
    <div
      className={cn("flex items-end rounded-lg bg-gradient-to-br from-[#5865f2] via-[#9b59b6] to-[#eb459e] p-1.5", className)}
      role="img"
      aria-label={label ?? "imagem"}
    >
      {label && <span className="truncate rounded bg-black/40 px-1 text-[10px] text-white/90">{label}</span>}
    </div>
  );
}

function Button({ b }: { b: Obj }) {
  const isLink = b.url != null;
  const style = isLink ? "link" : String(b.style ?? "primary");
  return (
    <span
      className={cn(
        "inline-flex h-8 items-center gap-1.5 rounded-lg px-3.5 text-sm font-medium",
        BUTTON[style] ?? BUTTON.primary,
        b.disabled ? "opacity-50" : null,
      )}
    >
      {b.emoji ? <span>{String(b.emoji)}</span> : null}
      {b.label ? String(b.label) : null}
      {isLink && <span className="text-xs opacity-80">↗</span>}
    </span>
  );
}

function Menu({ m }: { m: Obj }) {
  const options = (m.options as unknown[] | undefined) ?? [];
  const first = options[0];
  const label = m.placeholder ?? (typeof first === "object" && first ? (first as Obj).label : first) ?? "";
  return (
    <div className="flex h-10 w-full max-w-sm items-center justify-between rounded-lg border border-white/10 bg-[#1e1f22] px-3 text-sm text-[#949ba4]">
      <span className="truncate">{String(label)}</span>
      <svg viewBox="0 0 16 16" className="size-4 shrink-0" aria-hidden>
        <path d="M4 6l4 4 4-4" fill="none" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
      </svg>
    </div>
  );
}

function Piece({ piece }: { piece: Obj }): ReactNode {
  if ("container" in piece) {
    const accent = color(piece.color);
    return (
      <div
        className="space-y-2.5 rounded-lg border border-white/[0.07] bg-[#2b2d31] p-3.5"
        style={accent ? { borderLeft: `4px solid ${accent}` } : undefined}
      >
        {(piece.container as Obj[]).map((p, i) => (
          <Piece key={i} piece={p} />
        ))}
      </div>
    );
  }
  if ("section" in piece) {
    return (
      <div className="flex items-start justify-between gap-4">
        <div className="space-y-1">
          {(piece.section as unknown[]).map((t, i) => (
            <Markdown key={i} text={String(typeof t === "object" && t ? (t as Obj).text : t)} />
          ))}
        </div>
        {piece.thumbnail != null ? <Picture className="size-16 shrink-0" /> : <Button b={piece.button as Obj} />}
      </div>
    );
  }
  if ("row" in piece) {
    return (
      <div className="flex flex-wrap gap-2">
        {(piece.row as Obj[]).map((x, i) => ("menu" in x ? <Menu key={i} m={x} /> : <Button key={i} b={x} />))}
      </div>
    );
  }
  if ("text" in piece) return <Markdown text={String(piece.text)} />;
  if ("separator" in piece) {
    return (
      <div className={piece.spacing === "large" ? "py-2" : "py-0.5"}>
        {piece.line === false ? null : <hr className="border-white/10" />}
      </div>
    );
  }
  if ("gallery" in piece) {
    const items = piece.gallery as unknown[];
    return (
      <div className={cn("grid max-w-md gap-1", items.length > 1 ? "grid-cols-2" : "grid-cols-1")}>
        {items.map((_, i) => (
          <Picture key={i} className={items.length === 1 ? "h-40" : "h-24"} />
        ))}
      </div>
    );
  }
  if ("file" in piece) {
    return (
      <div className="flex max-w-sm items-center gap-3 rounded-lg border border-white/10 bg-[#2b2d31] p-3">
        <span className="flex size-9 items-center justify-center rounded bg-[#5865f2]/30 text-xs font-bold text-[#c9cdfb]">FILE</span>
        <span className="truncate text-sm text-[#00a8fc]">{String(piece.file).replace("attachment://", "")}</span>
      </div>
    );
  }
  return null;
}

function Embed({ e }: { e: Obj }) {
  const fields = (e.fields as Obj[] | undefined) ?? [];
  return (
    <div className="flex max-w-md gap-4 rounded border-l-4 bg-[#2b2d31] p-3.5" style={{ borderColor: color(e.color) ?? "#1e1f22" }}>
      <div className="min-w-0 flex-1 space-y-1.5">
        {e.author != null && <p className="text-xs font-semibold text-white">{String(e.author)}</p>}
        {e.title != null && <p className="font-semibold text-white">{String(e.title)}</p>}
        {e.description != null && <div className="text-sm"><Markdown text={String(e.description)} /></div>}
        {fields.length > 0 && (
          <div className="grid grid-cols-3 gap-2 pt-1">
            {fields.map((f, i) => (
              <div key={i} className={f.inline ? "" : "col-span-3"}>
                <p className="text-xs font-semibold text-white">{String(f.name)}</p>
                <p className="text-sm">{String(f.value)}</p>
              </div>
            ))}
          </div>
        )}
        {e.image != null && <Picture className="mt-2 h-32 w-full" />}
        {e.footer != null && <p className="pt-1 text-xs text-[#949ba4]">{String(e.footer)}</p>}
      </div>
      {e.thumbnail != null && <Picture className="size-16 shrink-0" />}
    </div>
  );
}

function Avatar({ bot, name }: { bot: boolean; name: string }) {
  return (
    <span
      className={cn(
        "flex size-10 shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white",
        bot ? "bg-gradient-to-br from-[#2a2a2a] to-[#0a0a0a] ring-1 ring-white/20" : "bg-[#5865f2]",
      )}
      aria-hidden
    >
      {bot ? "✦" : name.slice(0, 1).toUpperCase()}
    </span>
  );
}

export function DiscordPreview({ data, className }: { data: PreviewData; className?: string }) {
  return (
    <div
      className={cn(
        "overflow-hidden rounded-2xl border border-white/[0.08] bg-[#313338] font-sans text-[15px] leading-[1.375] text-[#dbdee1] shadow-[0_30px_80px_-40px_rgba(0,0,0,0.9)]",
        className,
      )}
    >
      <div className="flex items-center gap-2 border-b border-black/30 bg-[#2b2d31] px-4 py-2 text-xs text-[#949ba4]">
        <span className="text-base leading-none">#</span>
        <span className="font-medium text-[#dbdee1]">general</span>
        <span className="ml-auto rounded bg-black/25 px-1.5 py-0.5 text-[10px] uppercase tracking-wide">simulação</span>
      </div>
      <div className="space-y-4 p-4">
        {data.messages.map((m, i) => {
          const bot = m.from === "bot";
          const name = m.name ?? (bot ? "My Bot" : "Ana");
          return (
            <div key={i}>
              {m.command && (
                <p className="mb-1 ml-[52px] text-xs text-[#949ba4]">
                  <span className="font-medium text-[#dbdee1]">Ana</span> usou{" "}
                  <span className="rounded bg-[#5865f2]/30 px-1 font-medium text-[#c9cdfb]">{m.command}</span>
                </p>
              )}
              <div className="flex gap-3">
                <Avatar bot={bot} name={name} />
                <div className="min-w-0 flex-1 space-y-1.5">
                  <p className="flex items-center gap-1.5">
                    <span className={cn("font-medium", bot ? "text-white" : "text-[#c9cdfb]")}>{name}</span>
                    {bot && <span className="rounded bg-[#5865f2] px-1 py-px text-[10px] font-semibold text-white">APP</span>}
                    <span className="text-xs text-[#949ba4]">hoje</span>
                  </p>
                  {m.text != null && <div><Markdown text={m.text} /></div>}
                  {m.embed && <Embed e={m.embed} />}
                  {m.layout && (
                    <div className="max-w-lg space-y-2.5">
                      {m.layout.map((p, j) => (
                        <Piece key={j} piece={p} />
                      ))}
                    </div>
                  )}
                  {m.reactions && (
                    <div className="flex gap-1.5">
                      {m.reactions.map((r) => (
                        <span key={r} className="rounded-lg border border-[#5865f2]/60 bg-[#5865f2]/20 px-2 py-0.5 text-sm">
                          {r} <span className="text-xs text-[#c9cdfb]">1</span>
                        </span>
                      ))}
                    </div>
                  )}
                  {m.ephemeral && <p className="text-xs text-[#949ba4]">👁 Só você pode ver esta mensagem</p>}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
