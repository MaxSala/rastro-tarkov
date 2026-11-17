// Parser for the small Markdown subset CHANGELOG.md uses: "## [x.y.z] - date" versions,
// "### Group" headings and "- item" bullets. Inline `code` and *emphasis* are kept as tokens
// and rendered as React elements, never as HTML.

export interface Release {
  version: string;
  date: string;
  groups: { title: string; items: string[] }[];
}

const VERSION = /^## \[(\d+\.\d+\.\d+)\](?: - (\d{4}-\d{2}-\d{2}))?/;

export function parseChangelog(md: string): Release[] {
  const out: Release[] = [];
  for (const raw of md.split(/\r?\n/).slice(0, 4000)) {
    const line = raw.trimEnd();
    const v = VERSION.exec(line);
    if (v) {
      out.push({ version: v[1] ?? "", date: v[2] ?? "", groups: [] });
      continue;
    }
    const rel = out.at(-1);
    if (!rel) continue;
    if (line.startsWith("### ")) rel.groups.push({ title: line.slice(4).trim(), items: [] });
    else if (line.startsWith("- ")) {
      const group = rel.groups.at(-1) ?? { title: "", items: [] };
      if (rel.groups.length === 0) rel.groups.push(group);
      group.items.push(line.slice(2).trim());
    }
  }
  return out.slice(0, 50);
}

export type Token = { kind: "text" | "code" | "em"; value: string };

export function inline(text: string): Token[] {
  const tokens: Token[] = [];
  const re = /`([^`]+)`|\*([^*]+)\*/g;
  let last = 0;
  for (const m of text.matchAll(re)) {
    if (m.index > last) tokens.push({ kind: "text", value: text.slice(last, m.index) });
    if (m[1] !== undefined) tokens.push({ kind: "code", value: m[1] });
    else tokens.push({ kind: "em", value: m[2] ?? "" });
    last = m.index + m[0].length;
  }
  if (last < text.length) tokens.push({ kind: "text", value: text.slice(last) });
  return tokens;
}
