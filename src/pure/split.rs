use crate::Pipe;

#[derive(Debug, Clone, Default)]
pub struct Splitter<Left, Right>
where
    Left: Pipe,
    Right: Pipe,
{
    left: Left,
    right: Right,
}

impl<Left, Right> Splitter<Left, Right>
where
    Left: Pipe,
    Right: Pipe,
{
    pub fn new(left: Left, right: Right) -> Self {
        Self { left, right }
    }
}

impl<Left, Right> Pipe for Splitter<Left, Right>
where
    Left: Pipe,
    Right: Pipe,
{
    type Input = (Left::Input, Right::Input);
    type Output = (Left::Output, Right::Output);

    fn produce(&mut self, input: Self::Input) -> Self::Output {
        let (left, right) = input;
        (self.left.produce(left), self.right.produce(right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure::Function;

    #[test]
    fn simple() {
        let mut pipe = Splitter::new(
            Function::from(|input| input + 1),
            Function::from(|input| input - 1),
        );
        assert_eq!(pipe.produce((123, 456)), (124, 455));
    }
}
