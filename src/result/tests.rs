use super::TryPipe;
use crate::pure::tests::Doubler;

#[derive(Debug, Default)]
struct Divider;

impl TryPipe for Divider {
    type Input = (i32, i32);
    type Output = i32;
    type Error = &'static str;

    fn produce(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error> {
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

    fn produce(&mut self, _input: Self::Input) -> Result<Self::Output, Self::Error> {
        Err("error occured")
    }
}

#[test]
fn single() {
    let mut pipe = Divider;
    assert_eq!(pipe.produce((12, 2)), Ok(6));
    assert_eq!(pipe.produce((12, 0)), Err("divide by zero"));
}

#[test]
fn connect() {
    let mut pipe = Divider.pipe(AlwaysError);
    assert_eq!(pipe.produce((12, 0)), Err("divide by zero"));
    assert_eq!(pipe.produce((12, 2)), Err("error occured"));
}

#[test]
fn connect_pure() {
    let mut pipe = Divider.pipe_pure(Doubler);
    assert_eq!(pipe.produce((12, 2)), Ok(12));
    assert_eq!(pipe.produce((12, 0)), Err("divide by zero"));
}

#[test]
fn map() {
    let mut pipe = Divider.map(|input| input + 2);
    assert_eq!(pipe.produce((12, 2)), Ok(8));
    assert_eq!(pipe.produce((12, 0)), Err("divide by zero"));
}
