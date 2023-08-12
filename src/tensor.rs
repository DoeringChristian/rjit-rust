use crate::var::Bool;
use crate::*;

pub struct Tensor<T: rjit::AsVarType> {
    data: Var<T>,
    shape: Vec<u32>,
}

impl<T: rjit::AsVarType> Tensor<T> {
    pub fn new(data: impl Into<Var<T>>, shape: &[u32]) -> Self {
        let data: Var<T> = data.into();
        let size = shape.iter().fold(1, |a, b| a * b);
        let data = if data.is_literal() {
            let sized: UInt64 = TRACE.sized_literal(0u64, size as _).unwrap().into();
            data.gather(sized, true)
        } else {
            data.schedule();
            eval();
            data
        };
        assert_eq!(data.size(), size as _);

        Self {
            data,
            shape: shape.into_iter().map(|s| *s).collect(),
        }
    }
    pub fn get(&self, idx: &[UInt32], mask: impl Into<Bool>) -> Var<T> {
        let mut data_idx = literal(0u32);
        assert_eq!(self.shape.len(), idx.len());

        let mut m = 1;
        for i in 0..self.shape.len() {
            data_idx = data_idx + &idx[idx.len() - i - 1] * m;
            m *= self.shape[self.shape.len() - i - 1];
        }

        self.data.gather(data_idx, mask)
    }
    pub fn put<I: rjit::AsVarType + Int, IV: Into<Var<I>>>(
        &self,
        idx: impl IntoIterator<Item = IV>,
    ) -> Var<T> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    use super::Tensor;
    #[test]
    fn from_literal() {
        pretty_env_logger::try_init().ok();
        set_backend(["optix"]).unwrap();

        let ten = Tensor::new(array(&[0., 1., 2., 3., 4., 5., 6., 7., 8.]), &[3, 3]);

        let idx = [array(&[1]), array(&[1])];

        dbg!(&TRACE);
        let v = ten.get(&idx, true);
        v.schedule();
        eval();
        let v: Vec<_> = v.into();

        dbg!(v);
    }
}
