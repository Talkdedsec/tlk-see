//! Calendar arithmetic for the predicted date. No dependency: days-to-civil is
//! Hinnant's algorithm, and "today" is UTC, which is what the system clock gives.

use std::time::{SystemTime, UNIX_EPOCH};

pub const HORIZON_DAYS: i64 = 365;

pub fn today() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| (d.as_secs() / 86_400) as i64)
        .unwrap_or(0)
}

/// Days since 1970-01-01 to (year, month, day).
pub fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as i64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn iso(days: i64) -> String {
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_days_map_to_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(11_016), (2000, 2, 29), "leap day");
        assert_eq!(civil_from_days(19_723), (2024, 1, 1));
        assert_eq!(iso(19_723), "2024-01-01");
        assert_eq!(civil_from_days(-1), (1969, 12, 31));
    }

    #[test]
    fn today_is_in_this_century() {
        let (y, _, _) = civil_from_days(today());
        assert!((2020..2100).contains(&y), "clock says {y}");
    }
}
