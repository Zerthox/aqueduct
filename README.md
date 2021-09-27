# Aqueduct
Pipeline interface for chaining operations.

```toml
[dependencies]
aqueduct = { git = "https://github.com/zerthox/aqueduct", branch = "master" }
```

## Usage
Either the `Pipe` or the `TryPipe` trait needs to be implemented.

```rust
use aqueduct::Pipe;

impl Pipe for Foo {
    type Input = i32;
    type Output = f64;

    fn run(&mut self, input: i32) -> f64 {
        todo!("take input & generate output")
    }
}
```

```rust
use std::io;
use aqueduct::TryPipe;

impl TryPipe for Bar {
    type Input = i32;
    type Output = f64;
    type Error = io::Error;

    fn run(&mut self, input: i32) -> Result<f64, io::Error> {
        Err(io::Error::new(io::ErrorKind::Other, "oops"))
    }
}
```

Pipes and/or functions can then be connected into a pipeline and executed with an input.
```rust
let mut pipeline = Foo::new()
    .chain(Bar::new())
    .chain_default::<Baz>()
    .map(|input| input * 2);

let input = 123;
let result = pipeline.run(input);
```
