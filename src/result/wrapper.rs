use super::TryPipe;
use crate::pure::Pipe;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct TryWrapper<P, Err>
where
    P: Pipe,
{
    inner: P,
    _error: PhantomData<Err>,
}

impl<P, Err> TryWrapper<P, Err>
where
    P: Pipe,
{
    pub fn new(pipe: P) -> Self {
        Self {
            inner: pipe,
            _error: PhantomData,
        }
    }
}

impl<P, Err> TryPipe for TryWrapper<P, Err>
where
    P: Pipe,
{
    type Input = P::Input;
    type Output = P::Output;
    type Error = Err;

    fn run(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(self.inner.run(input))
    }
}
