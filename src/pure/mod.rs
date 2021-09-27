mod chain;
mod function;

pub use chain::Chain;
pub use function::Function;

/// Interface for a single simple element in the pipeline.
///
/// If your pipe may fail, you can use [`TryPipe`](crate::TryPipe) instead.
///
/// # Examples
/// ```
/// use aqueduct::Pipe;
///
/// struct Foo;
///
/// impl Pipe for Foo {
///    type Input = i32;
///    type Output = f64;
///
///    fn run(&mut self, input: i32) -> f64 {
///        todo!("take input & generate output")
///    }
/// }
/// ```
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
