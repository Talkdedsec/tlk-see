//! Port of TempleOS /Adam/God/HolySpirit.HC (GodBits/GodWord/GodBiblePassage).
//! Entropy is the CPU timer latched at a keypress with its low bits dropped,
//! exactly as GOD_BAD_BITS/GOD_GOOD_BITS do there.

use std::collections::VecDeque;

pub const GOD_BAD_BITS: u32 = 4;
pub const GOD_GOOD_BITS: u32 = 24;

pub fn tsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe { core::arch::x86_64::_rdtsc() }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0)
    }
}

pub struct God<'a> {
    fifo: VecDeque<u8>,
    /// Every bit ever queued, so a draw can be written down and replayed.
    tape: Vec<u8>,
    latch: Option<&'a mut dyn FnMut() -> u64>,
}

impl<'a> God<'a> {
    pub fn new(latch: &'a mut dyn FnMut() -> u64) -> Self {
        God {
            fifo: VecDeque::new(),
            tape: Vec::new(),
            latch: Some(latch),
        }
    }

    /// GodHexIns: a hex record read back into the queue, four bits a digit.
    /// Replaying needs no keystrokes, so there is no latch behind it.
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let mut god = God {
            fifo: VecDeque::new(),
            tape: Vec::new(),
            latch: None,
        };
        for c in hex.chars().filter(|c| !c.is_whitespace()) {
            let v = c
                .to_digit(16)
                .ok_or_else(|| format!("not a hex record: {c:?}"))?;
            for i in (0..4).rev() {
                let bit = ((v >> i) & 1) as u8;
                god.fifo.push_back(bit);
                god.tape.push(bit);
            }
        }
        if god.fifo.is_empty() {
            return Err("the record is empty".into());
        }
        Ok(god)
    }

    /// The draw so far, as the hex record GodHexIns would read.
    pub fn record(&self) -> String {
        self.tape
            .chunks(4)
            .filter(|c| c.len() == 4)
            .map(|c| {
                let v = c.iter().fold(0u8, |acc, b| acc * 2 + b);
                char::from_digit(v as u32, 16).unwrap_or('0')
            })
            .collect()
    }

    /// GodBitsIns: least significant bit first.
    fn bits_ins(&mut self, num_bits: u32, mut n: u64) {
        for _ in 0..num_bits {
            let bit = (n & 1) as u8;
            self.fifo.push_back(bit);
            self.tape.push(bit);
            n >>= 1;
        }
    }

    /// GodBits: pop N bits, refilling from the latch when the fifo runs dry.
    pub fn bits(&mut self, num_bits: u32) -> u64 {
        let mut res = 0u64;
        let mut left = num_bits;
        while left > 0 {
            match self.fifo.pop_front() {
                Some(b) => {
                    res = (res << 1) + b as u64;
                    left -= 1;
                }
                None => match self.latch.as_mut() {
                    Some(latch) => {
                        let n = latch();
                        self.bits_ins(GOD_GOOD_BITS, n);
                    }
                    None => {
                        eprintln!("tlk-see: the record ran out of bits");
                        std::process::exit(2);
                    }
                },
            }
        }
        res
    }

    /// GodWordStr: 17 bits modulo the vocabulary size.
    pub fn word<'v>(&mut self, vocab: &[&'v str]) -> &'v str {
        vocab[self.bits(17) as usize % vocab.len()]
    }
}

/// Largest-remainder split of weights into whole percents summing to 100.
pub fn percents(weights: &[u64]) -> Vec<u64> {
    let sum: u64 = weights.iter().sum();
    if sum == 0 {
        return vec![0; weights.len()];
    }
    let mut base: Vec<u64> = weights.iter().map(|w| w * 100 / sum).collect();
    let mut rem: Vec<(u64, usize)> = weights
        .iter()
        .enumerate()
        .map(|(i, w)| (w * 100 % sum, i))
        .collect();
    rem.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let mut left = 100 - base.iter().sum::<u64>();
    for (_, i) in rem {
        if left == 0 {
            break;
        }
        base[i] += 1;
        left -= 1;
    }
    base
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_come_back_reversed_like_templeos() {
        // bits go in LSB first and come out MSB first, so 0b1011 reads back as 0b1101.
        let mut latch = || 0b1011u64;
        let mut god = God::new(&mut latch);
        god.bits_ins(4, 0b1011);
        assert_eq!(god.bits(4), 0b1101);
    }

    #[test]
    fn empty_fifo_pulls_from_the_latch() {
        let calls = std::cell::Cell::new(0);
        let mut latch = || {
            calls.set(calls.get() + 1);
            0xABCDEF
        };
        let mut god = God::new(&mut latch);
        let v = god.bits(17);
        assert!(v < 1 << 17);
        assert_eq!(calls.get(), 1, "24 good bits cover one 17-bit draw");
    }

    #[test]
    fn a_record_replays_the_same_draw() {
        let vocab = ["alpha", "beta", "gamma", "delta", "epsilon"];
        let mut seed = 0x1234_5678u64;
        let mut latch = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            seed >> 20
        };
        let mut god = God::new(&mut latch);
        let first: Vec<&str> = (0..5).map(|_| god.word(&vocab)).collect();
        let hex = god.record();
        assert!(!hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit()));

        let mut replay = God::from_hex(&hex).expect("own record should parse");
        let again: Vec<&str> = (0..5).map(|_| replay.word(&vocab)).collect();
        assert_eq!(first, again);
        assert_eq!(replay.record(), hex, "a replay writes the same record");
    }

    #[test]
    fn a_bad_record_is_refused() {
        assert!(God::from_hex("").is_err());
        assert!(God::from_hex("zz").is_err());
        assert!(God::from_hex("00ff").is_ok());
    }

    #[test]
    fn percents_always_total_100() {
        for w in [
            vec![1u64],
            vec![1, 1, 1],
            vec![7, 7, 7, 7, 7, 7, 7],
            vec![1023, 1, 1, 1],
            vec![500, 300, 200],
        ] {
            let p = percents(&w);
            assert_eq!(p.iter().sum::<u64>(), 100, "weights {w:?} -> {p:?}");
            assert_eq!(p.len(), w.len());
        }
    }
}
