use super::TryPipe;
use crate::pure::Pipe;
use std::marker::PhantomData;

/// Wrapper around a [`Pipe`], turning it into a [`TryPipe`].
#[derive(Debug, Clone, Default)]
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
    /// Creates a new wrapper.
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

    fn produce(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(self.inner.produce(input))
    }
}
