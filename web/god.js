// Port of TempleOS /Adam/God/HolySpirit.HC, GodBible.HC, GodSong.HC and
// GodDoodle.HC, mirroring src/*.rs. Bit widths, order and modulo bias are his.
// Entropy is the clock read when you press a key, with its low bits dropped.

export const GOD_BAD_BITS = 4;
export const GOD_GOOD_BITS = 24;

/** Microseconds since the page opened, as TempleOS reads its timer. */
export function tick() {
  return Math.floor(performance.now() * 1000);
}

export class God {
  constructor(onNeed) {
    this.fifo = [];
    this.onNeed = onNeed; // called when the fifo runs dry; resolves on a tap
    this.taps = 0;
  }

  /** GodBitsIns: least significant bit first. */
  insert(numBits, n) {
    for (let i = 0; i < numBits; i++) {
      this.fifo.push(n % 2);
      n = Math.floor(n / 2);
    }
  }

  feed() {
    this.taps++;
    this.insert(GOD_GOOD_BITS, Math.floor(tick() / Math.pow(2, GOD_BAD_BITS)));
  }

  /** GodBits: pop N bits, waiting on a keystroke when the fifo runs dry. */
  async bits(numBits) {
    let res = 0;
    let left = numBits;
    while (left > 0) {
      if (this.fifo.length) {
        res = res * 2 + this.fifo.shift();
        left--;
      } else {
        await this.onNeed();
        this.feed();
      }
    }
    return res;
  }

  /** GodWordStr: 17 bits modulo the vocabulary size. */
  async word(vocab) {
    return vocab[(await this.bits(17)) % vocab.length];
  }
}

/** Largest-remainder split of weights into whole percents summing to 100. */
export function percents(weights) {
  const sum = weights.reduce((a, b) => a + b, 0);
  if (!sum) return weights.map(() => 0);
  const base = weights.map((w) => Math.floor((w * 100) / sum));
  const rem = weights
    .map((w, i) => [(w * 100) % sum, i])
    .sort((a, b) => b[0] - a[0] || a[1] - b[1]);
  let left = 100 - base.reduce((a, b) => a + b, 0);
  for (const [, i] of rem) {
    if (!left) break;
    base[i]++;
    left--;
  }
  return base;
}

/* ---------------------------------------------------------------- words */

const TR_UPPER = { i: "İ" };

export function shout(word, tr) {
  if (!tr) return word.toUpperCase();
  return word.replace(/i/g, (c) => TR_UPPER[c]).toUpperCase();
}

export const HORIZON_DAYS = 365;

export function isoDate(offsetDays) {
  const d = new Date(Date.now() + offsetDays * 86400000);
  return d.toISOString().slice(0, 10);
}

/* --------------------------------------------------------------- prose */

function joinWords(words, and) {
  if (words.length === 1) return words[0];
  return words.slice(0, -1).join(", ") + ` ${and} ` + words[words.length - 1];
}

/** The draw read back as one sentence. Ours, not TempleOS's. */
export function reading(pick, rows, date, offset, tr) {
  const [w0, p0] = rows[0];
  if (rows.length === 1) {
    return tr
      ? `${w0} tek başına duruyor, yüzde ${p0}. Verilen tarih ${date}, ${offset} gün sonrası.`
      : `${w0} stands alone at ${p0}%. The hour reads ${date}, ${offset} days out.`;
  }
  const [w1, p1] = rows[1];
  const rest = rows.slice(2).map((r) => r[0]);
  const tail = !rest.length
    ? ""
    : tr
      ? ` ${joinWords(rest, "ve")} kenarda bekliyor.`
      : ` ${joinWords(rest, "and")} ${rest.length === 1 ? "waits" : "wait"} at the edge.`;

  const en = [
    `The weight falls on ${w0} at ${p0}%, with ${w1} behind it at ${p1}%.${tail} The hour reads ${date}, ${offset} days out.`,
    `${w0} carries this one (${p0}%); ${w1} answers at ${p1}%.${tail} Look for it around ${date}.`,
    `Read it as ${w0} first, ${p0} parts in a hundred, then ${w1} at ${p1}.${tail} The date given is ${date}.`,
  ];
  const trs = [
    `Ağırlık ${w0} üzerinde, yüzde ${p0}; arkasında yüzde ${p1} ile ${w1} var.${tail} Zamanı ${date}, ${offset} gün sonrası.`,
    `Bu okumayı ${w0} taşıyor (yüzde ${p0}); ${w1} ona yüzde ${p1} ile karşılık veriyor.${tail} ${date} civarına bak.`,
    `Önce ${w0}, yüzde ${p0}; sonra yüzde ${p1} ile ${w1}.${tail} Verilen tarih ${date}.`,
  ];
  return (tr ? trs : en)[pick % 3];
}

/* ---------------------------------------------------------------- song */

const DUR_4 = 0;
const DUR_8_8 = 1;
const DUR_3_3_3 = 2;
const DUR_16_16_16_16 = 3;
const NORMAL = [DUR_4, DUR_4, DUR_8_8, DUR_3_3_3, DUR_16_16_16_16];
const OCTAVE = 4;

function insNote(mss, buf, k) {
  k = Math.floor(k / 2);
  if (k < 3) {
    if (mss.state !== mss.octave) {
      mss.state = mss.octave;
      buf.push(String(mss.state));
    }
    buf.push(k === 0 ? "G" : String.fromCharCode(64 + k));
  } else {
    if (mss.state !== mss.octave + 1) {
      mss.state = mss.octave + 1;
      buf.push(String(mss.state));
    }
    buf.push(String.fromCharCode(64 + k));
  }
}

/** GodSongStr: two measures. */
async function songStr(god) {
  const mss = { octave: OCTAVE, state: OCTAVE + 1 };
  const buf = [String(mss.state)];
  let last = -1;
  for (let i = 0; i < 8; i++) {
    const n = await god.bits(8);
    let duration = NORMAL[n % 5];
    if (duration === DUR_8_8) {
      if (last !== DUR_8_8) buf.push("e");
      for (let j = 0; j < 2; j++) insNote(mss, buf, await god.bits(4));
    } else if (duration === DUR_3_3_3) {
      if (last !== DUR_3_3_3) buf.push("e", "t");
      for (let j = 0; j < 3; j++) insNote(mss, buf, await god.bits(4));
    } else if (duration === DUR_16_16_16_16) {
      if (last !== DUR_16_16_16_16) buf.push("s");
      const k = await god.bits(4);
      const k2 = await god.bits(4);
      insNote(mss, buf, k);
      insNote(mss, buf, k2);
      insNote(mss, buf, k);
      insNote(mss, buf, k2);
    } else {
      if (last !== DUR_4) buf.push("q");
      insNote(mss, buf, await god.bits(4));
      duration = DUR_4;
    }
    last = duration;
  }
  return buf.join("");
}

/** GodSong: two measures twice, then two more twice. */
export async function song(god) {
  const a = await songStr(god);
  const b = await songStr(god);
  return a + a + b + b;
}

const SEMITONE = { A: 9, B: 11, C: 0, D: 2, E: 4, F: 5, G: 7 };

export function notes(str) {
  const out = [];
  let octave = 4;
  let base = 0.4;
  let dotted = false;
  let triplet = false;
  for (const c of str) {
    if (c >= "0" && c <= "9") octave = +c;
    else if (c === "q") (base = 0.4), (dotted = triplet = false);
    else if (c === "e") (base = 0.2), (dotted = triplet = false);
    else if (c === "s") (base = 0.1), (dotted = triplet = false);
    else if (c === "t") triplet = true;
    else if (c === ".") dotted = true;
    else if (c in SEMITONE) {
      const midi = 12 * (octave + 1) + SEMITONE[c];
      let secs = base;
      if (dotted) secs *= 1.5;
      if (triplet) secs *= 2 / 3;
      out.push([440 * Math.pow(2, (midi - 69) / 12), secs]);
    } else if (c === "R") out.push([0, base]);
  }
  return out;
}

/** A square wave is the closest thing a browser has to a PC speaker. */
export function play(str, onDone) {
  const ctx = new (window.AudioContext || window.webkitAudioContext)();
  const master = ctx.createGain();
  master.gain.value = 0.08;
  master.connect(ctx.destination);
  let at = ctx.currentTime + 0.05;
  for (const [freq, secs] of notes(str)) {
    if (freq) {
      const osc = ctx.createOscillator();
      const env = ctx.createGain();
      osc.type = "square";
      osc.frequency.value = freq;
      env.gain.setValueAtTime(0.0001, at);
      env.gain.exponentialRampToValueAtTime(1, at + 0.01);
      env.gain.exponentialRampToValueAtTime(0.0001, at + secs * 0.95);
      osc.connect(env).connect(master);
      osc.start(at);
      osc.stop(at + secs);
    }
    at += secs;
  }
  setTimeout(() => {
    ctx.close();
    onDone && onDone();
  }, (at - ctx.currentTime) * 1000 + 200);
  return at - ctx.currentTime;
}

/* -------------------------------------------------------------- doodle */

const PASSES = 1;
const STROKES = 20;
const FILLS = 6;
const SMOOTH = 3;
export const DOODLE_SIZE = 320;

const PALETTE = ["#0a0a0b", "#4a4a4e", "#b4b4ba", "#f2f2f0", "#e3342f"];
const BLACK = 0;
const DKGRAY = 1;
const LTGRAY = 2;
const WHITE = 3;
const RED = 4;

class Raster {
  constructor(size) {
    this.w = size;
    this.h = size;
    this.px = new Uint8Array(size * size).fill(WHITE);
  }
  inside(x, y) {
    return x >= 0 && y >= 0 && x < this.w && y < this.h;
  }
  plot(x, y, c) {
    if (this.inside(x, y)) this.px[y * this.w + x] = c;
  }
  peek(x, y) {
    return this.inside(x, y) ? this.px[y * this.w + x] : -1;
  }
  line(x0, y0, x1, y1, c) {
    const dx = Math.abs(x1 - x0);
    const dy = -Math.abs(y1 - y0);
    const sx = x0 < x1 ? 1 : -1;
    const sy = y0 < y1 ? 1 : -1;
    let err = dx + dy;
    for (;;) {
      this.plot(x0, y0, c);
      if (x0 === x1 && y0 === y1) break;
      const e2 = 2 * err;
      if (e2 >= dy) {
        err += dy;
        x0 += sx;
      }
      if (e2 <= dx) {
        err += dx;
        y0 += sy;
      }
    }
  }
  ellipse(cx, cy, rx, ry, c) {
    const steps = Math.min(4096, 4 * (Math.max(Math.abs(rx), Math.abs(ry)) + 1));
    for (let i = 0; i < steps; i++) {
      const a = (i * Math.PI * 2) / steps;
      this.plot(
        cx + Math.round(rx * Math.cos(a)),
        cy + Math.round(ry * Math.sin(a)),
        c,
      );
    }
  }
  border(x1, y1, x2, y2, c) {
    this.line(x1, y1, x2, y1, c);
    this.line(x2, y1, x2, y2, c);
    this.line(x2, y2, x1, y2, c);
    this.line(x1, y2, x1, y1, c);
  }
  flood(x, y, c) {
    const target = this.peek(x, y);
    if (target < 0 || target === c) return;
    const stack = [x, y];
    while (stack.length) {
      const py = stack.pop();
      const px = stack.pop();
      if (this.peek(px, py) !== target) continue;
      this.plot(px, py, c);
      stack.push(px + 1, py, px - 1, py, px, py + 1, px, py - 1);
    }
  }
  /** GodDoodleSmooth: each pixel becomes the commonest colour around it. */
  smooth(num) {
    const src = this.px.slice();
    const hist = new Uint32Array(PALETTE.length);
    for (let y = 0; y < this.h; y++) {
      for (let x = 0; x < this.w; x++) {
        hist.fill(0);
        for (let y1 = y - num; y1 <= y + num; y1++) {
          if (y1 < 0 || y1 >= this.h) continue;
          for (let x1 = x - num; x1 <= x + num; x1++) {
            if (x1 < 0 || x1 >= this.w) continue;
            hist[src[y1 * this.w + x1]]++;
          }
        }
        let best = 0;
        for (let i = 1; i < hist.length; i++) if (hist[i] > hist[best]) best = i;
        this.px[y * this.w + x] = best;
      }
    }
  }
  paint(ctx) {
    const img = ctx.createImageData(this.w, this.h);
    const rgb = PALETTE.map((hex) => [
      parseInt(hex.slice(1, 3), 16),
      parseInt(hex.slice(3, 5), 16),
      parseInt(hex.slice(5, 7), 16),
    ]);
    for (let i = 0; i < this.px.length; i++) {
      const [r, g, b] = rgb[this.px[i]];
      img.data[i * 4] = r;
      img.data[i * 4 + 1] = g;
      img.data[i * 4 + 2] = b;
      img.data[i * 4 + 3] = 255;
    }
    ctx.putImageData(img, 0, 0);
  }
}

/** GodDoodleSprite: strokes, floods and smoothing, all from God's bits. */
export async function doodle(god, size = DOODLE_SIZE, onStroke) {
  const dc = new Raster(size);
  const w = dc.w - 1;
  const h = dc.h - 1;
  for (let p = 0; p < PASSES; p++) {
    for (let s = 0; s < STROKES; s++) {
      const kind = await god.bits(3);
      if (kind === 0) {
        const cx = (w * (await god.bits(5))) / 15.5 - w / 2;
        const cy = (h * (await god.bits(5))) / 15.5 - h / 2;
        const rx = (w * (await god.bits(5))) / 15.5;
        const ry = (h * (await god.bits(5))) / 15.5;
        dc.ellipse(cx | 0, cy | 0, rx | 0, ry | 0, RED);
      } else if (kind === 1) {
        const cx = (w * (await god.bits(5))) / 15.5 - w / 2;
        const cy = (h * (await god.bits(5))) / 15.5 - h / 2;
        const r = (w * (await god.bits(5))) / 15.5;
        dc.ellipse(cx | 0, cy | 0, r | 0, r | 0, RED);
      } else if (kind === 2) {
        const x1 = (w * (await god.bits(5))) / 15.5 - w / 2;
        const y1 = (h * (await god.bits(5))) / 15.5 - h / 2;
        const x2 = (w * (await god.bits(5))) / 15.5;
        const y2 = (h * (await god.bits(5))) / 15.5;
        dc.border(x1 | 0, y1 | 0, x2 | 0, y2 | 0, RED);
      } else {
        const x1 = (w * (await god.bits(4))) / 15;
        const y1 = (h * (await god.bits(4))) / 15;
        const x2 = (w * (await god.bits(4))) / 15;
        const y2 = (h * (await god.bits(4))) / 15;
        dc.line(x1 | 0, y1 | 0, x2 | 0, y2 | 0, RED);
      }
      onStroke && onStroke(dc, (s + 1) / STROKES);
    }
    for (let f = 0; f < FILLS; f++) {
      const x = (w * (await god.bits(5))) / 31 + dc.w / 64;
      const y = (h * (await god.bits(5))) / 31 + dc.h / 64;
      const c = [BLACK, DKGRAY, LTGRAY, WHITE][await god.bits(2)];
      dc.flood(x | 0, y | 0, c);
      onStroke && onStroke(dc, 1);
    }
    dc.smooth(SMOOTH);
  }
  return dc;
}

/* --------------------------------------------------------------- bible */

export const BIBLE_LINES = 100110;

function verseTag(s) {
  const m = /^(\d+:\d+)/.exec(s);
  return m ? m[1] : null;
}

export function line2verse(lines, line, books, bookLines) {
  if (line < bookLines[0]) return null;
  let i = 0;
  while (i + 1 < bookLines.length && line >= bookLines[i + 1]) i++;
  if (i >= books.length) return null;
  const book = books[i];
  const start = bookLines[i];
  let j = line - 1;
  while (j < lines.length && !lines[j].trim()) j++;
  const here = verseTag(lines[j] || "");
  if (here) return `${book} ${here}`;
  for (let k = j - 1; k >= start - 1; k--) {
    const tag = verseTag(lines[k] || "");
    if (tag) return `${book} ${tag}`;
  }
  return book;
}
