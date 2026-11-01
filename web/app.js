import {
  God,
  percents,
  shout,
  reading,
  isoDate,
  today,
  HORIZON_DAYS,
  song,
  play,
  doodle,
  BIBLE_LINES,
  line2verse,
  GOD_GOOD_BITS,
} from "./god.js";
import { BOOKS, BOOK_LINES } from "./books.js";

const $ = (id) => document.getElementById(id);
const stage = $("stage");
const out = $("out");
const prompt = $("prompt");
const promptText = $("promptText");
const cells = $("cells");
const tapcount = $("tapcount");
const bitcount = $("bitcount");
const go = $("go");

let mode = "word";
let drawing = false;
let waiting = null; // resolver for the keystroke the oracle is blocked on
let baseDay = today(); // the day the draw was made, kept when one is shared
let lines = 20;

/* ----------------------------------------------------------- bit meter */

const CELLS = GOD_GOOD_BITS;
for (let i = 0; i < CELLS; i++) cells.appendChild(document.createElement("i"));

function paintMeter(god) {
  const held = god ? god.fifo.length : 0;
  [...cells.children].forEach((c, i) => {
    c.className = i < held ? "on" : i < CELLS ? "spent" : "";
  });
  tapcount.textContent = `${god ? god.taps : 0} tap${god && god.taps === 1 ? "" : "s"}`;
  bitcount.textContent = `${held} bits held`;
}
paintMeter(null);

function say(text, waitingNow) {
  promptText.textContent = text;
  prompt.classList.toggle("waiting", !!waitingNow);
}

/** Any key, click or touch is a keystroke as far as the oracle cares. */
function onTap() {
  if (waiting) {
    const resolve = waiting;
    waiting = null;
    resolve();
  }
}

addEventListener("keydown", (e) => {
  if (e.key === "Tab" || e.metaKey || e.ctrlKey || e.altKey) return;
  if (!drawing && (e.key === "Enter" || e.key === " ")) {
    if (document.activeElement === document.body) {
      e.preventDefault();
      draw();
    }
    return;
  }
  if (drawing) {
    e.preventDefault();
    onTap();
  }
});

stage.addEventListener("pointerdown", (e) => {
  if (drawing && !e.target.closest("button, input")) onTap();
});
$("tap").addEventListener("click", onTap);

/* -------------------------------------------------------------- vocab */

const vocabCache = {};
async function vocab(tr) {
  const key = tr ? "tr" : "en";
  if (!vocabCache[key]) {
    const res = await fetch(`data/vocab-${key}.txt`);
    vocabCache[key] = (await res.text()).split(/\s+/).filter(Boolean);
  }
  return vocabCache[key];
}

let bibleCache = null;
async function bible() {
  if (!bibleCache) {
    const res = await fetch("data/bible-kjv.txt");
    bibleCache = (await res.text()).split(/\r?\n/);
  }
  return bibleCache;
}

/* --------------------------------------------------------------- modes */

document.querySelectorAll(".modes button").forEach((b) => {
  b.addEventListener("click", () => {
    if (drawing) return;
    document
      .querySelectorAll(".modes button")
      .forEach((o) => o.setAttribute("aria-selected", String(o === b)));
    mode = b.dataset.mode;
    document.querySelector('[data-for="count"]').style.visibility =
      mode === "word" ? "visible" : "hidden";
    out.innerHTML = "";
    say(HINTS[mode], false);
  });
});

const HINTS = {
  word: "tap any key — the clock is read at that instant, and you are the randomness",
  bible: "one draw over 100,110 lines of the King James text",
  song: "two measures, twice; it plays when the last note is picked",
  doodle: "twenty strokes and six floods, one keystroke at a time",
};

$("count").addEventListener("input", (e) => {
  e.target.nextElementSibling.textContent = e.target.value;
});

/* ---------------------------------------------------------------- draw */

go.addEventListener("click", () => draw());

async function draw(replay) {
  if (drawing) return;
  drawing = true;
  go.disabled = true;
  out.innerHTML = "";

  const tr = $("tr").checked;
  let god;
  if (replay) {
    try {
      god = God.fromHex(replay);
    } catch (err) {
      drawing = false;
      go.disabled = false;
      return say(err.message, false);
    }
  } else {
    baseDay = today();
    god = new God(
      () =>
        new Promise((resolve) => {
          waiting = resolve;
          say("tap a key", true);
        }),
    );
  }
  const tick = () => paintMeter(god);
  const bits = god.bits.bind(god);
  god.bits = async (n) => {
    const v = await bits(n);
    tick();
    return v;
  };

  try {
    if (mode === "word") await drawWords(god, tr);
    else if (mode === "bible") await drawBible(god);
    else if (mode === "song") await drawSong(god);
    else await drawDoodle(god);
    showShare(god, tr);
    say(HINTS[mode], false);
  } catch (err) {
    say(`the draw failed: ${err.message}`, false);
  } finally {
    drawing = false;
    go.disabled = false;
    paintMeter(god);
  }
}

async function drawWords(god, tr) {
  const words = await vocab(tr);
  const n = +$("count").value;
  const picks = [];
  for (let i = 0; i < n; i++) picks.push(await god.word(words));
  const weights = [];
  for (let i = 0; i < n; i++) weights.push((await god.bits(10)) + 1);
  const offset = (await god.bits(10)) % HORIZON_DAYS;
  const phrasing = await god.bits(2);

  const rows = picks
    .map((w, i) => [shout(w, tr), percents(weights)[i]])
    .sort((a, b) => b[1] - a[1]);
  const when = isoDate(baseDay + offset);

  const list = document.createElement("div");
  list.className = "rows";
  rows.forEach(([w, p], i) => {
    const row = document.createElement("div");
    row.className = "row";
    row.style.animationDelay = `${i * 70}ms`;
    row.innerHTML = `<span class="w"></span><span class="bar" style="--p:${p}%"></span><span class="p">${p}%</span>`;
    row.querySelector(".w").textContent = w;
    row.querySelector(".bar").style.animationDelay = `${i * 70 + 120}ms`;
    list.appendChild(row);
  });

  const stamp = document.createElement("p");
  stamp.className = "stamp";
  stamp.innerHTML = `<b></b><span></span>`;
  stamp.querySelector("b").textContent = when;
  stamp.querySelector("span").textContent = tr
    ? `+${offset} gün`
    : `+${offset} days`;

  const verdict = document.createElement("p");
  verdict.className = "verdict";
  verdict.textContent = reading(phrasing, rows, when, offset, tr);

  out.append(list, stamp, verdict);
}

async function drawBible(god) {
  say("fetching the text…", false);
  const text = await bible();
  const span = BIBLE_LINES - (lines - 1);
  const start = ((await god.bits(21)) % span) + 1;
  const ref = line2verse(text, start, BOOKS, BOOK_LINES);

  const box = document.createElement("div");
  box.className = "passage";
  const h = document.createElement("p");
  h.className = "ref";
  h.textContent = ref || "—";
  const body = document.createElement("p");
  body.className = "body";
  body.textContent = text.slice(start - 1, start - 1 + lines).join("\n");
  box.append(h, body);
  out.append(box);
}

/* --------------------------------------------------------------- share */

/** The record is the draw written down; the link replays it anywhere. */
function showShare(god, tr) {
  const record = god.record();
  const params = `m=${mode}&n=${$("count").value}&l=${lines}&tr=${tr ? 1 : 0}&d=${baseDay}&h=${record}`;
  history.replaceState(null, "", `#${params}`);
  const link = `${location.origin}${location.pathname}#${params}`;

  const box = document.createElement("div");
  box.className = "share";
  box.innerHTML = `
    <p class="rec"><span>record</span><code></code></p>
    <div class="share-actions">
      <button data-copy="link">copy link</button>
      <button data-copy="text">copy reading</button>
      <button data-copy="cli">copy command</button>
    </div>`;
  box.querySelector("code").textContent = record;

  const plain = () =>
    [...out.querySelectorAll(".row")]
      .map((r) => `${r.querySelector(".w").textContent}  ${r.querySelector(".p").textContent}`)
      .join("\n") ||
    out.textContent.trim();

  box.querySelectorAll("button").forEach((b) => {
    b.addEventListener("click", async () => {
      const what = b.dataset.copy;
      const text =
        what === "link"
          ? link
          : what === "cli"
            ? `tlk-see ${mode === "word" ? "" : mode + " "}--hex ${record} --day ${baseDay}${tr ? " --tr" : ""}`
            : `${plain()}\n\n${out.querySelector(".verdict")?.textContent ?? ""}`.trim();
      try {
        await navigator.clipboard.writeText(text);
        const was = b.textContent;
        b.textContent = "copied";
        setTimeout(() => (b.textContent = was), 1400);
      } catch {
        say("the browser would not let me reach the clipboard", false);
      }
    });
  });
  out.append(box);
}

/** A link carries a whole draw; replay it instead of asking for keys. */
function fromHash() {
  const p = new URLSearchParams(location.hash.slice(1));
  const hex = p.get("h");
  if (!hex) return;
  const m = p.get("m");
  if (m && HINTS[m]) {
    mode = m;
    document
      .querySelectorAll(".modes button")
      .forEach((b) => b.setAttribute("aria-selected", String(b.dataset.mode === m)));
  }
  const n = +p.get("n");
  if (n >= 1 && n <= 8) {
    $("count").value = n;
    $("count").nextElementSibling.textContent = n;
  }
  const l = +p.get("l");
  if (l >= 1 && l <= 200) lines = l;
  $("tr").checked = p.get("tr") === "1";
  const d = +p.get("d");
  baseDay = Number.isFinite(d) && d > 0 ? d : today();
  document.querySelector('[data-for="count"]').style.visibility =
    mode === "word" ? "visible" : "hidden";
  say("replaying a shared draw", false);
  draw(hex);
}

fromHash();

async function drawSong(god) {
  const str = await song(god);
  const p = document.createElement("p");
  p.className = "notes";
  p.textContent = str;
  out.append(p);
  say("playing", false);
  play(str, () => say(HINTS.song, false));
}

async function drawDoodle(god) {
  const canvas = document.createElement("canvas");
  canvas.width = canvas.height = 320;
  out.append(canvas);
  const ctx = canvas.getContext("2d");
  const dc = await doodle(god, 320, (raster) => raster.paint(ctx));
  dc.paint(ctx);
}
