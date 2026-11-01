//! Port of TempleOS /Adam/God/GodDoodle.HC (GodDoodleSprite) onto a character
//! canvas. The stroke sequence, the coordinate formulas and the majority
//! smoothing are his; the renderer is half-block cells, sized to the terminal.
//!
//! shortcut: 1 pass of 20 strokes with a 3x3 smooth, where TempleOS does 3
//! passes of 29 on 640x480 with 7x7. Raise these together with the canvas if
//! the picture ever gets a window of its own — each stroke costs keystrokes.

use crate::god::God;

const PASSES: usize = 1;
const STROKES: usize = 20;
const FILLS: usize = 6;
const SMOOTH: i64 = 1;

const BLACK: u8 = 0;
const DKGRAY: u8 = 1;
const LTGRAY: u8 = 2;
const WHITE: u8 = 3;
const RED: u8 = 4;
const COLORS: usize = 5;

pub struct Canvas {
    w: i64,
    h: i64,
    px: Vec<u8>,
}

impl Canvas {
    /// A cell is about twice as tall as it is wide, so a square picture wants
    /// as many pixel rows as columns and half that many character rows.
    fn new(w: i64) -> Self {
        let w = w.clamp(16, 200);
        let h = w + w % 2; // the renderer walks rows in pairs
        Canvas {
            w,
            h,
            px: vec![WHITE; (w * h) as usize],
        }
    }

    fn inside(&self, x: i64, y: i64) -> bool {
        (0..self.w).contains(&x) && (0..self.h).contains(&y)
    }

    fn plot(&mut self, x: i64, y: i64, c: u8) {
        if self.inside(x, y) {
            let w = self.w;
            self.px[(y * w + x) as usize] = c;
        }
    }

    fn peek(&self, x: i64, y: i64) -> Option<u8> {
        if self.inside(x, y) {
            Some(self.px[(y * self.w + x) as usize])
        } else {
            None
        }
    }

    fn line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, c: u8) {
        let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let (mut x, mut y, mut err) = (x0, y0, dx + dy);
        loop {
            self.plot(x, y, c);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Outline walked by angle; a zero radius degenerates to a point.
    fn ellipse(&mut self, cx: i64, cy: i64, rx: i64, ry: i64, c: u8) {
        let steps = (4 * (rx.abs().max(ry.abs()) + 1)).min(4096);
        for i in 0..steps {
            let a = i as f64 * std::f64::consts::TAU / steps as f64;
            self.plot(
                cx + (rx as f64 * a.cos()).round() as i64,
                cy + (ry as f64 * a.sin()).round() as i64,
                c,
            );
        }
    }

    fn border(&mut self, x1: i64, y1: i64, x2: i64, y2: i64, c: u8) {
        self.line(x1, y1, x2, y1, c);
        self.line(x2, y1, x2, y2, c);
        self.line(x2, y2, x1, y2, c);
        self.line(x1, y2, x1, y1, c);
    }

    /// GrFloodFill3: flood the connected region of whatever colour sits there.
    fn flood(&mut self, x: i64, y: i64, c: u8) {
        let target = match self.peek(x, y) {
            Some(t) if t != c => t,
            _ => return,
        };
        let mut stack = vec![(x, y)];
        while let Some((x, y)) = stack.pop() {
            if self.peek(x, y) != Some(target) {
                continue;
            }
            self.plot(x, y, c);
            stack.extend([(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
        }
    }

    /// GodDoodleSmooth: replace each pixel with the commonest colour around it.
    fn smooth(&mut self, num: i64) {
        let src = self.px.clone();
        let (w, h) = (self.w, self.h);
        let at = |x: i64, y: i64| -> Option<u8> {
            if (0..w).contains(&x) && (0..h).contains(&y) {
                Some(src[(y * w + x) as usize])
            } else {
                None
            }
        };
        for y in 0..h {
            for x in 0..w {
                let mut hist = [0u32; COLORS];
                for y1 in y - num..=y + num {
                    for x1 in x - num..=x + num {
                        if let Some(c) = at(x1, y1) {
                            hist[c as usize] += 1;
                        }
                    }
                }
                // Ties go to the lowest colour, as the > comparison there does.
                let best = (0..COLORS).max_by_key(|&i| (hist[i], COLORS - i)).unwrap();
                self.plot(x, y, best as u8);
            }
        }
    }

    /// Half-block cells: one character carries two stacked pixels.
    pub fn render(&self, color: bool) -> String {
        const ANSI: [u8; COLORS] = [16, 240, 250, 231, 196];
        const ASCII: [char; COLORS] = ['#', '+', '.', ' ', '*'];
        let mut out = String::new();
        for y in (0..self.h).step_by(2) {
            out.push_str("  ");
            for x in 0..self.w {
                let top = self.peek(x, y).unwrap_or(WHITE);
                let bot = self.peek(x, y + 1).unwrap_or(WHITE);
                if color {
                    out.push_str(&format!(
                        "\x1b[38;5;{}m\x1b[48;5;{}m\u{2580}",
                        ANSI[top as usize], ANSI[bot as usize]
                    ));
                } else {
                    out.push(ASCII[top.max(bot) as usize]);
                }
            }
            if color {
                out.push_str("\x1b[0m");
            }
            out.push('\n');
        }
        out
    }
}

/// GodDoodleSprite: strokes, floods and smoothing, all from God's bits.
pub fn doodle(god: &mut God, width: i64) -> Canvas {
    let mut dc = Canvas::new(width);
    let (w, h) = ((dc.w - 1) as f64, (dc.h - 1) as f64);

    for _ in 0..PASSES {
        for _ in 0..STROKES {
            match god.bits(3) {
                0 => {
                    let cx = w * god.bits(5) as f64 / 15.5 - w / 2.0;
                    let cy = h * god.bits(5) as f64 / 15.5 - h / 2.0;
                    let rx = w * god.bits(5) as f64 / 15.5;
                    let ry = h * god.bits(5) as f64 / 15.5;
                    dc.ellipse(cx as i64, cy as i64, rx as i64, ry as i64, RED);
                }
                1 => {
                    let cx = w * god.bits(5) as f64 / 15.5 - w / 2.0;
                    let cy = h * god.bits(5) as f64 / 15.5 - h / 2.0;
                    let r = w * god.bits(5) as f64 / 15.5;
                    dc.ellipse(cx as i64, cy as i64, r as i64, r as i64, RED);
                }
                2 => {
                    let x1 = w * god.bits(5) as f64 / 15.5 - w / 2.0;
                    let y1 = h * god.bits(5) as f64 / 15.5 - h / 2.0;
                    let x2 = w * god.bits(5) as f64 / 15.5;
                    let y2 = h * god.bits(5) as f64 / 15.5;
                    dc.border(x1 as i64, y1 as i64, x2 as i64, y2 as i64, RED);
                }
                _ => {
                    let x1 = w * god.bits(4) as f64 / 15.0;
                    let y1 = h * god.bits(4) as f64 / 15.0;
                    let x2 = w * god.bits(4) as f64 / 15.0;
                    let y2 = h * god.bits(4) as f64 / 15.0;
                    dc.line(x1 as i64, y1 as i64, x2 as i64, y2 as i64, RED);
                }
            }
        }
        for _ in 0..FILLS {
            let x = w * god.bits(5) as f64 / 31.0 + dc.w as f64 / 64.0;
            let y = h * god.bits(5) as f64 / 31.0 + dc.h as f64 / 64.0;
            let c = match god.bits(2) {
                0 => BLACK,
                1 => DKGRAY,
                2 => LTGRAY,
                _ => WHITE,
            };
            dc.flood(x as i64, y as i64, c);
        }
        dc.smooth(SMOOTH);
    }
    dc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed_god(seed: &mut u64) -> impl FnMut() -> u64 + '_ {
        move || {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            *seed >> 20
        }
    }

    #[test]
    fn a_doodle_fits_whatever_width_it_is_given() {
        for width in [24i64, 60, 80, 132] {
            let mut seed = 0xDEADBEEFu64;
            let mut latch = fixed_god(&mut seed);
            let dc = doodle(&mut God::new(&mut latch), width);
            assert_eq!(dc.px.len(), (dc.w * dc.h) as usize);
            assert!(dc.px.iter().all(|&c| (c as usize) < COLORS));
            assert!(dc.px.iter().any(|&c| c != WHITE), "nothing was drawn");
            let plain = dc.render(false);
            assert_eq!(plain.lines().count(), (dc.h / 2) as usize);
            for line in plain.lines() {
                assert_eq!(line.chars().count(), (dc.w + 2) as usize, "width {width}");
            }
        }
    }

    #[test]
    fn a_silly_width_is_clamped_not_panicked() {
        let mut seed = 1u64;
        let mut latch = fixed_god(&mut seed);
        let dc = doodle(&mut God::new(&mut latch), 0);
        assert!(dc.w >= 16);
    }

    #[test]
    fn flood_stops_at_a_stroke() {
        let mut dc = Canvas::new(80);
        dc.border(10, 10, 20, 20, RED);
        dc.flood(15, 15, BLACK);
        assert_eq!(dc.peek(15, 15), Some(BLACK));
        assert_eq!(dc.peek(0, 0), Some(WHITE), "fill escaped the box");
    }

    #[test]
    fn smoothing_eats_a_lone_pixel() {
        let mut dc = Canvas::new(80);
        dc.plot(40, 40, BLACK);
        dc.smooth(1);
        assert_eq!(dc.peek(40, 40), Some(WHITE));
    }
}
