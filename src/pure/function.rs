use crate::pure::Pipe;
use std::marker::PhantomData;

#[derive(Debug)]
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

    fn run(&mut self, input: Self::Input) -> Self::Output {
        (self.inner)(input)
    }
}