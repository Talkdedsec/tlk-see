//! Port of TempleOS /Adam/God/GodSong.HC (GodSongStr/GodSong) plus playback.
//! Settings are the defaults of that form: Normal complexity, no rests, 4/4,
//! octave 4.

use crate::god::God;

const OCTAVE: i64 = 4;
const NORMAL: [u8; 5] = [DUR_4, DUR_4, DUR_8_8, DUR_3_3_3, DUR_16_16_16_16];

const DUR_4: u8 = 0;
const DUR_8_8: u8 = 1;
const DUR_3_3_3: u8 = 2;
const DUR_16_16_16_16: u8 = 3;

struct Mss {
    octave: i64,
    octave_state: i64,
}

/// InsNote: k is a random note nibble.
fn ins_note(mss: &mut Mss, buf: &mut String, k: u64) {
    let k = k / 2;
    if k < 3 {
        if mss.octave_state != mss.octave {
            mss.octave_state = mss.octave;
            buf.push((b'0' + mss.octave_state as u8) as char);
        }
        buf.push(if k == 0 {
            'G'
        } else {
            (b'A' + k as u8 - 1) as char
        });
    } else {
        if mss.octave_state != mss.octave + 1 {
            mss.octave_state = mss.octave + 1;
            buf.push((b'0' + mss.octave_state as u8) as char);
        }
        buf.push((b'A' + k as u8 - 1) as char);
    }
}

/// GodSongStr: two measures.
fn song_str(god: &mut God) -> String {
    let mut mss = Mss {
        octave: OCTAVE,
        octave_state: OCTAVE + 1,
    };
    let mut buf = String::new();
    buf.push((b'0' + mss.octave_state as u8) as char);
    let mut last = u8::MAX;

    for _ in 0..8 {
        let n = god.bits(8);
        let mut duration = NORMAL[n as usize % 5];
        match duration {
            DUR_8_8 => {
                if last != DUR_8_8 {
                    buf.push('e');
                }
                for _ in 0..2 {
                    let k = god.bits(4);
                    ins_note(&mut mss, &mut buf, k);
                }
            }
            DUR_3_3_3 => {
                if last != DUR_3_3_3 {
                    buf.push('e');
                    buf.push('t');
                }
                for _ in 0..3 {
                    let k = god.bits(4);
                    ins_note(&mut mss, &mut buf, k);
                }
            }
            DUR_16_16_16_16 => {
                if last != DUR_16_16_16_16 {
                    buf.push('s');
                }
                let k = god.bits(4);
                let k2 = god.bits(4);
                ins_note(&mut mss, &mut buf, k);
                ins_note(&mut mss, &mut buf, k2);
                ins_note(&mut mss, &mut buf, k);
                ins_note(&mut mss, &mut buf, k2);
            }
            _ => {
                if last != DUR_4 {
                    buf.push('q');
                }
                let k = god.bits(4);
                ins_note(&mut mss, &mut buf, k);
                duration = DUR_4;
            }
        }
        last = duration;
    }
    buf
}

/// The two halves GodSong repeats.
pub fn song_parts(god: &mut God) -> (String, String) {
    let st1 = song_str(god);
    let st2 = song_str(god);
    (st1, st2)
}

/// GodSong: two measures twice, then two more twice.
pub fn song(god: &mut God) -> String {
    let (st1, st2) = song_parts(god);
    format!("{st1}{st1}{st2}{st2}")
}

/// A note or rest with its length in milliseconds.
pub fn notes(song: &str) -> Vec<(u32, u32)> {
    const SEMITONE: [i32; 7] = [9, 11, 0, 2, 4, 5, 7]; // A B C D E F G
    let mut out = Vec::new();
    let mut octave = 4i32;
    let mut base = 400u32; // a quarter note at 150 BPM
    let mut dotted = false;
    let mut triplet = false;
    for c in song.chars() {
        match c {
            '0'..='9' => octave = c as i32 - '0' as i32,
            'q' => {
                base = 400;
                triplet = false;
                dotted = false;
            }
            'e' => {
                base = 200;
                triplet = false;
                dotted = false;
            }
            's' => {
                base = 100;
                triplet = false;
                dotted = false;
            }
            't' => triplet = true,
            '.' => dotted = true,
            'A'..='G' => {
                let midi = 12 * (octave + 1) + SEMITONE[c as usize - 'A' as usize];
                let freq = (440.0 * 2f64.powf((midi - 69) as f64 / 12.0)).round() as u32;
                let mut ms = base;
                if dotted {
                    ms = ms * 3 / 2;
                }
                if triplet {
                    ms = ms * 2 / 3;
                }
                out.push((freq, ms));
            }
            'R' => out.push((0, base)),
            _ => {}
        }
    }
    out
}

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn Beep(dwfreq: u32, dwduration: u32) -> i32;
}

pub fn play(song: &str) {
    for (freq, ms) in notes(song) {
        if freq == 0 {
            std::thread::sleep(std::time::Duration::from_millis(ms as u64));
            continue;
        }
        #[cfg(windows)]
        unsafe {
            Beep(freq, ms);
        }
        #[cfg(not(windows))]
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_song_is_two_measures_twice_then_two_more_twice() {
        let mut seed = 0x9E3779B97F4A7C15u64;
        let mut latch = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            seed >> 20
        };
        let mut god = God::new(&mut latch);
        let (a, b) = song_parts(&mut god);
        let s = format!("{a}{a}{b}{b}");
        assert!(a.len() > 8 && b.len() > 8, "got {a:?} {b:?}");

        assert!(
            s.chars().all(|c| "0123456789ABCDEFGqest.R".contains(c)),
            "{s:?}"
        );
    }

    #[test]
    fn notes_land_on_concert_pitch() {
        assert_eq!(notes("4A"), vec![(440, 400)]);
        assert_eq!(notes("5A"), vec![(880, 400)]);
        assert_eq!(notes("4e A"), vec![(440, 200)]);
        assert_eq!(notes("4e.A"), vec![(440, 300)]);
        assert_eq!(notes("4etA"), vec![(440, 133)]);
    }
}
