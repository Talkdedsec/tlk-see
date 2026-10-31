//! tlk-see — TempleOS's oracle in a terminal, with a percentage spread.
//!
//! Terry A. Davis let the Holy Spirit pick words through a timer latched on a
//! keypress. The picking is his, bit for bit; the percentages are ours.

mod bible;
mod bible_books;
mod date;
mod god;

use god::{percents, tsc, God, GOD_BAD_BITS};
use std::io::{self, IsTerminal, Write};

const VOCAB_EN: &str = include_str!("../data/vocab-en.txt");
const VOCAB_TR: &str = include_str!("../data/vocab-tr.txt");
const BIBLE: &str = include_str!("../data/bible-kjv.txt");

const HELP: &str = "\
tlk-see — TempleOS oracle, with percentages

USAGE
  tlk-see [word] [-n <count>]   words with a percentage spread, plus a date (default, 4 words)
  tlk-see bible [-l <lines>]    a random passage (default 20 lines)

OPTIONS
  --tr          Turkish vocabulary
  --en          English vocabulary, TempleOS Vocab.DD verbatim (default)
  -n <count>    how many words (1-32)
  -l <lines>    how many lines of scripture (1-200)
  -h, --help    this
  -V, --version version

Each draw needs entropy: press ENTER when asked. The timer is read at that
instant and its low bits thrown away, as GOD_BAD_BITS does in TempleOS.
";

struct Args {
    bible: bool,
    tr: bool,
    count: usize,
    lines: usize,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args { bible: false, tr: false, count: 4, lines: 20 };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "word" | "words" => a.bible = false,
            "bible" | "verse" => a.bible = true,
            "--tr" => a.tr = true,
            "--en" => a.tr = false,
            "-n" | "--count" => {
                a.count = numeric(it.next(), "-n", 1, 32)?;
            }
            "-l" | "--lines" => {
                a.lines = numeric(it.next(), "-l", 1, 200)?;
            }
            "-h" | "--help" => {
                print!("{HELP}");
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("tlk-see {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(a)
}

fn numeric(v: Option<String>, flag: &str, lo: usize, hi: usize) -> Result<usize, String> {
    let n: usize = v
        .ok_or_else(|| format!("{flag} needs a number"))?
        .parse()
        .map_err(|_| format!("{flag} needs a number"))?;
    if n < lo || n > hi {
        return Err(format!("{flag} must be {lo}-{hi}"));
    }
    Ok(n)
}

struct Style {
    dim: &'static str,
    hot: &'static str,
    off: &'static str,
}

fn style() -> Style {
    let plain = std::env::var_os("NO_COLOR").is_some() || !io::stdout().is_terminal();
    if plain {
        Style { dim: "", hot: "", off: "" }
    } else {
        Style { dim: "\x1b[90m", hot: "\x1b[1;33m", off: "\x1b[0m" }
    }
}

/// Turkish keeps its dotted I when upper-cased; Rust's default would not.
fn shout(w: &str, tr: bool) -> String {
    if tr {
        w.replace('i', "\u{0130}").to_uppercase()
    } else {
        w.to_uppercase()
    }
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("tlk-see: {e}\n\n{HELP}");
            std::process::exit(2);
        }
    };
    let st = style();

    // GodPick: wait on a keypress, then read the timer. Piped stdin hits EOF,
    // so stop prompting and keep drawing from the timer alone.
    let mut eof = false;
    let mut latch = || {
        if !eof {
            print!("{}  press ENTER {}", st.dim, st.off);
            let _ = io::stdout().flush();
            let mut line = String::new();
            if io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
                eof = true;
                println!();
            }
        }
        tsc() >> GOD_BAD_BITS
    };
    let mut god = God::new(&mut latch);

    if args.bible {
        let lines: Vec<&str> = BIBLE.lines().collect();
        let span = bible::BIBLE_LINES - (args.lines - 1);
        let start = god.bits(21) as usize % span + 1;
        let verse = bible::line2verse(&lines, start);
        println!();
        if let Some(v) = &verse {
            println!("{}  {v}{}", st.hot, st.off);
            println!();
        }
        for l in bible::book_lines(&lines, start, args.lines) {
            println!("  {l}");
        }
        println!();
        return;
    }

    let vocab: Vec<&str> = if args.tr { VOCAB_TR } else { VOCAB_EN }
        .split_whitespace()
        .collect();

    let picks: Vec<&str> = (0..args.count).map(|_| god.word(&vocab)).collect();
    let weights: Vec<u64> = (0..args.count).map(|_| god.bits(10) + 1).collect();
    let pct = percents(&weights);

    let mut rows: Vec<(String, u64)> = picks
        .iter()
        .zip(pct)
        .map(|(w, p)| (shout(w, args.tr), p))
        .collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));

    let width = rows.iter().map(|(w, _)| w.chars().count()).max().unwrap_or(0);
    println!();
    for (i, (w, p)) in rows.iter().enumerate() {
        let pad = " ".repeat(width - w.chars().count());
        let color = if i == 0 { st.hot } else { "" };
        let off = if i == 0 { st.off } else { "" };
        println!("  {color}{w}{off}{pad}   {p:>3}%");
    }

    // The date comes from the same entropy: a day inside the horizon.
    let offset = god.bits(10) as i64 % date::HORIZON_DAYS;
    let when = date::iso(date::today() + offset);
    let unit = if args.tr { "gün" } else { "days" };
    println!();
    println!(
        "  {}{when}{}  {}(+{offset} {unit}){}",
        st.hot, st.off, st.dim, st.off
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vocabularies_are_the_templeos_size() {
        assert_eq!(VOCAB_EN.split_whitespace().count(), 7569);
        assert_eq!(VOCAB_TR.split_whitespace().count(), 7569);
    }

    #[test]
    fn turkish_shouting_keeps_the_dot() {
        assert_eq!(shout("iyi", true), "\u{0130}Y\u{0130}");
        assert_eq!(shout("kaygı", true), "KAYGI");
        assert_eq!(shout("light", false), "LIGHT");
    }
}
