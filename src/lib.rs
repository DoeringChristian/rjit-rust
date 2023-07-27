mod autogen;
#[cfg(test)]
mod test;

use once_cell::sync::Lazy;

pub use autogen::*;

pub fn set_backend<'a>(backends: impl IntoIterator<Item = impl AsRef<str>>) -> rjit::Result<()> {
    TRACE.set_backend(backends)
}
