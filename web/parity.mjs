// Replays a record through the browser build and prints one canonical line,
// so CI can check it against the same record replayed by the Rust build.
//
//   node web/parity.mjs <record> <count> <base-day>

import { readFileSync } from "node:fs";
import { God, percents, shout, isoDate, HORIZON_DAYS } from "./god.js";

const [record, count = "3", baseDay = "20758"] = process.argv.slice(2);
if (!record) {
  console.error("usage: node web/parity.mjs <record> [count] [base-day]");
  process.exit(2);
}

const vocab = readFileSync(
  new URL("../data/vocab-en.txt", import.meta.url),
  "utf8",
)
  .split(/\s+/)
  .filter(Boolean);

const god = God.fromHex(record);
const n = Number(count);
const picks = [];
for (let i = 0; i < n; i++) picks.push(await god.word(vocab));
const weights = [];
for (let i = 0; i < n; i++) weights.push((await god.bits(10)) + 1);
const pct = percents(weights);
const offset = (await god.bits(10)) % HORIZON_DAYS;

const rows = picks
  .map((w, i) => `${shout(w, false)} ${pct[i]}`)
  .sort((a, b) => Number(b.split(" ")[1]) - Number(a.split(" ")[1]));

console.log(`${rows.join("|")}|${isoDate(Number(baseDay) + offset)}`);
