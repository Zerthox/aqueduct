use crate::pure::Pipe;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Function<F, I, O>
where
    F: FnMut(I) -> O,
{
    inner: F,
    _input: PhantomData<I>,
    _output: PhantomData<O>,
}

impl<F, I, O> From<F> for Function<F, I, O>
where
    F: FnMut(I) -> O,
{
    fn from(function: F) -> Self {
        Self {
            inner: function,
            _input: PhantomData,
            _output: PhantomData,
        }
    }
}

impl<F, I, O> Pipe for Function<F, I, O>
where
    F: FnMut(I) -> O,
{
    type Input = I;
    type Output = O;

    fn produce(&mut self, input: Self::Input) -> Self::Output {
        (self.inner)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut pipe = Function::from(|input| input * 4);
        assert_eq!(pipe.produce(1), 4);
    }
}
