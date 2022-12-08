//! Error handling utilities for use cases of the Flexible Project system.

use std::backtrace::Backtrace;
use std::error::Error;

use derive_more::Display;

/// Boxed error type with all required trait bounds.
pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;

/// Custom error type (used for use case internal error).
#[derive(Debug, Display)]
#[display(fmt = "internal error: {}\ncaptured backtrace: {}", source, backtrace)]
pub struct InternalError {
    source: BoxedError,
    backtrace: Backtrace,
}

impl InternalError {
    /// Creates new internal error from almost any error.
    pub fn new<T>(error: T) -> Self
    where
        T: Error + Send + Sync + 'static,
    {
        let source = Box::new(error);
        source.into()
    }
}

impl Error for InternalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl<T> From<Box<T>> for InternalError
where
    T: Error + Send + Sync + 'static,
{
    fn from(source: Box<T>) -> Self {
        (source as BoxedError).into()
    }
}

impl From<BoxedError> for InternalError {
    fn from(source: BoxedError) -> Self {
        Self {
            source,
            backtrace: Backtrace::capture(),
        }
    }
}
