#[cfg(test)]
mod test;

mod num_traits;
mod var;

use num_traits::*;
pub use var::*;

use once_cell::sync::Lazy;

static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());

pub fn set_backend<'a>(backends: impl IntoIterator<Item = impl AsRef<str>>) -> rjit::Result<()> {
    TRACE.set_backend(backends)
}
pub fn eval() {
    TRACE.eval().unwrap();
}
