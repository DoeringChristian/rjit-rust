
use once_cell::sync::Lazy;

pub static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());

pub struct Float32(pub rjit::VarRef);

impl From<f32> for Float32 {
    fn from(value: f32) -> Self {
        Self(TRACE.literal(value).unwrap())
    }
}
impl<'a> From<&'a [f32]> for Float32 {
    fn from(value: &'a [f32]) -> Self {
        Self(TRACE.array(value).unwrap())
    }
}

impl From<Float64> for Float32 {
    fn from(value: Float64) -> Self {
        Self(value.0.cast(&rjit::VarType::F32).unwrap())
    }
}
pub struct Float64(pub rjit::VarRef);

impl From<f64> for Float64 {
    fn from(value: f64) -> Self {
        Self(TRACE.literal(value).unwrap())
    }
}
impl<'a> From<&'a [f64]> for Float64 {
    fn from(value: &'a [f64]) -> Self {
        Self(TRACE.array(value).unwrap())
    }
}

impl From<Float32> for Float64 {
    fn from(value: Float32) -> Self {
        Self(value.0.cast(&rjit::VarType::F64).unwrap())
    }
}

impl std::ops::Add<Float32> for Float32 {
    type Output = Float32;

    fn add(self, rhs: Float32) -> Self::Output {
        Float32(self.0.add(&rhs.0).unwrap())
    }
}

impl std::ops::Add<Float64> for Float32 {
    type Output = Float64;

    fn add(self, rhs: Float64) -> Self::Output {
        Float64(self.0.add(&rhs.0).unwrap())
    }
}

impl std::ops::Add<Float32> for Float64 {
    type Output = Float64;

    fn add(self, rhs: Float32) -> Self::Output {
        Float64(self.0.add(&rhs.0).unwrap())
    }
}

impl std::ops::Add<Float64> for Float64 {
    type Output = Float64;

    fn add(self, rhs: Float64) -> Self::Output {
        Float64(self.0.add(&rhs.0).unwrap())
    }
}

impl std::ops::Mul<Float32> for Float32 {
    type Output = Float32;

    fn mul(self, rhs: Float32) -> Self::Output {
        Float32(self.0.mul(&rhs.0).unwrap())
    }
}

impl std::ops::Mul<Float64> for Float32 {
    type Output = Float64;

    fn mul(self, rhs: Float64) -> Self::Output {
        Float64(self.0.mul(&rhs.0).unwrap())
    }
}

impl std::ops::Mul<Float32> for Float64 {
    type Output = Float64;

    fn mul(self, rhs: Float32) -> Self::Output {
        Float64(self.0.mul(&rhs.0).unwrap())
    }
}

impl std::ops::Mul<Float64> for Float64 {
    type Output = Float64;

    fn mul(self, rhs: Float64) -> Self::Output {
        Float64(self.0.mul(&rhs.0).unwrap())
    }
}
