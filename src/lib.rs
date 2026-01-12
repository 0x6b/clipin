//! Get text from clipboard or stdin.
//!
//! This crate provides a single function [`get`] that reads text from the system clipboard
//! (if running in a terminal) or stdin (if piped). Returns trimmed text and optionally a
//! [`Clipboard`] instance for further operations.
//!
//! # Features
//!
//! Enable exactly one feature:
//! - `sync` - Synchronous API
//! - `async` - Asynchronous API (requires tokio runtime)

#[cfg(feature = "sync")]
use std::io::Read;
#[cfg(any(feature = "async", feature = "sync"))]
use std::io::{stdin, IsTerminal};
use std::{borrow::Cow, io};

use thiserror::Error;
#[cfg(feature = "async")]
use tokio::io::{stdin as async_stdin, AsyncReadExt};

/// Errors that can occur when reading from stdin or clipboard.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to access or read from the system clipboard.
    #[error("clipboard: {0}")]
    Clipboard(#[from] arboard::Error),
    /// Failed to read from stdin.
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

/// Gets text from clipboard or stdin.
///
/// Reads from the system clipboard if stdin is a terminal, returning the [`Clipboard`]
/// instance for further operations. If stdin is piped, reads from stdin and returns
/// `None` for the clipboard.
///
/// The returned text is trimmed of leading and trailing whitespace.
///
/// # Errors
///
/// Returns [`Error::Clipboard`] if clipboard access fails, or [`Error::Io`] if reading
/// from stdin fails.
#[cfg(feature = "sync")]
pub fn get() -> Result<(String, Option<Clipboard>), Error> {
    if stdin().is_terminal() {
        let mut cb = Clipboard(arboard::Clipboard::new()?);
        Ok((cb.get_text()?.trim().to_owned(), Some(cb)))
    } else {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        Ok((buf.trim().to_owned(), None))
    }
}

/// Gets text from clipboard or stdin.
///
/// Reads from the system clipboard if stdin is a terminal, returning the [`Clipboard`]
/// instance for further operations. If stdin is piped, reads from stdin and returns
/// `None` for the clipboard.
///
/// The returned text is trimmed of leading and trailing whitespace.
///
/// # Errors
///
/// Returns [`Error::Clipboard`] if clipboard access fails, or [`Error::Io`] if reading
/// from stdin fails.
#[cfg(feature = "async")]
pub async fn get() -> Result<(String, Option<Clipboard>), Error> {
    if stdin().is_terminal() {
        let mut cb = Clipboard(arboard::Clipboard::new()?);
        Ok((cb.get_text()?.trim().to_owned(), Some(cb)))
    } else {
        let mut buf = String::new();
        async_stdin().read_to_string(&mut buf).await?;
        Ok((buf.trim().to_owned(), None))
    }
}
