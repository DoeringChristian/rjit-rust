use super::*;
use paste::paste;
pub use rjit::ReduceOp;
use std::fmt::Debug;
use std::marker::PhantomData;

pub fn literal<T: rjit::AsVarType>(value: T) -> Var<T> {
    Var(TRACE.literal(value).unwrap(), PhantomData)
}
pub fn array<T: rjit::AsVarType>(value: &[T]) -> Var<T> {
    Var(TRACE.array(value).unwrap(), PhantomData)
}

pub struct Var<T>(pub rjit::VarRef, pub PhantomData<T>);

impl<T> Clone for Var<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

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
    ($($prim:ident),*) => {
        $(from_prim!($prim);)*
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
    ($($prim:ident),*) => {
        $(into_prim!($prim);)*
    };
}

macro_rules! lhs_arythmetic {
    ($op:ident) => {
        paste! {
            impl<T: rjit::AsVarType, Rhs: Into<Var<T>>> std::ops::[<$op:camel>]<Rhs> for Var<T> {
                type Output = Var<T>;

                fn $op(self, rhs: Rhs) -> Self::Output {
                    Self::$op(&self, rhs)
                }
            }
        }
    };
    ($($op:ident),*) => {
        $(lhs_arythmetic!($op);)*
    }
}

macro_rules! arythmetic_ops {
    ($Op:ident as $op:ident for $ty:ident) => {
        paste! {
            impl std::ops::$Op<Var<$ty>> for $ty {
                type Output = Var<$ty>;

                fn [<$Op:lower>](self, rhs: Var<$ty>) -> Self::Output {
                    rhs.$op(self)
                }
            }
            impl<'a> std::ops::$Op<&'a Var<$ty>> for $ty {
                type Output = Var<$ty>;

                fn [<$Op:lower>](self, rhs: &'a Var<$ty>) -> Self::Output {
                    rhs.$op(self)
                }
            }
            impl<Rhs: Into<Var<$ty>>> std::ops::$Op<Rhs> for Var<$ty> {
                type Output = Var<$ty>;
                fn [<$Op:lower>](self, rhs: Rhs) -> Self::Output {
                    Self::$op(&self, rhs)
                }
            }
        }
    };
    ($Op:ident as $op:ident for $($ty:ident),*) => {
        $(arythmetic_ops!($Op as $op for $ty);)*
    }
}

from_prim!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, bool);
into_prim!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, bool);

arythmetic_ops!(Add as add for u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
arythmetic_ops!(Sub as sub for u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
arythmetic_ops!(Mul as mul for u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
arythmetic_ops!(Div as div for u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

arythmetic_ops!(BitAnd as and for u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
arythmetic_ops!(BitOr as or for u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
arythmetic_ops!(BitXor as xor for u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

impl<T: rjit::AsVarType + Signed> std::ops::Neg for Var<T> {
    type Output = Var<T>;

    fn neg(self) -> Self::Output {
        Self::neg(&self)
    }
}
impl<T: rjit::AsVarType + LogicOrBitNot> std::ops::Not for Var<T> {
    type Output = Var<T>;

    fn not(self) -> Self::Output {
        Self::not(&self)
    }
}

// Into from ref as clone
impl<'a, T: rjit::AsVarType + Copy + Into<Var<T>>> From<&'a T> for Var<T> {
    fn from(value: &'a T) -> Self {
        (*value).into()
    }
}
impl<'a, T: rjit::AsVarType> From<&'a Var<T>> for Var<T> {
    fn from(value: &'a Var<T>) -> Self {
        value.clone().into()
    }
}

macro_rules! bop {
    ($op:ident -> $ty:ident) => {
        pub fn $op(&self, rhs: impl Into<Var<T>>) -> Var<$ty> {
            let rhs = rhs.into();
            Var(self.0.$op(&rhs.0).unwrap(), PhantomData)
        }
    };
    ($op:ident) => {
        pub fn $op(&self, rhs: impl Into<Var<T>>) -> Var<T> {
            let rhs = rhs.into();
            Var(self.0.$op(&rhs.0).unwrap(), PhantomData)
        }
    };
}
macro_rules! uop {
    ($op:ident) => {
        pub fn $op(&self) -> Self {
            Var(self.0.$op().unwrap(), PhantomData)
        }
    };
}

impl<T: rjit::AsVarType> Var<T> {
    pub fn internal(&self) -> &rjit::VarRef {
        &self.0
    }
    pub fn schedule(&self) {
        self.0.schedule()
    }
    pub fn size(&self) -> usize {
        self.0.size()
    }
    pub fn cast<U: rjit::AsVarType>(&self) -> Var<U> {
        Var(self.0.cast(&U::as_var_type()).unwrap(), PhantomData)
    }
    pub fn bitcast<U: rjit::AsVarType>(&self) -> Var<U> {
        Var(self.0.bitcast(&U::as_var_type()).unwrap(), PhantomData)
    }

    bop!(eq -> bool);
    bop!(neq -> bool);
    bop!(lt -> bool);
    bop!(le -> bool);
    bop!(gt -> bool);
    bop!(ge -> bool);

    bop!(or);
    bop!(xor);
    bop!(and);

    bop!(modulo);

    pub fn fma(&self, b: impl Into<Var<T>>, c: impl Into<Var<T>>) -> Self {
        let b = b.into();
        let c = c.into();
        Var(self.0.fma(&b.0, &c.0).unwrap(), PhantomData)
    }

    pub fn gather<I: rjit::AsVarType + Int>(
        &self,
        index: impl Into<Var<I>>,
        mask: Option<impl Into<Var<bool>>>,
    ) -> Self {
        let index = index.into();
        let mask = mask.map(|m| m.into().0);
        Self(self.0.gather(&index.0, mask.as_ref()).unwrap(), PhantomData)
    }
    pub fn scatter<I: rjit::AsVarType + Int>(
        &self,
        dst: impl Into<Var<T>>,
        index: impl Into<Var<I>>,
        mask: Option<impl Into<Var<bool>>>,
    ) {
        let dst = dst.into();
        let index = index.into();
        let mask = mask.map(|m| m.into().0);
        self.0.scatter(&dst.0, &index.0, mask.as_ref()).unwrap();
    }
    pub fn scatter_reduce<I: rjit::AsVarType + Int>(
        &self,
        dst: impl Into<Var<T>>,
        index: impl Into<Var<I>>,
        mask: Option<impl Into<Var<bool>>>,
        reduce_op: ReduceOp,
    ) {
        let dst = dst.into();
        let index = index.into();
        let mask = mask.map(|m| m.into().0);
        self.0
            .scatter_reduce(&dst.0, &index.0, mask.as_ref(), reduce_op)
            .unwrap();
    }
}
impl<T: rjit::AsVarType + Arithmetic> Var<T> {
    // Arithmetic operations:
    bop!(add);
    bop!(sub);
    bop!(mul);
    bop!(div);

    // Min/Max
    bop!(min);
    bop!(max);
}
impl<T: rjit::AsVarType + LogicOrBitNot> Var<T> {
    uop!(not);
}
impl<T: rjit::AsVarType + Float> Var<T> {
    uop!(rcp);
    uop!(rsqrt);
    uop!(sin);
    uop!(cos);
    uop!(exp2);
    uop!(log2);

    uop!(trunc);
    uop!(ceil);
    uop!(floor);
    uop!(round);
}
impl<T: rjit::AsVarType + Signed> Var<T> {
    uop!(neg);
    uop!(abs);
}
impl Var<bool> {
    pub fn select<U: rjit::AsVarType>(
        &self,
        true_value: impl Into<Var<U>>,
        false_value: impl Into<Var<U>>,
    ) -> Var<U> {
        let true_value = true_value.into();
        let false_value = false_value.into();
        Var(
            self.0.select(&true_value.0, &false_value.0).unwrap(),
            PhantomData,
        )
    }
}
impl<T: rjit::AsVarType + Int> Var<T> {
    // Bit shift operations
    pub fn shl<U: rjit::AsVarType + Int>(&self, rhs: impl Into<Var<U>>) -> Self {
        let rhs = rhs.into();
        Var(self.0.shl(&rhs.0).unwrap(), PhantomData)
    }
    pub fn shr<U: rjit::AsVarType + Int>(&self, rhs: impl Into<Var<U>>) -> Self {
        let rhs = rhs.into();
        Var(self.0.shr(&rhs.0).unwrap(), PhantomData)
    }
    uop!(popc);
    uop!(clz);
    uop!(ctz);
}

impl<T: rjit::AsVarType> Debug for Var<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type Bool = Var<bool>;
pub type Int8 = Var<i8>;
pub type UInt8 = Var<u8>;
pub type Int16 = Var<i16>;
pub type UInt16 = Var<u16>;
pub type Int32 = Var<i32>;
pub type UInt32 = Var<u32>;
pub type Int64 = Var<i64>;
pub type UInt64 = Var<u64>;
pub type Float32 = Var<f32>;
pub type Float64 = Var<f64>;
