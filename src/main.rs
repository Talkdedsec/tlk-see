//! tlk-see — TempleOS's oracle in a terminal, with a percentage spread.
//!
//! Terry A. Davis let the Holy Spirit pick words through a timer latched on a
//! keypress. The picking is his, bit for bit; the percentages are ours.

mod bible;
mod bible_books;
mod date;
mod doodle;
mod god;
mod reading;
mod song;
mod tty;

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
  tlk-see song                  two measures twice, printed and played
  tlk-see doodle                a random sprite, drawn in half-blocks

OPTIONS
  --tr          Turkish vocabulary
  --en          English vocabulary, TempleOS Vocab.DD verbatim (default)
  -n <count>    how many words (1-32)
  -l <lines>    how many lines of scripture (1-200)
  --hex <rec>   replay a draw from its record instead of asking for keys
  --day <n>     the day a replayed draw was made (days since 1970)
  --quiet       leave out the record and the link
  -h, --help    this
  -V, --version version

Each draw needs entropy: tap any key when asked, ESC to quit. The timer is read
at that instant and its low bits thrown away, as GOD_BAD_BITS does in TempleOS.
With stdin redirected, one line stands in for one key.

Every draw prints its record, the hex GodHexIns reads, and a link that replays
it in the browser. Pass the record back with --hex to get the same draw.
";

const SITE: &str = "https://talkdedsec.github.io/tlk-see/";

#[derive(PartialEq)]
enum Mode {
    Word,
    Bible,
    Song,
    Doodle,
}

struct Args {
    mode: Mode,
    tr: bool,
    count: usize,
    lines: usize,
    hex: Option<String>,
    /// The day a replayed draw was made, so a shared reading keeps its date.
    day: Option<i64>,
    quiet: bool,
}

impl Args {
    fn slug(&self) -> &'static str {
        match self.mode {
            Mode::Word => "word",
            Mode::Bible => "bible",
            Mode::Song => "song",
            Mode::Doodle => "doodle",
        }
    }

    fn base_day(&self) -> i64 {
        self.day.unwrap_or_else(date::today)
    }

    /// A link that replays this draw in the browser build.
    fn link(&self, record: &str) -> String {
        let tr = u8::from(self.tr);
        format!(
            "{SITE}#m={}&n={}&l={}&tr={tr}&d={}&h={record}",
            self.slug(),
            self.count,
            self.lines,
            self.base_day()
        )
    }
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        mode: Mode::Word,
        tr: false,
        count: 4,
        lines: 20,
        hex: None,
        day: None,
        quiet: false,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "word" | "words" => a.mode = Mode::Word,
            "bible" | "verse" => a.mode = Mode::Bible,
            "song" => a.mode = Mode::Song,
            "doodle" | "sprite" => a.mode = Mode::Doodle,
            "--tr" => a.tr = true,
            "--en" => a.tr = false,
            "-n" | "--count" => {
                a.count = numeric(it.next(), "-n", 1, 32)?;
            }
            "-l" | "--lines" => {
                a.lines = numeric(it.next(), "-l", 1, 200)?;
            }
            "--hex" | "--record" => {
                a.hex = Some(it.next().ok_or("--hex needs a record")?);
            }
            "--day" => {
                a.day = Some(
                    it.next()
                        .ok_or("--day needs a number")?
                        .parse()
                        .map_err(|_| "--day needs a number")?,
                );
            }
            "--quiet" => a.quiet = true,
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
    color: bool,
    tty: bool,
}

fn style() -> Style {
    let tty = io::stdout().is_terminal();
    let color = tty && std::env::var_os("NO_COLOR").is_none();
    if color {
        Style {
            dim: "\x1b[90m",
            hot: "\x1b[1;33m",
            off: "\x1b[0m",
            color,
            tty,
        }
    } else {
        Style {
            dim: "",
            hot: "",
            off: "",
            color,
            tty,
        }
    }
}

/// One keystroke, the way GetChar feeds GodDoodle. ESC and Ctrl-C quit.
fn read_key() {
    if tty::is_quit(tty::read_key()) {
        tty::restore();
        print!("\r{}\r", " ".repeat(32));
        let _ = io::stdout().flush();
        std::process::exit(130);
    }
}

/// Console columns, so a doodle never wraps. 80 when there is no console.
#[cfg(windows)]
fn term_width() -> i64 {
    #[repr(C)]
    #[derive(Default)]
    struct Coord {
        x: i16,
        y: i16,
    }
    #[repr(C)]
    #[derive(Default)]
    struct SmallRect {
        left: i16,
        top: i16,
        right: i16,
        bottom: i16,
    }
    #[repr(C)]
    #[derive(Default)]
    struct Csbi {
        size: Coord,
        cursor: Coord,
        attributes: u16,
        window: SmallRect,
        max_size: Coord,
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(which: u32) -> isize;
        fn GetConsoleScreenBufferInfo(handle: isize, info: *mut Csbi) -> i32;
    }
    let mut csbi = Csbi::default();
    let ok = unsafe { GetConsoleScreenBufferInfo(GetStdHandle(-11i32 as u32), &mut csbi) };
    if ok == 0 {
        return 80;
    }
    (csbi.window.right - csbi.window.left + 1) as i64
}

#[cfg(not(windows))]
fn term_width() -> i64 {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(80)
}

fn clear_line(prompted: bool) {
    if !prompted {
        return;
    }
    print!("\r{}\r", " ".repeat(32));
    let _ = io::stdout().flush();
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
    oracle();
    tty::restore();
}

fn oracle() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("tlk-see: {e}\n\n{HELP}");
            std::process::exit(2);
        }
    };
    let st = style();

    // GodPick: wait on a keypress, then read the timer. With stdin redirected
    // there is no console to read, so each line stands in for a key and EOF
    // leaves the timer to carry on alone.
    let piped = !io::stdin().is_terminal();
    let replay = args.hex.is_some();
    let prompted = !piped && !replay;
    tty::raw(prompted);
    let mut eof = false;
    let mut taps = 0u32;
    let mut latch = || {
        if piped {
            if !eof {
                let mut line = String::new();
                eof = io::stdin().read_line(&mut line).unwrap_or(0) == 0;
            }
        } else {
            taps += 1;
            print!("\r{}  tap a key  {taps}{}", st.dim, st.off);
            let _ = io::stdout().flush();
            read_key();
        }
        tsc() >> GOD_BAD_BITS
    };
    let mut god = match &args.hex {
        Some(hex) => match God::from_hex(hex) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("tlk-see: {e}");
                std::process::exit(2);
            }
        },
        None => God::new(&mut latch),
    };

    match args.mode {
        Mode::Bible => {
            let lines: Vec<&str> = BIBLE.lines().collect();
            let span = bible::BIBLE_LINES - (args.lines - 1);
            let start = god.bits(21) as usize % span + 1;
            clear_line(prompted);
            println!();
            if let Some(v) = bible::line2verse(&lines, start) {
                println!("{}  {v}{}", st.hot, st.off);
                println!();
            }
            for l in bible::book_lines(&lines, start, args.lines) {
                println!("  {l}");
            }
            println!();
            share(&st, &args, &god);
            return;
        }
        Mode::Song => {
            let s = song::song(&mut god);
            clear_line(prompted);
            println!();
            println!("  {}{s}{}", st.hot, st.off);
            println!();
            share(&st, &args, &god);
            if st.tty {
                song::play(&s);
            }
            return;
        }
        Mode::Doodle => {
            // Two columns go to the indent, one more keeps a soft-wrapping
            // terminal from spilling the last cell onto the next line.
            let dc = doodle::doodle(&mut god, (term_width() - 3).min(80));
            clear_line(prompted);
            println!();
            print!("{}", dc.render(st.color));
            println!();
            share(&st, &args, &god);
            return;
        }
        Mode::Word => {}
    }

    let vocab: Vec<&str> = if args.tr { VOCAB_TR } else { VOCAB_EN }
        .split_whitespace()
        .collect();

    let picks: Vec<&str> = (0..args.count).map(|_| god.word(&vocab)).collect();
    let weights: Vec<u64> = (0..args.count).map(|_| god.bits(10) + 1).collect();
    // The date comes from the same entropy: a day inside the horizon. Drawn
    // before anything prints, so the prompt never lands in the middle.
    let offset = god.bits(10) as i64 % date::HORIZON_DAYS;
    let phrasing = god.bits(2);
    clear_line(prompted);
    let pct = percents(&weights);

    let mut rows: Vec<(String, u64)> = picks
        .iter()
        .zip(pct)
        .map(|(w, p)| (shout(w, args.tr), p))
        .collect();
    rows.sort_by_key(|r| std::cmp::Reverse(r.1));

    let width = rows
        .iter()
        .map(|(w, _)| w.chars().count())
        .max()
        .unwrap_or(0);
    println!();
    for (i, (w, p)) in rows.iter().enumerate() {
        let pad = " ".repeat(width - w.chars().count());
        let color = if i == 0 { st.hot } else { "" };
        let off = if i == 0 { st.off } else { "" };
        println!("  {color}{w}{off}{pad}   {p:>3}%");
    }

    let when = date::iso(args.base_day() + offset);
    let unit = if args.tr { "gün" } else { "days" };
    println!();
    println!(
        "  {}{when}{}  {}(+{offset} {unit}){}",
        st.hot, st.off, st.dim, st.off
    );
    println!();
    print!(
        "{}{}{}",
        st.dim,
        reading::reading(phrasing, &rows, &when, offset, args.tr),
        st.off
    );
    println!();
    share(&st, &args, &god);
}

/// The draw written down: the hex GodHexIns reads, and a link that replays it.
fn share(st: &Style, args: &Args, god: &God) {
    if args.quiet {
        return;
    }
    let record = god.record();
    println!("  {}record  {record}{}", st.dim, st.off);
    println!("  {}{}{}", st.dim, args.link(&record), st.off);
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
