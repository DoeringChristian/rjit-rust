use std::ops::Deref;

use crate::*;

pub struct Point2<T>(Vec2<T>);
#[macro_export]
macro_rules! point2 {
    ($x:expr, $y:expr) => {
        $crate::point::Point2::from(($x, $y))
    };
    ($v:expr) => {
        $crate::point::Point2::from($v)
    };
}

impl<T, V: Into<Vec2<T>>> From<V> for Point2<T> {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}
impl<T> Deref for Point2<T> {
    type Target = Vec2<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Point3<T>(Vec3<T>);
#[macro_export]
macro_rules! point3 {
    ($x:expr, $y:expr, $z:expr) => {
        Point3::from(($x, $y, $z))
    };
    ($v:expr) => {
        Point3::from($v)
    };
}

impl<T, V: Into<Vec3<T>>> From<V> for Point3<T> {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}

impl<T> Deref for Point3<T> {
    type Target = Vec3<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn construct_point2() {
        set_backend(["optix"]).unwrap();

        let p = point2!(array(&[1f32, 2f32, 3f32]), array(&[0f32, 0f32, 0f32]));
    }
}
