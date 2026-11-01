// Every data-i18n key on the page must have Turkish, or the switch leaves
// English behind in the middle of a Turkish sentence.

import { readFileSync } from "node:fs";
import { TR, HINTS } from "./i18n.js";

const here = (f) => new URL(f, import.meta.url);
const sources = ["index.html", "app.js"].map((f) =>
  readFileSync(here(f), "utf8"),
);

const keys = new Set();
for (const src of sources) {
  for (const m of src.matchAll(/data-i18n="([^"]+)"/g)) keys.add(m[1]);
}

const missing = [...keys].filter((k) => TR[k] === undefined);
const unused = Object.keys(TR).filter((k) => !keys.has(k));

const modes = ["word", "bible", "song", "doodle"];
const hintKeys = [
  ...modes,
  "tap",
  "taps",
  "bits",
  "fetching",
  "playing",
  "replaying",
  "clipboard",
  "copied",
  "failed",
];
const hintGaps = [];
for (const lang of ["en", "tr"]) {
  for (const k of hintKeys) {
    if (HINTS[lang]?.[k] === undefined) hintGaps.push(`${lang}.${k}`);
  }
}

console.log(`${keys.size} marked strings, ${Object.keys(TR).length} translated`);
if (missing.length) console.error("no Turkish for:", missing.join(", "));
if (unused.length) console.error("translated but unused:", unused.join(", "));
if (hintGaps.length) console.error("missing hints:", hintGaps.join(", "));
process.exit(missing.length || unused.length || hintGaps.length ? 1 : 0);
