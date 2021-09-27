mod chain;
mod wrapper;

use crate::pure::{Function, Pipe};
pub use chain::TryChain;
pub use wrapper::TryWrapper;

pub trait TryPipe
where
    Self: Sized,
{
    type Input;
    type Output;
    type Error;

    fn run(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn chain<Next>(self, next: Next) -> TryChain<Self, Next>
    where
        Next: TryPipe<Input = Self::Output>,
        Next::Error: From<Self::Error>,
    {
        TryChain::new(self, next)
    }

    fn chain_default<Next>(self) -> TryChain<Self, Next>
    where
        Next: Default + TryPipe<Input = Self::Output>,
        Next::Error: From<Self::Error>,
    {
        self.chain(Next::default())
    }

    fn chain_pure<Next>(self, next: Next) -> TryChain<Self, TryWrapper<Next, Self::Error>>
    where
        Next: Pipe<Input = Self::Output>,
    {
        self.chain(TryWrapper::new(next))
    }

    fn map<F, O>(
        self,
        function: F,
    ) -> TryChain<Self, TryWrapper<Function<F, Self::Output, O>, Self::Error>>
    where
        F: FnMut(Self::Output) -> O,
    {
        self.chain_pure(function.into())
    }
}

#[cfg(test)]
mod tests;
