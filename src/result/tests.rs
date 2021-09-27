use crate::{pure::tests::Doubler, TryPipe};

#[derive(Debug, Default)]
struct Divider;

impl TryPipe for Divider {
    type Input = (i32, i32);
    type Output = i32;
    type Error = &'static str;

    fn run(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let (lhs, rhs) = input;
        lhs.checked_div(rhs).ok_or("divide by zero")
    }
}

#[derive(Debug, Default)]
struct AlwaysError;

impl TryPipe for AlwaysError {
    type Input = i32;
    type Output = i32;
    type Error = &'static str;

    fn run(&mut self, _input: Self::Input) -> Result<Self::Output, Self::Error> {
        Err("error occured")
    }
}

#[test]
fn single() {
    let mut pipeline = Divider;
    assert_eq!(pipeline.run((12, 2)), Ok(6));
    assert_eq!(pipeline.run((12, 0)), Err("divide by zero"));
}

#[test]
fn chained() {
    let mut pipeline = Divider.chain(AlwaysError);
    assert_eq!(pipeline.run((12, 0)), Err("divide by zero"));
    assert_eq!(pipeline.run((12, 2)), Err("error occured"));
}

#[test]
fn pure() {
    let mut pipeline = Divider.chain_pure(Doubler);
    assert_eq!(pipeline.run((12, 2)), Ok(12));
    assert_eq!(pipeline.run((12, 0)), Err("divide by zero"));
}

#[test]
fn function() {
    let mut pipeline = Divider.map(|input| input + 2);
    assert_eq!(pipeline.run((12, 2)), Ok(8));
    assert_eq!(pipeline.run((12, 0)), Err("divide by zero"));
}
