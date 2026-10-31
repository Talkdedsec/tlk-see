//! Port of TempleOS /Adam/God/GodBible.HC (BibleLine2Verse, BookLines).

use crate::bible_books::{BOOKS, BOOK_LINES};

/// ST_BIBLE_LINES: last book-line entry minus one, as BibleInit computes it.
pub const BIBLE_LINES: usize = BOOK_LINES[BOOK_LINES.len() - 1] - 1;

/// Leading "chapter:verse" of a line, if it has one.
fn verse_tag(s: &str) -> Option<&str> {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == 0 || b.get(i) != Some(&b':') {
        return None;
    }
    let mut j = i + 1;
    while j < b.len() && b[j].is_ascii_digit() {
        j += 1;
    }
    if j == i + 1 {
        None
    } else {
        Some(&s[..j])
    }
}

/// Book index and its first line, for a one-based line number.
fn book_of(line: usize) -> Option<(usize, usize)> {
    if line < BOOK_LINES[0] {
        return None;
    }
    let mut i = 0;
    while i + 1 < BOOK_LINES.len() && line >= BOOK_LINES[i + 1] {
        i += 1;
    }
    if i < BOOKS.len() {
        Some((i, BOOK_LINES[i]))
    } else {
        None
    }
}

/// BibleLine2Verse: "Book chapter:verse" for a line, or just "Book" before the
/// first verse of it. None in the Gutenberg front matter, same as TempleOS.
pub fn line2verse(lines: &[&str], line: usize) -> Option<String> {
    let (bi, start) = book_of(line)?;
    let book = BOOKS[bi];

    // Forward to the first non-blank line, like the doc-entry walk there.
    let mut i = line - 1;
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    if let Some(tag) = lines.get(i).and_then(|l| verse_tag(l)) {
        return Some(format!("{book} {tag}"));
    }
    // Otherwise the nearest verse tag above, without leaving the book.
    let mut j = i;
    while j > start - 1 {
        j -= 1;
        if let Some(tag) = lines.get(j).and_then(|l| verse_tag(l)) {
            return Some(format!("{book} {tag}"));
        }
    }
    Some(book.to_string())
}

/// BookLines: N lines starting at a one-based line number.
pub fn book_lines(lines: &[&str], start: usize, count: usize) -> Vec<String> {
    lines
        .iter()
        .skip(start - 1)
        .take(count)
        .map(|l| l.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bible() -> Vec<&'static str> {
        crate::BIBLE.lines().collect()
    }

    #[test]
    fn book_table_matches_templeos() {
        assert_eq!(BOOKS.len(), 66);
        assert_eq!(BOOK_LINES.len(), 67);
        assert_eq!(BIBLE_LINES, 100110);
    }

    #[test]
    fn known_lines_map_to_known_verses() {
        let b = bible();
        assert_eq!(line2verse(&b, 300).as_deref(), Some("Genesis 1:1"));
        assert_eq!(line2verse(&b, 297).as_deref(), Some("Genesis"));
        assert_eq!(line2verse(&b, 1), None, "Gutenberg header has no book");
        let v = line2verse(&b, 5100).unwrap();
        assert!(v.starts_with("Exodus"), "got {v}");
    }

    #[test]
    fn a_passage_is_twenty_lines() {
        let b = bible();
        assert_eq!(book_lines(&b, 300, 20).len(), 20);
        // The last legal draw must still yield a full passage.
        assert_eq!(book_lines(&b, BIBLE_LINES - 19, 20).len(), 20);
    }
}
