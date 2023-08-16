#[cfg(test)]
mod test;

mod accel;
mod num_traits;
mod point;
mod tensor;
mod texture;
mod var;
mod vector;

use num_traits::*;
pub use point::*;
pub use var::*;
pub use vector::*;

use once_cell::sync::Lazy;

static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());

pub fn set_backend<'a>(backends: impl IntoIterator<Item = impl AsRef<str>>) -> rjit::Result<()> {
    TRACE.set_backend(backends)
}
pub fn eval() {
    TRACE.eval().unwrap();
}
