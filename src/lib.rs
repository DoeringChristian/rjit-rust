mod autogen;
#[cfg(test)]
mod test;

use paste::paste;
use std::marker::PhantomData;

use once_cell::sync::Lazy;

pub use autogen::*;

static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());

pub fn set_backend<'a>(backends: impl IntoIterator<Item = impl AsRef<str>>) -> rjit::Result<()> {
    TRACE.set_backend(backends)
}

// pub struct Var<T>(rjit::VarRef, PhantomData<T>);
//
// macro_rules! from_prim {
//     ($prim:ident) => {
//         impl From<$prim> for Var<$prim> {
//             fn from(value: $prim) -> Self {
//                 Self(TRACE.literal(value).unwrap(), PhantomData)
//             }
//         }
//         impl<'a> From<&'a [$prim]> for Var<$prim> {
//             fn from(value: &'a [$prim]) -> Self {
//                 Self(TRACE.array(value).unwrap(), PhantomData)
//             }
//         }
//     };
// }
//
// from_prim!(f32);
//
// impl<Rhs: Into<Var<f32>>> std::ops::Add<Rhs> for Var<f32> {
//     type Output = Var<f32>;
//
//     fn add(self, rhs: Rhs) -> Self::Output {
//         let rhs = rhs.into();
//         Var(self.0.add(&rhs.0).unwrap(), PhantomData)
//     }
// }
//
// macro_rules! bop_sym {
//     ($lhs:ident _ $rhs:ident = $out:ident) => {
//         bop_sym!($lhs Add $rhs = $out);
//         bop_sym!($lhs Sub $rhs = $out);
//         bop_sym!($lhs Mul $rhs = $out);
//         bop_sym!($lhs Div $rhs = $out);
//     };
//     ($lhs:ident $Op:ident $rhs:ident = $out:ident) => {
//         paste! {
//             impl std::ops::$Op<Var<$rhs>> for Var<$lhs> {
//                 type Output = Var<$out>;
//
//                 fn [<$Op:lower>](self, rhs: Var<$rhs>) -> Self::Output {
//                     let lhs = self.0.cast(&rjit::VarType::[<$out:camel>]).unwrap();
//                     let rhs = rhs.0.cast(&rjit::VarType::[<$out:camel>]).unwrap();
//                     Var(lhs.[<$Op:lower>](&rhs).unwrap(), PhantomData)
//                 }
//             }
//             impl std::ops::$Op<Var<$lhs>> for Var<$rhs> {
//                 type Output = Var<$out>;
//
//                 fn [<$Op:lower>](self, rhs: Var<$lhs>) -> Self::Output {
//                     let lhs = self.0.cast(&rjit::VarType::[<$out:camel>]).unwrap();
//                     let rhs = rhs.0.cast(&rjit::VarType::[<$out:camel>]).unwrap();
//                     Var(lhs.[<$Op:lower>](&rhs).unwrap(), PhantomData)
//                 }
//             }
//         }
//     };
// }
// macro_rules! bop_relation {
//     ($low:ident < $high:ident, ) => {};
// }
//
// bop_sym!(f32 _ f64 = f64);
