//! The drawn words read back as a sentence. Nothing here comes from TempleOS;
//! the phrasing is ours, the template choice is still God's.

pub const WRAP: usize = 72;

/// "A, B and C", with the last joiner spelled out.
fn join(words: &[&str], and: &str) -> String {
    match words {
        [] => String::new(),
        [a] => a.to_string(),
        [rest @ .., last] => format!("{} {and} {last}", rest.join(", ")),
    }
}

/// Wrap on spaces at `width`, indenting every line by two.
pub fn wrap(text: &str, width: usize) -> String {
    let mut out = String::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            out.push_str("  ");
            out.push_str(&line);
            out.push('\n');
            line.clear();
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        out.push_str("  ");
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// One reading in prose. `pick` is drawn from God like everything else.
pub fn reading(pick: u64, rows: &[(String, u64)], date: &str, offset: i64, tr: bool) -> String {
    let (w0, p0) = (&rows[0].0, rows[0].1);

    if rows.len() == 1 {
        let body = if tr {
            format!(
                "{w0} tek başına duruyor, yüzde {p0}. Verilen tarih {date}, {offset} gün sonrası."
            )
        } else {
            format!("{w0} stands alone at {p0}%. The hour reads {date}, {offset} days out.")
        };
        return wrap(&body, WRAP);
    }

    let (w1, p1) = (&rows[1].0, rows[1].1);
    let rest: Vec<&str> = rows[2..].iter().map(|(w, _)| w.as_str()).collect();
    let tail = if rest.is_empty() {
        String::new()
    } else if tr {
        format!(" {} kenarda bekliyor.", join(&rest, "ve"))
    } else {
        let verb = if rest.len() == 1 { "waits" } else { "wait" };
        format!(" {} {verb} at the edge.", join(&rest, "and"))
    };

    let body = match (tr, pick % 3) {
        (false, 0) => format!(
            "The weight falls on {w0} at {p0}%, with {w1} behind it at {p1}%.{tail} \
             The hour reads {date}, {offset} days out."
        ),
        (false, 1) => format!(
            "{w0} carries this one ({p0}%); {w1} answers at {p1}%.{tail} \
             Look for it around {date}."
        ),
        (false, _) => format!(
            "Read it as {w0} first, {p0} parts in a hundred, then {w1} at {p1}.{tail} \
             The date given is {date}."
        ),
        (true, 0) => format!(
            "Ağırlık {w0} üzerinde, yüzde {p0}; arkasında yüzde {p1} ile {w1} var.{tail} \
             Zamanı {date}, {offset} gün sonrası."
        ),
        (true, 1) => format!(
            "Bu okumayı {w0} taşıyor (yüzde {p0}); {w1} ona yüzde {p1} ile karşılık \
             veriyor.{tail} {date} civarına bak."
        ),
        (true, _) => format!(
            "Önce {w0}, yüzde {p0}; sonra yüzde {p1} ile {w1}.{tail} \
             Verilen tarih {date}."
        ),
    };
    wrap(&body, WRAP)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(n: usize) -> Vec<(String, u64)> {
        ["LIGHT", "STONE", "WAIT", "BURN"][..n]
            .iter()
            .enumerate()
            .map(|(i, w)| (w.to_string(), (40 - 10 * i) as u64))
            .collect()
    }

    #[test]
    fn every_template_names_every_word() {
        for tr in [false, true] {
            for pick in 0..3u64 {
                for n in 1..=4 {
                    let r = rows(n);
                    let out = reading(pick, &r, "2027-07-11", 253, tr);
                    for (w, _) in &r {
                        assert!(out.contains(w.as_str()), "{tr} {pick} {n}: {out}");
                    }
                    assert!(out.contains("2027-07-11"), "{tr} {pick} {n}: {out}");
                }
            }
        }
    }

    #[test]
    fn lines_stay_inside_the_wrap() {
        let out = reading(0, &rows(4), "2027-07-11", 253, false);
        for line in out.lines() {
            assert!(line.chars().count() <= WRAP + 2, "too long: {line:?}");
        }
        assert!(out.lines().count() > 1, "should wrap at all");
    }

    #[test]
    fn a_long_word_still_gets_its_own_line() {
        let long = "x".repeat(WRAP + 10);
        let out = wrap(&format!("a {long} b"), WRAP);
        assert_eq!(out.lines().count(), 3);
    }
}
