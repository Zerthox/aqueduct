use crate::Pipe;
use std::marker::PhantomData;

/// Simple generic pipe forwarding its input.
#[derive(Debug, Clone, Default)]
pub struct Forward<Data>(PhantomData<Data>);

impl<Data> Forward<Data> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Data> Pipe for Forward<Data> {
    type Input = Data;
    type Output = Data;

    fn produce(&mut self, input: Self::Input) -> Self::Output {
        input
    }
}
