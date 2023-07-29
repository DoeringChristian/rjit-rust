use std::marker::PhantomData;

use super::*;

pub struct Texture {
    textures: Vec<rjit::VarRef>,
    shape: Vec<usize>,
}

impl Texture {
    pub fn new<T: Into<Float32>>(
        shape: impl IntoIterator<Item = usize>,
        channels: impl IntoIterator<Item = T>,
    ) -> Self {
        // TODO: more efficient textures
        let shape = shape.into_iter().collect::<Vec<_>>();

        let textures = channels
            .into_iter()
            .map(|c| c.into().internal().to_texture(&shape, 1).unwrap())
            .collect::<Vec<_>>();

        assert!(
            textures.windows(2).all(|w| w[0].size() == w[1].size()),
            "All channels of a texture have to have the same size!"
        );

        Self { textures, shape }
    }
    pub fn eval3<V: rjit::AsVarType>(&self, pos: &Vec3<V>) -> Vec<Float32> {
        let results = self
            .textures
            .iter()
            .map(|t| {
                Var(
                    t.tex_lookup(&[&pos.x.internal(), &pos.y.internal(), &pos.z.internal()])
                        .unwrap()[0]
                        .clone(),
                    PhantomData,
                )
            })
            .collect::<Vec<_>>();

        results
    }
    pub fn eval2<V: rjit::AsVarType>(&self, pos: &Vec2<V>) -> Vec<Float32> {
        let results = self
            .textures
            .iter()
            .map(|t| {
                Var(
                    t.tex_lookup(&[&pos.x.internal(), &pos.y.internal()])
                        .unwrap()[0]
                        .clone(),
                    PhantomData,
                )
            })
            .collect::<Vec<_>>();

        results
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tex2() {
        pretty_env_logger::try_init().ok();
        set_backend(["optix"]).unwrap();

        let val = Float32::from(vec![1.; 100].as_slice());
        let tex = Texture::new([10, 10], [val]);

        let x = tex.eval2(&vec2!(0.5, 0.5))[0].clone();

        x.schedule();
        eval();

        let z: Vec<_> = x.into();
        dbg!(z);
    }
}
