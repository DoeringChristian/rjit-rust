
use once_cell::sync::Lazy;
use std::marker::PhantomData;

pub static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());

pub struct Var<T>(rjit::VarRef, PhantomData<T>);

impl From<bool> for Var<bool> {
    fn from(value: bool) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [bool]> for Var<bool> {
    fn from(value: &'a [bool]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<i8>> for Var<bool> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<bool> {
    fn from(value: i8) -> Self {
        Var::<bool>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<bool> {
    fn from(value: &'a [i8]) -> Self {
        Var::<bool>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<bool> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<bool> {
    fn from(value: u8) -> Self {
        Var::<bool>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<bool> {
    fn from(value: &'a [u8]) -> Self {
        Var::<bool>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<bool> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<bool> {
    fn from(value: i16) -> Self {
        Var::<bool>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<bool> {
    fn from(value: &'a [i16]) -> Self {
        Var::<bool>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<bool> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<bool> {
    fn from(value: u16) -> Self {
        Var::<bool>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<bool> {
    fn from(value: &'a [u16]) -> Self {
        Var::<bool>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<bool> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<bool> {
    fn from(value: i32) -> Self {
        Var::<bool>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<bool> {
    fn from(value: &'a [i32]) -> Self {
        Var::<bool>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<bool> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<bool> {
    fn from(value: u32) -> Self {
        Var::<bool>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<bool> {
    fn from(value: &'a [u32]) -> Self {
        Var::<bool>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<bool> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<bool> {
    fn from(value: i64) -> Self {
        Var::<bool>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<bool> {
    fn from(value: &'a [i64]) -> Self {
        Var::<bool>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<bool> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<bool> {
    fn from(value: u64) -> Self {
        Var::<bool>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<bool> {
    fn from(value: &'a [u64]) -> Self {
        Var::<bool>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<bool> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<bool> {
    fn from(value: f32) -> Self {
        Var::<bool>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<bool> {
    fn from(value: &'a [f32]) -> Self {
        Var::<bool>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<bool> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::Bool).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<bool> {
    fn from(value: f64) -> Self {
        Var::<bool>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<bool> {
    fn from(value: &'a [f64]) -> Self {
        Var::<bool>::from(Var::<f64>::from(value))
    }
}

impl From<i8> for Var<i8> {
    fn from(value: i8) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [i8]> for Var<i8> {
    fn from(value: &'a [i8]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<i8> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<i8> {
    fn from(value: bool) -> Self {
        Var::<i8>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<i8> {
    fn from(value: &'a [bool]) -> Self {
        Var::<i8>::from(Var::<bool>::from(value))
    }
}

impl From<Var<u8>> for Var<i8> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<i8> {
    fn from(value: u8) -> Self {
        Var::<i8>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<i8> {
    fn from(value: &'a [u8]) -> Self {
        Var::<i8>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<i8> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<i8> {
    fn from(value: i16) -> Self {
        Var::<i8>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<i8> {
    fn from(value: &'a [i16]) -> Self {
        Var::<i8>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<i8> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<i8> {
    fn from(value: u16) -> Self {
        Var::<i8>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<i8> {
    fn from(value: &'a [u16]) -> Self {
        Var::<i8>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<i8> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<i8> {
    fn from(value: i32) -> Self {
        Var::<i8>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<i8> {
    fn from(value: &'a [i32]) -> Self {
        Var::<i8>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<i8> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<i8> {
    fn from(value: u32) -> Self {
        Var::<i8>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<i8> {
    fn from(value: &'a [u32]) -> Self {
        Var::<i8>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<i8> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<i8> {
    fn from(value: i64) -> Self {
        Var::<i8>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<i8> {
    fn from(value: &'a [i64]) -> Self {
        Var::<i8>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<i8> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<i8> {
    fn from(value: u64) -> Self {
        Var::<i8>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<i8> {
    fn from(value: &'a [u64]) -> Self {
        Var::<i8>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<i8> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<i8> {
    fn from(value: f32) -> Self {
        Var::<i8>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<i8> {
    fn from(value: &'a [f32]) -> Self {
        Var::<i8>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<i8> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I8).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<i8> {
    fn from(value: f64) -> Self {
        Var::<i8>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<i8> {
    fn from(value: &'a [f64]) -> Self {
        Var::<i8>::from(Var::<f64>::from(value))
    }
}

impl From<u8> for Var<u8> {
    fn from(value: u8) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [u8]> for Var<u8> {
    fn from(value: &'a [u8]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<u8> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<u8> {
    fn from(value: bool) -> Self {
        Var::<u8>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<u8> {
    fn from(value: &'a [bool]) -> Self {
        Var::<u8>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<u8> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<u8> {
    fn from(value: i8) -> Self {
        Var::<u8>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<u8> {
    fn from(value: &'a [i8]) -> Self {
        Var::<u8>::from(Var::<i8>::from(value))
    }
}

impl From<Var<i16>> for Var<u8> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<u8> {
    fn from(value: i16) -> Self {
        Var::<u8>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<u8> {
    fn from(value: &'a [i16]) -> Self {
        Var::<u8>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<u8> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<u8> {
    fn from(value: u16) -> Self {
        Var::<u8>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<u8> {
    fn from(value: &'a [u16]) -> Self {
        Var::<u8>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<u8> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<u8> {
    fn from(value: i32) -> Self {
        Var::<u8>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<u8> {
    fn from(value: &'a [i32]) -> Self {
        Var::<u8>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<u8> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<u8> {
    fn from(value: u32) -> Self {
        Var::<u8>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<u8> {
    fn from(value: &'a [u32]) -> Self {
        Var::<u8>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<u8> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<u8> {
    fn from(value: i64) -> Self {
        Var::<u8>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<u8> {
    fn from(value: &'a [i64]) -> Self {
        Var::<u8>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<u8> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<u8> {
    fn from(value: u64) -> Self {
        Var::<u8>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<u8> {
    fn from(value: &'a [u64]) -> Self {
        Var::<u8>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<u8> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<u8> {
    fn from(value: f32) -> Self {
        Var::<u8>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<u8> {
    fn from(value: &'a [f32]) -> Self {
        Var::<u8>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<u8> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U8).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<u8> {
    fn from(value: f64) -> Self {
        Var::<u8>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<u8> {
    fn from(value: &'a [f64]) -> Self {
        Var::<u8>::from(Var::<f64>::from(value))
    }
}

impl From<i16> for Var<i16> {
    fn from(value: i16) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [i16]> for Var<i16> {
    fn from(value: &'a [i16]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<i16> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<i16> {
    fn from(value: bool) -> Self {
        Var::<i16>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<i16> {
    fn from(value: &'a [bool]) -> Self {
        Var::<i16>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<i16> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<i16> {
    fn from(value: i8) -> Self {
        Var::<i16>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<i16> {
    fn from(value: &'a [i8]) -> Self {
        Var::<i16>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<i16> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<i16> {
    fn from(value: u8) -> Self {
        Var::<i16>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<i16> {
    fn from(value: &'a [u8]) -> Self {
        Var::<i16>::from(Var::<u8>::from(value))
    }
}

impl From<Var<u16>> for Var<i16> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<i16> {
    fn from(value: u16) -> Self {
        Var::<i16>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<i16> {
    fn from(value: &'a [u16]) -> Self {
        Var::<i16>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<i16> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<i16> {
    fn from(value: i32) -> Self {
        Var::<i16>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<i16> {
    fn from(value: &'a [i32]) -> Self {
        Var::<i16>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<i16> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<i16> {
    fn from(value: u32) -> Self {
        Var::<i16>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<i16> {
    fn from(value: &'a [u32]) -> Self {
        Var::<i16>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<i16> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<i16> {
    fn from(value: i64) -> Self {
        Var::<i16>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<i16> {
    fn from(value: &'a [i64]) -> Self {
        Var::<i16>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<i16> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<i16> {
    fn from(value: u64) -> Self {
        Var::<i16>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<i16> {
    fn from(value: &'a [u64]) -> Self {
        Var::<i16>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<i16> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<i16> {
    fn from(value: f32) -> Self {
        Var::<i16>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<i16> {
    fn from(value: &'a [f32]) -> Self {
        Var::<i16>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<i16> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I16).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<i16> {
    fn from(value: f64) -> Self {
        Var::<i16>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<i16> {
    fn from(value: &'a [f64]) -> Self {
        Var::<i16>::from(Var::<f64>::from(value))
    }
}

impl From<u16> for Var<u16> {
    fn from(value: u16) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [u16]> for Var<u16> {
    fn from(value: &'a [u16]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<u16> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<u16> {
    fn from(value: bool) -> Self {
        Var::<u16>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<u16> {
    fn from(value: &'a [bool]) -> Self {
        Var::<u16>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<u16> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<u16> {
    fn from(value: i8) -> Self {
        Var::<u16>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<u16> {
    fn from(value: &'a [i8]) -> Self {
        Var::<u16>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<u16> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<u16> {
    fn from(value: u8) -> Self {
        Var::<u16>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<u16> {
    fn from(value: &'a [u8]) -> Self {
        Var::<u16>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<u16> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<u16> {
    fn from(value: i16) -> Self {
        Var::<u16>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<u16> {
    fn from(value: &'a [i16]) -> Self {
        Var::<u16>::from(Var::<i16>::from(value))
    }
}

impl From<Var<i32>> for Var<u16> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<u16> {
    fn from(value: i32) -> Self {
        Var::<u16>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<u16> {
    fn from(value: &'a [i32]) -> Self {
        Var::<u16>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<u16> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<u16> {
    fn from(value: u32) -> Self {
        Var::<u16>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<u16> {
    fn from(value: &'a [u32]) -> Self {
        Var::<u16>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<u16> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<u16> {
    fn from(value: i64) -> Self {
        Var::<u16>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<u16> {
    fn from(value: &'a [i64]) -> Self {
        Var::<u16>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<u16> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<u16> {
    fn from(value: u64) -> Self {
        Var::<u16>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<u16> {
    fn from(value: &'a [u64]) -> Self {
        Var::<u16>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<u16> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<u16> {
    fn from(value: f32) -> Self {
        Var::<u16>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<u16> {
    fn from(value: &'a [f32]) -> Self {
        Var::<u16>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<u16> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U16).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<u16> {
    fn from(value: f64) -> Self {
        Var::<u16>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<u16> {
    fn from(value: &'a [f64]) -> Self {
        Var::<u16>::from(Var::<f64>::from(value))
    }
}

impl From<i32> for Var<i32> {
    fn from(value: i32) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [i32]> for Var<i32> {
    fn from(value: &'a [i32]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<i32> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<i32> {
    fn from(value: bool) -> Self {
        Var::<i32>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<i32> {
    fn from(value: &'a [bool]) -> Self {
        Var::<i32>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<i32> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<i32> {
    fn from(value: i8) -> Self {
        Var::<i32>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<i32> {
    fn from(value: &'a [i8]) -> Self {
        Var::<i32>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<i32> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<i32> {
    fn from(value: u8) -> Self {
        Var::<i32>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<i32> {
    fn from(value: &'a [u8]) -> Self {
        Var::<i32>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<i32> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<i32> {
    fn from(value: i16) -> Self {
        Var::<i32>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<i32> {
    fn from(value: &'a [i16]) -> Self {
        Var::<i32>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<i32> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<i32> {
    fn from(value: u16) -> Self {
        Var::<i32>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<i32> {
    fn from(value: &'a [u16]) -> Self {
        Var::<i32>::from(Var::<u16>::from(value))
    }
}

impl From<Var<u32>> for Var<i32> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<i32> {
    fn from(value: u32) -> Self {
        Var::<i32>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<i32> {
    fn from(value: &'a [u32]) -> Self {
        Var::<i32>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<i32> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<i32> {
    fn from(value: i64) -> Self {
        Var::<i32>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<i32> {
    fn from(value: &'a [i64]) -> Self {
        Var::<i32>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<i32> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<i32> {
    fn from(value: u64) -> Self {
        Var::<i32>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<i32> {
    fn from(value: &'a [u64]) -> Self {
        Var::<i32>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<i32> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<i32> {
    fn from(value: f32) -> Self {
        Var::<i32>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<i32> {
    fn from(value: &'a [f32]) -> Self {
        Var::<i32>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<i32> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I32).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<i32> {
    fn from(value: f64) -> Self {
        Var::<i32>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<i32> {
    fn from(value: &'a [f64]) -> Self {
        Var::<i32>::from(Var::<f64>::from(value))
    }
}

impl From<u32> for Var<u32> {
    fn from(value: u32) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [u32]> for Var<u32> {
    fn from(value: &'a [u32]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<u32> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<u32> {
    fn from(value: bool) -> Self {
        Var::<u32>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<u32> {
    fn from(value: &'a [bool]) -> Self {
        Var::<u32>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<u32> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<u32> {
    fn from(value: i8) -> Self {
        Var::<u32>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<u32> {
    fn from(value: &'a [i8]) -> Self {
        Var::<u32>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<u32> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<u32> {
    fn from(value: u8) -> Self {
        Var::<u32>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<u32> {
    fn from(value: &'a [u8]) -> Self {
        Var::<u32>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<u32> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<u32> {
    fn from(value: i16) -> Self {
        Var::<u32>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<u32> {
    fn from(value: &'a [i16]) -> Self {
        Var::<u32>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<u32> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<u32> {
    fn from(value: u16) -> Self {
        Var::<u32>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<u32> {
    fn from(value: &'a [u16]) -> Self {
        Var::<u32>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<u32> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<u32> {
    fn from(value: i32) -> Self {
        Var::<u32>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<u32> {
    fn from(value: &'a [i32]) -> Self {
        Var::<u32>::from(Var::<i32>::from(value))
    }
}

impl From<Var<i64>> for Var<u32> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<u32> {
    fn from(value: i64) -> Self {
        Var::<u32>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<u32> {
    fn from(value: &'a [i64]) -> Self {
        Var::<u32>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<u32> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<u32> {
    fn from(value: u64) -> Self {
        Var::<u32>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<u32> {
    fn from(value: &'a [u64]) -> Self {
        Var::<u32>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<u32> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<u32> {
    fn from(value: f32) -> Self {
        Var::<u32>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<u32> {
    fn from(value: &'a [f32]) -> Self {
        Var::<u32>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<u32> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U32).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<u32> {
    fn from(value: f64) -> Self {
        Var::<u32>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<u32> {
    fn from(value: &'a [f64]) -> Self {
        Var::<u32>::from(Var::<f64>::from(value))
    }
}

impl From<i64> for Var<i64> {
    fn from(value: i64) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [i64]> for Var<i64> {
    fn from(value: &'a [i64]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<i64> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<i64> {
    fn from(value: bool) -> Self {
        Var::<i64>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<i64> {
    fn from(value: &'a [bool]) -> Self {
        Var::<i64>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<i64> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<i64> {
    fn from(value: i8) -> Self {
        Var::<i64>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<i64> {
    fn from(value: &'a [i8]) -> Self {
        Var::<i64>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<i64> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<i64> {
    fn from(value: u8) -> Self {
        Var::<i64>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<i64> {
    fn from(value: &'a [u8]) -> Self {
        Var::<i64>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<i64> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<i64> {
    fn from(value: i16) -> Self {
        Var::<i64>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<i64> {
    fn from(value: &'a [i16]) -> Self {
        Var::<i64>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<i64> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<i64> {
    fn from(value: u16) -> Self {
        Var::<i64>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<i64> {
    fn from(value: &'a [u16]) -> Self {
        Var::<i64>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<i64> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<i64> {
    fn from(value: i32) -> Self {
        Var::<i64>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<i64> {
    fn from(value: &'a [i32]) -> Self {
        Var::<i64>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<i64> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<i64> {
    fn from(value: u32) -> Self {
        Var::<i64>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<i64> {
    fn from(value: &'a [u32]) -> Self {
        Var::<i64>::from(Var::<u32>::from(value))
    }
}

impl From<Var<u64>> for Var<i64> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<i64> {
    fn from(value: u64) -> Self {
        Var::<i64>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<i64> {
    fn from(value: &'a [u64]) -> Self {
        Var::<i64>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<i64> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<i64> {
    fn from(value: f32) -> Self {
        Var::<i64>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<i64> {
    fn from(value: &'a [f32]) -> Self {
        Var::<i64>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<i64> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::I64).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<i64> {
    fn from(value: f64) -> Self {
        Var::<i64>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<i64> {
    fn from(value: &'a [f64]) -> Self {
        Var::<i64>::from(Var::<f64>::from(value))
    }
}

impl From<u64> for Var<u64> {
    fn from(value: u64) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [u64]> for Var<u64> {
    fn from(value: &'a [u64]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<u64> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<u64> {
    fn from(value: bool) -> Self {
        Var::<u64>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<u64> {
    fn from(value: &'a [bool]) -> Self {
        Var::<u64>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<u64> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<u64> {
    fn from(value: i8) -> Self {
        Var::<u64>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<u64> {
    fn from(value: &'a [i8]) -> Self {
        Var::<u64>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<u64> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<u64> {
    fn from(value: u8) -> Self {
        Var::<u64>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<u64> {
    fn from(value: &'a [u8]) -> Self {
        Var::<u64>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<u64> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<u64> {
    fn from(value: i16) -> Self {
        Var::<u64>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<u64> {
    fn from(value: &'a [i16]) -> Self {
        Var::<u64>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<u64> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<u64> {
    fn from(value: u16) -> Self {
        Var::<u64>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<u64> {
    fn from(value: &'a [u16]) -> Self {
        Var::<u64>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<u64> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<u64> {
    fn from(value: i32) -> Self {
        Var::<u64>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<u64> {
    fn from(value: &'a [i32]) -> Self {
        Var::<u64>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<u64> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<u64> {
    fn from(value: u32) -> Self {
        Var::<u64>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<u64> {
    fn from(value: &'a [u32]) -> Self {
        Var::<u64>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<u64> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<u64> {
    fn from(value: i64) -> Self {
        Var::<u64>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<u64> {
    fn from(value: &'a [i64]) -> Self {
        Var::<u64>::from(Var::<i64>::from(value))
    }
}

impl From<Var<f32>> for Var<u64> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<u64> {
    fn from(value: f32) -> Self {
        Var::<u64>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<u64> {
    fn from(value: &'a [f32]) -> Self {
        Var::<u64>::from(Var::<f32>::from(value))
    }
}

impl From<Var<f64>> for Var<u64> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::U64).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<u64> {
    fn from(value: f64) -> Self {
        Var::<u64>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<u64> {
    fn from(value: &'a [f64]) -> Self {
        Var::<u64>::from(Var::<f64>::from(value))
    }
}

impl From<f32> for Var<f32> {
    fn from(value: f32) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [f32]> for Var<f32> {
    fn from(value: &'a [f32]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<f32> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<f32> {
    fn from(value: bool) -> Self {
        Var::<f32>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<f32> {
    fn from(value: &'a [bool]) -> Self {
        Var::<f32>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<f32> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<f32> {
    fn from(value: i8) -> Self {
        Var::<f32>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<f32> {
    fn from(value: &'a [i8]) -> Self {
        Var::<f32>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<f32> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<f32> {
    fn from(value: u8) -> Self {
        Var::<f32>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<f32> {
    fn from(value: &'a [u8]) -> Self {
        Var::<f32>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<f32> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<f32> {
    fn from(value: i16) -> Self {
        Var::<f32>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<f32> {
    fn from(value: &'a [i16]) -> Self {
        Var::<f32>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<f32> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<f32> {
    fn from(value: u16) -> Self {
        Var::<f32>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<f32> {
    fn from(value: &'a [u16]) -> Self {
        Var::<f32>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<f32> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<f32> {
    fn from(value: i32) -> Self {
        Var::<f32>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<f32> {
    fn from(value: &'a [i32]) -> Self {
        Var::<f32>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<f32> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<f32> {
    fn from(value: u32) -> Self {
        Var::<f32>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<f32> {
    fn from(value: &'a [u32]) -> Self {
        Var::<f32>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<f32> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<f32> {
    fn from(value: i64) -> Self {
        Var::<f32>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<f32> {
    fn from(value: &'a [i64]) -> Self {
        Var::<f32>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<f32> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<f32> {
    fn from(value: u64) -> Self {
        Var::<f32>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<f32> {
    fn from(value: &'a [u64]) -> Self {
        Var::<f32>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f64>> for Var<f32> {
    fn from(value: Var<f64>) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap(), PhantomData)
    }
}

impl From<f64> for Var<f32> {
    fn from(value: f64) -> Self {
        Var::<f32>::from(Var::<f64>::from(value))
    }
}
impl<'a> From<&'a [f64]> for Var<f32> {
    fn from(value: &'a [f64]) -> Self {
        Var::<f32>::from(Var::<f64>::from(value))
    }
}

impl From<f64> for Var<f64> {
    fn from(value: f64) -> Self {
        Self(TRACE.literal(value).unwrap(), PhantomData)
    }
}
impl<'a> From<&'a [f64]> for Var<f64> {
    fn from(value: &'a [f64]) -> Self {
        Self(TRACE.array(value).unwrap(), PhantomData)
    }
}

impl From<Var<bool>> for Var<f64> {
    fn from(value: Var<bool>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<bool> for Var<f64> {
    fn from(value: bool) -> Self {
        Var::<f64>::from(Var::<bool>::from(value))
    }
}
impl<'a> From<&'a [bool]> for Var<f64> {
    fn from(value: &'a [bool]) -> Self {
        Var::<f64>::from(Var::<bool>::from(value))
    }
}

impl From<Var<i8>> for Var<f64> {
    fn from(value: Var<i8>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<i8> for Var<f64> {
    fn from(value: i8) -> Self {
        Var::<f64>::from(Var::<i8>::from(value))
    }
}
impl<'a> From<&'a [i8]> for Var<f64> {
    fn from(value: &'a [i8]) -> Self {
        Var::<f64>::from(Var::<i8>::from(value))
    }
}

impl From<Var<u8>> for Var<f64> {
    fn from(value: Var<u8>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<u8> for Var<f64> {
    fn from(value: u8) -> Self {
        Var::<f64>::from(Var::<u8>::from(value))
    }
}
impl<'a> From<&'a [u8]> for Var<f64> {
    fn from(value: &'a [u8]) -> Self {
        Var::<f64>::from(Var::<u8>::from(value))
    }
}

impl From<Var<i16>> for Var<f64> {
    fn from(value: Var<i16>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<i16> for Var<f64> {
    fn from(value: i16) -> Self {
        Var::<f64>::from(Var::<i16>::from(value))
    }
}
impl<'a> From<&'a [i16]> for Var<f64> {
    fn from(value: &'a [i16]) -> Self {
        Var::<f64>::from(Var::<i16>::from(value))
    }
}

impl From<Var<u16>> for Var<f64> {
    fn from(value: Var<u16>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<u16> for Var<f64> {
    fn from(value: u16) -> Self {
        Var::<f64>::from(Var::<u16>::from(value))
    }
}
impl<'a> From<&'a [u16]> for Var<f64> {
    fn from(value: &'a [u16]) -> Self {
        Var::<f64>::from(Var::<u16>::from(value))
    }
}

impl From<Var<i32>> for Var<f64> {
    fn from(value: Var<i32>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<i32> for Var<f64> {
    fn from(value: i32) -> Self {
        Var::<f64>::from(Var::<i32>::from(value))
    }
}
impl<'a> From<&'a [i32]> for Var<f64> {
    fn from(value: &'a [i32]) -> Self {
        Var::<f64>::from(Var::<i32>::from(value))
    }
}

impl From<Var<u32>> for Var<f64> {
    fn from(value: Var<u32>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<u32> for Var<f64> {
    fn from(value: u32) -> Self {
        Var::<f64>::from(Var::<u32>::from(value))
    }
}
impl<'a> From<&'a [u32]> for Var<f64> {
    fn from(value: &'a [u32]) -> Self {
        Var::<f64>::from(Var::<u32>::from(value))
    }
}

impl From<Var<i64>> for Var<f64> {
    fn from(value: Var<i64>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<i64> for Var<f64> {
    fn from(value: i64) -> Self {
        Var::<f64>::from(Var::<i64>::from(value))
    }
}
impl<'a> From<&'a [i64]> for Var<f64> {
    fn from(value: &'a [i64]) -> Self {
        Var::<f64>::from(Var::<i64>::from(value))
    }
}

impl From<Var<u64>> for Var<f64> {
    fn from(value: Var<u64>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<u64> for Var<f64> {
    fn from(value: u64) -> Self {
        Var::<f64>::from(Var::<u64>::from(value))
    }
}
impl<'a> From<&'a [u64]> for Var<f64> {
    fn from(value: &'a [u64]) -> Self {
        Var::<f64>::from(Var::<u64>::from(value))
    }
}

impl From<Var<f32>> for Var<f64> {
    fn from(value: Var<f32>) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap(), PhantomData)
    }
}

impl From<f32> for Var<f64> {
    fn from(value: f32) -> Self {
        Var::<f64>::from(Var::<f32>::from(value))
    }
}
impl<'a> From<&'a [f32]> for Var<f64> {
    fn from(value: &'a [f32]) -> Self {
        Var::<f64>::from(Var::<f32>::from(value))
    }
}

impl std::ops::Add<Var<bool>> for Var<bool> {
    type Output = Var<bool>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::Bool).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<bool> {
    type Output = Var<bool>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = Var::<bool>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<bool> {
    type Output = Var<i8>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<bool> {
    type Output = Var<i8>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<bool> {
    type Output = Var<u8>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<bool> {
    type Output = Var<u8>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<bool> {
    type Output = Var<i16>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<bool> {
    type Output = Var<i16>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<bool> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<bool> {
    type Output = Var<u16>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<bool> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<bool> {
    type Output = Var<i32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<bool> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<bool> {
    type Output = Var<u32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<bool> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<bool> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<bool> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<bool> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<bool> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<bool> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<bool> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<bool> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<i8> {
    type Output = Var<i8>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<i8> {
    type Output = Var<i8>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<i8> {
    type Output = Var<i8>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<i8> {
    type Output = Var<i8>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<i8> {
    type Output = Var<u8>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<i8> {
    type Output = Var<u8>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<i8> {
    type Output = Var<i16>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<i8> {
    type Output = Var<i16>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<i8> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<i8> {
    type Output = Var<u16>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<i8> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<i8> {
    type Output = Var<i32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<i8> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<i8> {
    type Output = Var<u32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<i8> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<i8> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<i8> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<i8> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<i8> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<i8> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<i8> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<i8> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<u8> {
    type Output = Var<u8>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<u8> {
    type Output = Var<u8>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<u8> {
    type Output = Var<u8>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<u8> {
    type Output = Var<u8>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<u8> {
    type Output = Var<u8>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<u8> {
    type Output = Var<u8>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<u8> {
    type Output = Var<i16>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<u8> {
    type Output = Var<i16>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<u8> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<u8> {
    type Output = Var<u16>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<u8> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<u8> {
    type Output = Var<i32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<u8> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<u8> {
    type Output = Var<u32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<u8> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<u8> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<u8> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<u8> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<u8> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<u8> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<u8> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<u8> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<i16> {
    type Output = Var<i16>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<i16> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<i16> {
    type Output = Var<u16>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<i16> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<i16> {
    type Output = Var<i32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<i16> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<i16> {
    type Output = Var<u32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<i16> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<i16> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<i16> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<i16> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<i16> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<i16> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<i16> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<i16> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<u16> {
    type Output = Var<u16>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<u16> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<u16> {
    type Output = Var<i32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<u16> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<u16> {
    type Output = Var<u32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<u16> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<u16> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<u16> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<u16> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<u16> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<u16> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<u16> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<u16> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<i32> {
    type Output = Var<i32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<i32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<i32> {
    type Output = Var<u32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<i32> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<i32> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<i32> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<i32> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<i32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<i32> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<i32> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<i32> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<u32> {
    type Output = Var<u32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<u32> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<u32> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<u32> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<u32> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<u32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<u32> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<u32> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<u32> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<i64> {
    type Output = Var<i64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<i64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<i64> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<i64> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<i64> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<i64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<i64> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<u64> {
    type Output = Var<u64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<u64> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<u64> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<u64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<u64> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<f32> {
    type Output = Var<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<f32> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<f32> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<bool>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<bool> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i8>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i8> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u8>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u8> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i16>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i16> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u16>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u16> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i32>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i32> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u32>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u32> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<i64>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<i64> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<u64>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<u64> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f32>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f32> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Add<Var<f64>> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.add(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Add<f64> for Var<f64> {
    type Output = Var<f64>;

    fn add(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.add(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<bool> {
    type Output = Var<bool>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::Bool).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<bool> {
    type Output = Var<bool>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = Var::<bool>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<bool> {
    type Output = Var<i8>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<bool> {
    type Output = Var<i8>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<bool> {
    type Output = Var<u8>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<bool> {
    type Output = Var<u8>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<bool> {
    type Output = Var<i16>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<bool> {
    type Output = Var<i16>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<bool> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<bool> {
    type Output = Var<u16>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<bool> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<bool> {
    type Output = Var<i32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<bool> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<bool> {
    type Output = Var<u32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<bool> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<bool> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<bool> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<bool> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<bool> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<bool> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<bool> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<bool> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<i8> {
    type Output = Var<i8>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<i8> {
    type Output = Var<i8>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<i8> {
    type Output = Var<i8>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<i8> {
    type Output = Var<i8>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<i8> {
    type Output = Var<u8>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<i8> {
    type Output = Var<u8>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<i8> {
    type Output = Var<i16>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<i8> {
    type Output = Var<i16>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<i8> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<i8> {
    type Output = Var<u16>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<i8> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<i8> {
    type Output = Var<i32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<i8> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<i8> {
    type Output = Var<u32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<i8> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<i8> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<i8> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<i8> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<i8> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<i8> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<i8> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<i8> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<u8> {
    type Output = Var<u8>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<u8> {
    type Output = Var<u8>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<u8> {
    type Output = Var<u8>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<u8> {
    type Output = Var<u8>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<u8> {
    type Output = Var<u8>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<u8> {
    type Output = Var<u8>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<u8> {
    type Output = Var<i16>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<u8> {
    type Output = Var<i16>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<u8> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<u8> {
    type Output = Var<u16>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<u8> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<u8> {
    type Output = Var<i32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<u8> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<u8> {
    type Output = Var<u32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<u8> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<u8> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<u8> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<u8> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<u8> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<u8> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<u8> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<u8> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<i16> {
    type Output = Var<i16>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<i16> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<i16> {
    type Output = Var<u16>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<i16> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<i16> {
    type Output = Var<i32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<i16> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<i16> {
    type Output = Var<u32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<i16> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<i16> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<i16> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<i16> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<i16> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<i16> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<i16> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<i16> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<u16> {
    type Output = Var<u16>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<u16> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<u16> {
    type Output = Var<i32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<u16> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<u16> {
    type Output = Var<u32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<u16> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<u16> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<u16> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<u16> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<u16> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<u16> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<u16> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<u16> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<i32> {
    type Output = Var<i32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<i32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<i32> {
    type Output = Var<u32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<i32> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<i32> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<i32> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<i32> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<i32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<i32> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<i32> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<i32> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<u32> {
    type Output = Var<u32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<u32> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<u32> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<u32> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<u32> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<u32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<u32> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<u32> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<u32> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<i64> {
    type Output = Var<i64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<i64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<i64> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<i64> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<i64> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<i64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<i64> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<u64> {
    type Output = Var<u64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<u64> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<u64> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<u64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<u64> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<f32> {
    type Output = Var<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<f32> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<f32> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<bool>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<bool> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i8>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i8> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u8>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u8> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i16>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i16> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u16>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u16> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i32>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i32> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u32>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u32> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<i64>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<i64> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<u64>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<u64> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f32>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f32> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<Var<f64>> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.sub(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Sub<f64> for Var<f64> {
    type Output = Var<f64>;

    fn sub(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.sub(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<bool> {
    type Output = Var<bool>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::Bool).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<bool> {
    type Output = Var<bool>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = Var::<bool>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<bool> {
    type Output = Var<i8>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<bool> {
    type Output = Var<i8>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<bool> {
    type Output = Var<u8>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<bool> {
    type Output = Var<u8>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<bool> {
    type Output = Var<i16>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<bool> {
    type Output = Var<i16>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<bool> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<bool> {
    type Output = Var<u16>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<bool> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<bool> {
    type Output = Var<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<bool> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<bool> {
    type Output = Var<u32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<bool> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<bool> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<bool> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<bool> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<bool> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<bool> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<bool> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<bool> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<i8> {
    type Output = Var<i8>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<i8> {
    type Output = Var<i8>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<i8> {
    type Output = Var<i8>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<i8> {
    type Output = Var<i8>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<i8> {
    type Output = Var<u8>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<i8> {
    type Output = Var<u8>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<i8> {
    type Output = Var<i16>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<i8> {
    type Output = Var<i16>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<i8> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<i8> {
    type Output = Var<u16>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<i8> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<i8> {
    type Output = Var<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<i8> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<i8> {
    type Output = Var<u32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<i8> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<i8> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<i8> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<i8> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<i8> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<i8> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<i8> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<i8> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<u8> {
    type Output = Var<u8>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<u8> {
    type Output = Var<u8>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<u8> {
    type Output = Var<u8>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<u8> {
    type Output = Var<u8>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<u8> {
    type Output = Var<u8>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<u8> {
    type Output = Var<u8>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<u8> {
    type Output = Var<i16>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<u8> {
    type Output = Var<i16>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<u8> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<u8> {
    type Output = Var<u16>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<u8> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<u8> {
    type Output = Var<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<u8> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<u8> {
    type Output = Var<u32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<u8> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<u8> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<u8> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<u8> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<u8> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<u8> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<u8> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<u8> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<i16> {
    type Output = Var<i16>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<i16> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<i16> {
    type Output = Var<u16>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<i16> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<i16> {
    type Output = Var<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<i16> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<i16> {
    type Output = Var<u32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<i16> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<i16> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<i16> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<i16> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<i16> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<i16> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<i16> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<i16> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<u16> {
    type Output = Var<u16>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<u16> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<u16> {
    type Output = Var<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<u16> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<u16> {
    type Output = Var<u32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<u16> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<u16> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<u16> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<u16> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<u16> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<u16> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<u16> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<u16> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<i32> {
    type Output = Var<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<i32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<i32> {
    type Output = Var<u32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<i32> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<i32> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<i32> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<i32> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<i32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<i32> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<i32> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<i32> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<u32> {
    type Output = Var<u32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<u32> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<u32> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<u32> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<u32> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<u32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<u32> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<u32> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<u32> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<i64> {
    type Output = Var<i64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<i64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<i64> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<i64> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<i64> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<i64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<i64> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<u64> {
    type Output = Var<u64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<u64> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<u64> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<u64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<u64> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<f32> {
    type Output = Var<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<f32> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<f32> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<bool>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<bool> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i8>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i8> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u8>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u8> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i16>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i16> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u16>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u16> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i32>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i32> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u32>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u32> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<i64>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<i64> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<u64>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<u64> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f32>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f32> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<Var<f64>> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.mul(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Mul<f64> for Var<f64> {
    type Output = Var<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.mul(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<bool> {
    type Output = Var<bool>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::Bool).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<bool> {
    type Output = Var<bool>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::Bool).unwrap();
        let rhs = Var::<bool>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<bool> {
    type Output = Var<i8>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<bool> {
    type Output = Var<i8>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<bool> {
    type Output = Var<u8>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<bool> {
    type Output = Var<u8>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<bool> {
    type Output = Var<i16>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<bool> {
    type Output = Var<i16>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<bool> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<bool> {
    type Output = Var<u16>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<bool> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<bool> {
    type Output = Var<i32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<bool> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<bool> {
    type Output = Var<u32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<bool> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<bool> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<bool> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<bool> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<bool> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<bool> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<bool> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<bool> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<i8> {
    type Output = Var<i8>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<i8> {
    type Output = Var<i8>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<i8> {
    type Output = Var<i8>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<i8> {
    type Output = Var<i8>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I8).unwrap();
        let rhs = Var::<i8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<i8> {
    type Output = Var<u8>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<i8> {
    type Output = Var<u8>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<i8> {
    type Output = Var<i16>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<i8> {
    type Output = Var<i16>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<i8> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<i8> {
    type Output = Var<u16>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<i8> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<i8> {
    type Output = Var<i32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<i8> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<i8> {
    type Output = Var<u32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<i8> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<i8> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<i8> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<i8> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<i8> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<i8> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<i8> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<i8> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<u8> {
    type Output = Var<u8>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<u8> {
    type Output = Var<u8>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<u8> {
    type Output = Var<u8>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<u8> {
    type Output = Var<u8>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<u8> {
    type Output = Var<u8>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U8).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<u8> {
    type Output = Var<u8>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U8).unwrap();
        let rhs = Var::<u8>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<u8> {
    type Output = Var<i16>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<u8> {
    type Output = Var<i16>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<u8> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<u8> {
    type Output = Var<u16>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<u8> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<u8> {
    type Output = Var<i32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<u8> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<u8> {
    type Output = Var<u32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<u8> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<u8> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<u8> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<u8> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<u8> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<u8> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<u8> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<u8> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<i16> {
    type Output = Var<i16>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I16).unwrap();
        let rhs = Var::<i16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<i16> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<i16> {
    type Output = Var<u16>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<i16> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<i16> {
    type Output = Var<i32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<i16> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<i16> {
    type Output = Var<u32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<i16> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<i16> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<i16> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<i16> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<i16> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<i16> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<i16> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<i16> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U16).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<u16> {
    type Output = Var<u16>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U16).unwrap();
        let rhs = Var::<u16>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<u16> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<u16> {
    type Output = Var<i32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<u16> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<u16> {
    type Output = Var<u32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<u16> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<u16> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<u16> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<u16> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<u16> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<u16> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<u16> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<u16> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<i32> {
    type Output = Var<i32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I32).unwrap();
        let rhs = Var::<i32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<i32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<i32> {
    type Output = Var<u32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<i32> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<i32> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<i32> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<i32> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<i32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<i32> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<i32> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<i32> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<u32> {
    type Output = Var<u32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U32).unwrap();
        let rhs = Var::<u32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<u32> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<u32> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<u32> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<u32> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<u32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<u32> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<u32> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<u32> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::I64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<i64> {
    type Output = Var<i64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::I64).unwrap();
        let rhs = Var::<i64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<i64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<i64> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<i64> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<i64> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<i64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<i64> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::U64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<u64> {
    type Output = Var<u64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::U64).unwrap();
        let rhs = Var::<u64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<u64> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<u64> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<u64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<u64> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F32).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<f32> {
    type Output = Var<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F32).unwrap();
        let rhs = Var::<f32>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<f32> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<f32> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<bool>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<bool>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<bool> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: bool) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i8>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<i8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i8> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: i8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u8>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<u8>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u8> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: u8) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i16>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<i16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i16> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: i16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u16>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<u16>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u16> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: u16) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i32>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<i32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i32> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u32>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<u32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u32> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: u32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<i64>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<i64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<i64> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: i64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<u64>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<u64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<u64> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: u64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f32>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f32>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f32> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: f32) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}

impl std::ops::Div<Var<f64>> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: Var<f64>) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = rhs.0.cast(&rjit::VarType::F64).unwrap();
        Var(lhs.div(&rhs).unwrap(), PhantomData)
    }
}

impl std::ops::Div<f64> for Var<f64> {
    type Output = Var<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        let lhs = self.0.cast(&rjit::VarType::F64).unwrap();
        let rhs = Var::<f64>::from(rhs);
        Var(lhs.div(&rhs.0).unwrap(), PhantomData)
    }
}
