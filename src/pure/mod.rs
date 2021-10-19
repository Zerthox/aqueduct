mod connect;
mod forward;
mod function;
mod split;

use crate::result::TryWrapper;

pub use connect::Connector;
pub use forward::Forwarder;
pub use function::Function;
pub use split::Splitter;

/// Interface for a single simple element in the pipeline.
///
/// If your pipe may fail, you can use [`TryPipe`](crate::TryPipe) instead.
///
/// # Examples
/// ```
/// # use aqueduct::Pipe;
/// #
/// # struct Foo;
/// #
/// impl Pipe for Foo {
///    type Input = i32;
///    type Output = f64;
///
///    fn produce(&mut self, input: i32) -> f64 {
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

    /// Produces output from this pipe with a given input.
    fn produce(&mut self, input: Self::Input) -> Self::Output;

    fn as_try<Err>(self) -> TryWrapper<Self, Err> {
        TryWrapper::new(self)
    }

    /// Connects this pipe to another [`Pipe`].
    ///
    /// Next pipe must accept the output of this pipe as input.
    fn pipe<Next>(self, next: Next) -> Connector<Self, Next>
    where
        Next: Pipe<Input = Self::Output>,
    {
        Connector::new(self, next)
    }

    /// Connects this pipe to the [`Default`] of another [`Pipe`].
    ///
    /// Equivalent to to:
    /// ```ignore
    /// pipe.pipe(Next::default())
    /// ```
    fn pipe_default<Next>(self) -> Connector<Self, Next>
    where
        Next: Default + Pipe<Input = Self::Output>,
    {
        self.pipe(Next::default())
    }

    /// Connects this pipe to a function.
    fn map<F, O>(self, function: F) -> Connector<Self, Function<F, Self::Output, O>>
    where
        F: FnMut(Self::Output) -> O,
    {
        self.pipe(function.into())
    }
}

#[cfg(test)]
pub mod tests;
