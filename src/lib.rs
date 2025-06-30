use std::io::{stdin, IsTerminal};
#[cfg(feature = "sync")]
use std::io::Read;

#[cfg(feature = "async")]
use tokio::io::{stdin as async_stdin, AsyncReadExt};

pub use arboard::Clipboard;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("clipboard: {0}")]
    Clipboard(#[from] arboard::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "sync")]
pub mod sync {
    use super::*;
    
    pub fn get_text_from_stdin_or_clipboard() -> Result<(String, Option<Clipboard>), Error> {
        if stdin().is_terminal() {
            let mut cb = Clipboard::new()?;
            Ok((cb.get_text()?.trim().to_owned(), Some(cb)))
        } else {
            let mut buf = String::new();
            stdin().read_to_string(&mut buf)?;
            Ok((buf.trim().to_owned(), Clipboard::new().ok()))
        }
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
        Ok((buf.trim().to_owned(), Clipboard::new().ok()))
    }
}