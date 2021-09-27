use super::Pipe;

#[derive(Debug)]
pub struct Chain<Prev, Next>
where
    Prev: Pipe,
    Next: Pipe<Input = Prev::Output>,
{
    previous: Prev,
    next: Next,
}

impl<Prev, Next> Chain<Prev, Next>
where
    Prev: Pipe,
    Next: Pipe<Input = Prev::Output>,
{
    pub fn new(previous: Prev, next: Next) -> Self {
        Self { previous, next }
    }
}

impl<Prev, Next> Pipe for Chain<Prev, Next>
where
    Prev: Pipe,
    Next: Pipe<Input = Prev::Output>,
{
    type Input = Prev::Input;
    type Output = Next::Output;

    fn run(&mut self, input: Self::Input) -> Self::Output {
        let input = self.previous.run(input);
        self.next.run(input)
    }
}
