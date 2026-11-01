//! Raw terminal input, so one keystroke is one draw everywhere.
//!
//! Windows reads the console directly. Elsewhere `stty` does the work:
//! shelling out beats carrying a libc dependency and a termios struct whose
//! layout differs per platform, and if `stty` is missing we simply stay in
//! line mode.

#[cfg(windows)]
mod imp {
    pub fn raw(_on: bool) {}
    pub fn restore() {}

    /// One keystroke, unbuffered and unechoed, as GetChar feeds GodDoodle.
    pub fn read_key() -> u8 {
        #[link(name = "msvcrt")]
        extern "C" {
            fn _getch() -> i32;
        }
        let ch = unsafe { _getch() };
        if (0..=255).contains(&ch) {
            ch as u8
        } else {
            0
        }
    }
}

#[cfg(not(windows))]
mod imp {
    use std::io::Read;
    use std::process::Command;
    use std::sync::OnceLock;

    static SAVED: OnceLock<String> = OnceLock::new();

    fn stty(args: &str) -> Option<String> {
        let out = Command::new("sh")
            .arg("-c")
            .arg(format!("stty {args} </dev/tty"))
            .output()
            .ok()?;
        if out.status.success() {
            Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
        } else {
            None
        }
    }

    pub fn raw(on: bool) {
        if !on {
            return;
        }
        if SAVED.get().is_none() {
            match stty("-g") {
                Some(s) => {
                    let _ = SAVED.set(s);
                }
                // No tty to save: leave the terminal alone and read lines.
                None => return,
            }
        }
        stty("raw -echo");
    }

    pub fn restore() {
        if let Some(saved) = SAVED.get() {
            stty(&format!("\"{saved}\""));
        }
    }

    /// One byte if the terminal is raw, otherwise a whole line.
    pub fn read_key() -> u8 {
        let mut b = [0u8; 1];
        match std::io::stdin().read(&mut b) {
            Ok(1) => b[0],
            _ => 0,
        }
    }
}

pub use imp::{raw, read_key, restore};

/// ESC and Ctrl-C never reach the runtime from a raw terminal, so they quit.
pub fn is_quit(key: u8) -> bool {
    key == 3 || key == 27
}
