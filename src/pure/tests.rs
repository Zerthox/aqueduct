use crate::pure::Pipe;

#[derive(Debug, Default)]
pub struct Doubler;

impl Pipe for Doubler {
    type Input = i32;
    type Output = i32;

    fn produce(&mut self, input: Self::Input) -> Self::Output {
        input * 2
    }
}

#[derive(Debug, Default)]
struct Tripler;

impl Pipe for Tripler {
    type Input = i32;
    type Output = i32;

    fn produce(&mut self, input: Self::Input) -> Self::Output {
        input * 3
    }
}

#[test]
fn single() {
    let mut pipeline = Doubler;
    assert_eq!(pipeline.produce(2), 4);
}

#[test]
fn chained() {
    let mut pipeline = Doubler.pipe_default::<Tripler>();
    assert_eq!(pipeline.produce(2), 12);
}

#[test]
fn function() {
    let mut pipeline = Tripler.map(|input| input + 1);
    assert_eq!(pipeline.produce(2), 7);
}
