use super::Pipe;

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
    let mut pipe = Doubler;
    assert_eq!(pipe.produce(2), 4);
}

#[test]
fn connect() {
    let mut pipe = Doubler.pipe_default::<Tripler>();
    assert_eq!(pipe.produce(2), 12);
}

#[test]
fn map() {
    let mut pipe = Tripler.map(|input| input + 1);
    assert_eq!(pipe.produce(2), 7);
}
