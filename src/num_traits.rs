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

// impl<T: Float> Signed for T {}
// impl<T: Int> Signed for T {}
