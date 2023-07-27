#[cfg(test)]
mod test;

use paste::paste;
use std::marker::PhantomData;

use once_cell::sync::Lazy;

static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());

pub fn set_backend<'a>(backends: impl IntoIterator<Item = impl AsRef<str>>) -> rjit::Result<()> {
    TRACE.set_backend(backends)
}
pub fn eval() {
    TRACE.eval().unwrap();
}

pub struct Var<T>(rjit::VarRef, PhantomData<T>);

macro_rules! from_prim {
    ($prim:ident) => {
        impl From<$prim> for Var<$prim> {
            fn from(value: $prim) -> Self {
                Self(TRACE.literal(value).unwrap(), PhantomData)
            }
        }
        impl<'a> From<&'a [$prim]> for Var<$prim> {
            fn from(value: &'a [$prim]) -> Self {
                Self(TRACE.array(value).unwrap(), PhantomData)
            }
        }
    };
}
macro_rules! into_prim {
    ($prim:ident) => {
        impl Into<Vec<$prim>> for Var<$prim> {
            fn into(self) -> Vec<$prim> {
                self.0.to_host().unwrap()
            }
        }
    };
}

// bop!(Add, f32);

macro_rules! bop {
    ($op:ident) => {
        paste! {
            impl<T: rjit::AsVarType, Rhs: Into<Var<T>>> std::ops::$op<Rhs> for Var<T> {
                type Output = Var<T>;
                fn [<$op:lower>](self, rhs: Rhs) -> Self::Output {
                    let rhs = rhs.into();
                    Var(self.0.[<$op:lower>](&rhs.0).unwrap(), PhantomData)
                }
            }
            // impl<'a, T: rjit::AsVarType> std::ops::Add<Var<T>> for &'a [T]
            // where
            //     &'a [T]: Into<Var<T>>,
            // {
            //     type Output = Var<T>;
            //     fn add(self, rhs: Var<T>) -> Self::Output {
            //         let s: Var<T> = self.into();
            //         Var(s.0.add(&rhs.0).unwrap(), PhantomData)
            //     }
            // }
        }
    };
}

from_prim!(f32);
into_prim!(f32);

bop!(Add);
bop!(Sub);
bop!(Mul);
bop!(Div);
