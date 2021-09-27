mod chain;
mod function;

pub use chain::Chain;
pub use function::Function;

pub trait Pipe
where
    Self: Sized,
{
    type Input;
    type Output;

    fn run(&mut self, input: Self::Input) -> Self::Output;

    fn chain<Next>(self, next: Next) -> Chain<Self, Next>
    where
        Next: Pipe<Input = Self::Output>,
    {
        Chain::new(self, next)
    }

    fn chain_default<Next>(self) -> Chain<Self, Next>
    where
        Next: Default + Pipe<Input = Self::Output>,
    {
        self.chain(Next::default())
    }

    fn map<F, O>(self, function: F) -> Chain<Self, Function<F, Self::Output, O>>
    where
        F: FnMut(Self::Output) -> O,
    {
        self.chain(function.into())
    }
}

#[cfg(test)]
pub mod tests;
