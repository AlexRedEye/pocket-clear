use std::io::{self, Write};

#[cfg(windows)]
mod windows;

pub fn clear() -> io::Result<()> {
    #[cfg(windows)]
    {
        windows::enable_ansi()?;
    }

    let mut out = io::stdout();
    write!(out, "\x1B[2J\x1B[H")?;
    out.flush()
}
