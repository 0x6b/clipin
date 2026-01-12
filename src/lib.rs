//! Get text from stdin or clipboard.
//!
//! Returns trimmed text and optionally the clipboard instance. When stdin is used, clipboard is
//! `None`.
//!
//! Enable exactly one feature: `async` or `sync`.

#[cfg(feature = "sync")]
use std::io::Read;
#[cfg(any(feature = "async", feature = "sync"))]
use std::io::{stdin, IsTerminal};
use std::{borrow::Cow, io};

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

/// A wrapper around the system clipboard.
pub struct Clipboard(arboard::Clipboard);

impl Clipboard {
    /// Gets the current text contents of the clipboard.
    pub fn get_text(&mut self) -> Result<String, Error> {
        Ok(self.0.get_text()?)
    }

    /// Sets the text contents of the clipboard.
    pub fn set_text<'a, T>(&mut self, text: T) -> Result<(), Error>
    where
        T: Into<Cow<'a, str>>,
    {
        Ok(self.0.set_text(text)?)
    }

    /// Sets the HTML contents of the clipboard, with an optional plain-text alternative.
    pub fn set_html<'a, T>(&mut self, html: T, alt_text: Option<T>) -> Result<(), Error>
    where
        T: Into<Cow<'a, str>>,
    {
        Ok(self.0.set_html(html, alt_text)?)
    }
}

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!("features \"async\" and \"sync\" are mutually exclusive; enable exactly one.");

#[cfg(feature = "sync")]
pub fn get_text_from_stdin_or_clipboard() -> Result<(String, Option<Clipboard>), Error> {
    if stdin().is_terminal() {
        let mut cb = Clipboard(arboard::Clipboard::new()?);
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
        let mut cb = Clipboard(arboard::Clipboard::new()?);
        Ok((cb.get_text()?.trim().to_owned(), Some(cb)))
    } else {
        let mut buf = String::new();
        async_stdin().read_to_string(&mut buf).await?;
        Ok((buf.trim().to_owned(), None))
    }
}
