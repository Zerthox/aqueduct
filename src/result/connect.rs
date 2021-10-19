use super::TryPipe;

/// Connection between two [`TryPipe`] elements.
///
/// If the first pipe errors, the second pipe will not produce and the error will be forwarded.
#[derive(Debug, Clone, Default)]
pub struct TryConnector<Prev, Next>
where
    Prev: TryPipe,
    Next: TryPipe<Input = Prev::Output>,
    Next::Error: From<Prev::Error>,
{
    previous: Prev,
    next: Next,
}

impl<Prev, Next> TryConnector<Prev, Next>
where
    Prev: TryPipe,
    Next: TryPipe<Input = Prev::Output>,
    Next::Error: From<Prev::Error>,
{
    /// Creates a new connector.
    pub fn new(previous: Prev, next: Next) -> Self {
        Self { previous, next }
    }
}

impl<Prev, Next> TryPipe for TryConnector<Prev, Next>
where
    Prev: TryPipe,
    Next: TryPipe<Input = Prev::Output>,
    Next::Error: From<Prev::Error>,
{
    type Input = Prev::Input;
    type Output = Next::Output;
    type Error = Next::Error;

    fn produce(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let input = self.previous.produce(input)?;
        self.next.produce(input)
    }
}
