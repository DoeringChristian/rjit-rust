use crate::*;

pub struct Tensor<T: rjit::AsVarType> {
    data: Var<T>,
    shape: Vec<usize>,
}

impl<T: rjit::AsVarType> Tensor<T> {
    pub fn new(data: impl Into<Var<T>>, shape: &[usize]) -> Self {
        let data: Var<T> = data.into();
        let size = shape.iter().fold(1, |a, b| a * b);
        let data = if data.is_literal() {
            let sized: UInt32 = TRACE.sized_literal(0u32, size).unwrap().into();
            data.gather(sized, Some(true))
        } else {
            data
        };
        assert_eq!(data.size(), size);

        Self {
            data,
            shape: shape.into_iter().map(|s| *s).collect(),
        }
    }
    pub fn get<I: rjit::AsVarType + Int, IV: Into<Var<I>>>(
        &self,
        idx: impl IntoIterator<Item = IV>,
    ) -> Var<T> {
        todo!()
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

        Tensor::new(1, &[10, 10]);
    }
}
