import { CodeBlock } from "@/components/agents/code-block";
import type { AgentCodeLanguage } from "@/components/agents/agent-code";
import { cn } from "@/lib/utils";

const ALIASES: Record<string, AgentCodeLanguage> = {
  dcs: "dcs",
  diracord: "dcs",
  js: "javascript",
  javascript: "javascript",
  ts: "typescript",
  typescript: "typescript",
  bash: "bash",
  sh: "bash",
  shell: "bash",
  json: "json",
  toml: "toml",
  diff: "diff",
};

/** A highlighted code window (beUI CodeBlock), with Diracord support. */
export function Code({
  code,
  lang = "dcs",
  filename,
  className,
  lineNumbers = false,
}: {
  code: string;
  lang?: string;
  filename?: string;
  className?: string;
  lineNumbers?: boolean;
}) {
  const language = ALIASES[lang] ?? "text";
  return (
    <CodeBlock
      code={code.replace(/\n$/, "")}
      language={language}
      filename={filename}
      showLineNumbers={lineNumbers}
      className={cn("text-left", className)}
    />
  );
}
