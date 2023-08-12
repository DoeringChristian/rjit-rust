pub trait SInt {}
pub trait UInt {}
pub trait Int {}
pub trait Bool {}
pub trait Float {}
pub trait Signed {}
pub trait Unsigned {}
pub trait LogicOrBitNot {}
pub trait Arithmetic {}

macro_rules! impl_trait {
    ($t:ident for $($ty:ident),*) => {
        $( impl $t for $ty{})*
    };
}

impl_trait!(SInt for i8, i16, i32, i64);
impl_trait!(UInt for u8, u16, u32, u64);
impl_trait!(Float for f32, f64);
impl_trait!(Bool for bool);
impl_trait!(Int for u8, u16, u32, u64, i8, i16, i32, i64);
impl_trait!(Unsigned for u8, u16, u32, u64);
impl_trait!(Signed for i8, i16, i32, i64, f32, f64);
impl_trait!(LogicOrBitNot for u8, u16, u32, u64, i8, i16, i32, i64, bool);
impl_trait!(Arithmetic for u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}

macro_rules! impl_zero {
    ($ty:ident: $zero:expr) => {
        impl Zero for $ty {
            fn zero() -> Self {
                $zero
            }
        }
    };
}
impl_zero!(i8: 0);
impl_zero!(u8: 0);
impl_zero!(u16: 0);
impl_zero!(i16: 0);
impl_zero!(u32: 0);
impl_zero!(i32: 0);
impl_zero!(i64: 0);
impl_zero!(u64: 0);
impl_zero!(f32: 0.);
impl_zero!(f64: 0.);
impl_zero!(bool: false);

macro_rules! impl_one {
    ($ty:ident: $one:expr) => {
        impl One for $ty {
            fn one() -> Self {
                $one
            }
        }
    };
}
impl_one!(i8: 1);
impl_one!(u8: 1);
impl_one!(u16: 1);
impl_one!(i16: 1);
impl_one!(u32: 1);
impl_one!(i32: 1);
impl_one!(i64: 1);
impl_one!(u64: 1);
impl_one!(f32: 1.);
impl_one!(f64: 1.);
impl_one!(bool: true);

// impl<T: Float> Signed for T {}
// impl<T: Int> Signed for T {}
