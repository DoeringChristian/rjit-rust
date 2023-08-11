use std::marker::PhantomData;

use rjit::VarRef;

use super::*;

pub struct Texture<const N: usize> {
    texture: VarRef,
    n_channels: usize,
    shape: [usize; N],
}

impl<const N: usize> Texture<N> {
    pub fn new(data: impl Into<Float32>, shape: [usize; N], n_channels: usize) -> Self {
        let data: Float32 = data.into();
        let texture = data
            .internal()
            .to_texture(shape.as_slice(), n_channels)
            .unwrap();
        Self {
            texture,
            shape,
            n_channels,
        }
    }
    pub fn eval(&self, pos: impl Into<[Var<f32>; N]>) -> Vec<Float32> {
        let pos = pos
            .into()
            .into_iter()
            .map(|p| p.internal().clone())
            .collect::<Vec<_>>();
        let posref = pos.iter().map(|p| p).collect::<Vec<_>>();
        self.texture
            .tex_lookup(&posref)
            .unwrap()
            .into_iter()
            .map(|r| Float32::from(r))
            .collect::<Vec<_>>()
    }
}

pub type Texture2f = Texture<2>;
pub type Texture3f = Texture<3>;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tex2() {
        pretty_env_logger::try_init().ok();
        set_backend(["optix"]).unwrap();

        let val = Float32::from(vec![1.; 100].as_slice());
        let tex = Texture2f::new(val, [10, 10], 1);

        let x = tex.eval(vec2!(0.5, 0.5))[0].clone();

        x.schedule();
        eval();

        let z: Vec<_> = x.into();
        dbg!(z);
    }
}
