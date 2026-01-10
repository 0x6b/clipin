//! Get text from stdin or the clipboard.
//!
//! Note: returned text is trimmed (leading/trailing whitespace removed). When stdin is used,
//! the clipboard value is `None`.
//!
//! Note: `async` and `sync` are mutually exclusive; enable exactly one feature.

use std::io::{self, stdin, IsTerminal};
#[cfg(feature = "sync")]
use std::io::Read;

pub use arboard::Clipboard;
use thiserror::Error;
#[cfg(feature = "async")]
use tokio::io::{stdin as async_stdin, AsyncReadExt};

#[derive(Error, Debug)]
pub enum Error {
    #[error("clipboard: {0}")]
    Clipboard(#[from] arboard::Error),
    #[error("io: {0}")]
    Io(#[from] io::Error),
}

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("features \"async\" and \"sync\" are mutually exclusive; enable exactly one.");

#[cfg(feature = "sync")]
pub fn get_text_from_stdin_or_clipboard() -> Result<(String, Option<Clipboard>), Error> {
    if stdin().is_terminal() {
        let mut cb = Clipboard::new()?;
        Ok((cb.get_text()?.trim().to_owned(), Some(cb)))
    } else {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        Ok((buf.trim().to_owned(), None))
    }
}

#[cfg(feature = "async")]
pub async fn get_text_from_stdin_or_clipboard() -> Result<(String, Option<Clipboard>), Error> {
    if stdin().is_terminal() {
        let mut cb = Clipboard::new()?;
        Ok((cb.get_text()?.trim().to_owned(), Some(cb)))
    } else {
        let mut buf = String::new();
        async_stdin().read_to_string(&mut buf).await?;
        Ok((buf.trim().to_owned(), None))
    }
}
