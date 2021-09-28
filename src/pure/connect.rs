use super::Pipe;

#[derive(Debug)]
pub struct Connector<Prev, Next>
where
    Prev: Pipe,
    Next: Pipe<Input = Prev::Output>,
{
    previous: Prev,
    next: Next,
}

impl<Prev, Next> Connector<Prev, Next>
where
    Prev: Pipe,
    Next: Pipe<Input = Prev::Output>,
{
    pub fn new(previous: Prev, next: Next) -> Self {
        Self { previous, next }
    }
}

impl<Prev, Next> Pipe for Connector<Prev, Next>
where
    Prev: Pipe,
    Next: Pipe<Input = Prev::Output>,
{
    type Input = Prev::Input;
    type Output = Next::Output;

    fn produce(&mut self, input: Self::Input) -> Self::Output {
        let input = self.previous.produce(input);
        self.next.produce(input)
    }
}
