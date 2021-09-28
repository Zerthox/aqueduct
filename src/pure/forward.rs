use crate::Pipe;
use std::marker::PhantomData;

/// Simple generic pipe forwarding its input.
#[derive(Debug, Clone, Default)]
pub struct Forwarder<Data>(PhantomData<Data>);

impl<Data> Forwarder<Data> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Data> Pipe for Forwarder<Data> {
    type Input = Data;
    type Output = Data;

    fn produce(&mut self, input: Self::Input) -> Self::Output {
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut pipe = Forwarder::new();
        assert_eq!(pipe.produce(123), 123);
    }
}
