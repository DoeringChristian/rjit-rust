use super::*;
use paste::paste;

#[derive(Clone)]
pub struct Vec2<T> {
    pub x: Var<T>,
    pub y: Var<T>,
}
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2::from(($x, $y))
    };
    ($v:expr) => {
        Vec2::from($v)
    };
}
impl<T: rjit::AsVarType> From<Vec2<T>> for [Var<T>; 2] {
    fn from(value: Vec2<T>) -> Self {
        [value.x, value.y]
    }
}
impl<T: rjit::AsVarType, V: Into<Var<T>>> From<V> for Vec2<T> {
    fn from(value: V) -> Self {
        let value = value.into();
        Self {
            x: value.clone(),
            y: value.clone(),
        }
    }
}
impl<T: rjit::AsVarType, V: Into<Var<T>>> From<(V, V)> for Vec2<T> {
    fn from(value: (V, V)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}
impl<T: rjit::AsVarType> From<Vec3<T>> for Vec2<T> {
    fn from(value: Vec3<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl<T: rjit::AsVarType> From<Vec4<T>> for Vec2<T> {
    fn from(value: Vec4<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Clone)]
pub struct Vec3<T> {
    pub x: Var<T>,
    pub y: Var<T>,
    pub z: Var<T>,
}
#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::from(($x, $y, $z))
    };
    ($v:expr) => {
        Vec3::from($v)
    };
}
impl<T: rjit::AsVarType> From<Vec3<T>> for [Var<T>; 3] {
    fn from(value: Vec3<T>) -> Self {
        [value.x, value.y, value.z]
    }
}
impl<T: rjit::AsVarType, V: Into<Var<T>>> From<V> for Vec3<T> {
    fn from(value: V) -> Self {
        let value = value.into();
        Self {
            x: value.clone(),
            y: value.clone(),
            z: value.clone(),
        }
    }
}
impl<T: rjit::AsVarType, V: Into<Var<T>>> From<(V, V, V)> for Vec3<T> {
    fn from(value: (V, V, V)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
        }
    }
}
impl<T: rjit::AsVarType> From<Vec4<T>> for Vec3<T> {
    fn from(value: Vec4<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Clone)]
pub struct Vec4<T> {
    pub x: Var<T>,
    pub y: Var<T>,
    pub z: Var<T>,
    pub w: Var<T>,
}
#[macro_export]
macro_rules! vec4 {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        Vec4::from(($x, $y, $z, $w))
    };
    ($v:expr) => {
        Vec4::from($v)
    };
}
impl<T: rjit::AsVarType> From<Vec4<T>> for [Var<T>; 4] {
    fn from(value: Vec4<T>) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}
impl<T: rjit::AsVarType, V: Into<Var<T>>> From<V> for Vec4<T> {
    fn from(value: V) -> Self {
        let value = value.into();
        Self {
            x: value.clone(),
            y: value.clone(),
            z: value.clone(),
            w: value.clone(),
        }
    }
}
impl<T: rjit::AsVarType, V: Into<Var<T>>> From<(V, V, V, V)> for Vec4<T> {
    fn from(value: (V, V, V, V)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
            w: value.3.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct_vec2() {
        set_backend(["optix"]).unwrap();

        let v = Float32::from(1.);

        let v = vec2!(v);
        v.x;
    }
}
