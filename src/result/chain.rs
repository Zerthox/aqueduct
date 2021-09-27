use super::TryPipe;

#[derive(Debug)]
pub struct TryChain<Prev, Next>
where
    Prev: TryPipe,
    Next: TryPipe<Input = Prev::Output>,
    Next::Error: From<Prev::Error>,
{
    previous: Prev,
    next: Next,
}

impl<Prev, Next> TryChain<Prev, Next>
where
    Prev: TryPipe,
    Next: TryPipe<Input = Prev::Output>,
    Next::Error: From<Prev::Error>,
{
    pub fn new(previous: Prev, next: Next) -> Self {
        Self { previous, next }
    }
}

impl<Prev, Next> TryPipe for TryChain<Prev, Next>
where
    Prev: TryPipe,
    Next: TryPipe<Input = Prev::Output>,
    Next::Error: From<Prev::Error>,
{
    type Input = Prev::Input;
    type Output = Next::Output;
    type Error = Next::Error;

    fn run(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let input = self.previous.run(input)?;
        self.next.run(input)
    }
}
