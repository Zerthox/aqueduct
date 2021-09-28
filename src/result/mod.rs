mod connect;
mod wrap;

use crate::pure::{Function, Pipe};

pub use connect::TryConnector;
pub use wrap::TryWrapper;

/// Interface for a single pipeline element which may fail.
///
/// If your pipe cannot fail, use [`Pipe`] instead.
///
/// # Examples
///
/// ```
/// use std::io;
/// # use aqueduct::TryPipe;
/// #
/// # struct Foo;
///
/// impl TryPipe for Foo {
///     type Input = i32;
///     type Output = f64;
///     type Error = io::Error;
///
///     fn produce(&mut self, input: i32) -> Result<f64, io::Error> {
///         Err(io::Error::new(io::ErrorKind::Other, "oops"))
///     }
/// }
/// ```
pub trait TryPipe
where
    Self: Sized,
{
    type Input;
    type Output;
    type Error;

    fn produce(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn pipe<Next>(self, next: Next) -> TryConnector<Self, Next>
    where
        Next: TryPipe<Input = Self::Output>,
        Next::Error: From<Self::Error>,
    {
        TryConnector::new(self, next)
    }

    fn pipe_default<Next>(self) -> TryConnector<Self, Next>
    where
        Next: Default + TryPipe<Input = Self::Output>,
        Next::Error: From<Self::Error>,
    {
        self.pipe(Next::default())
    }

    fn pipe_pure<Next>(self, next: Next) -> TryConnector<Self, TryWrapper<Next, Self::Error>>
    where
        Next: Pipe<Input = Self::Output>,
    {
        self.pipe(TryWrapper::new(next))
    }

    fn map<F, O>(
        self,
        function: F,
    ) -> TryConnector<Self, TryWrapper<Function<F, Self::Output, O>, Self::Error>>
    where
        F: FnMut(Self::Output) -> O,
    {
        self.pipe_pure(function.into())
    }
}

#[cfg(test)]
mod tests;
