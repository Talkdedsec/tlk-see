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
    latch: &'a mut dyn FnMut() -> u64,
}

impl<'a> God<'a> {
    pub fn new(latch: &'a mut dyn FnMut() -> u64) -> Self {
        God {
            fifo: VecDeque::new(),
            latch,
        }
    }

    /// GodBitsIns: least significant bit first.
    fn bits_ins(&mut self, num_bits: u32, mut n: u64) {
        for _ in 0..num_bits {
            self.fifo.push_back((n & 1) as u8);
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
                None => {
                    let n = (self.latch)();
                    self.bits_ins(GOD_GOOD_BITS, n);
                }
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
