//! [`Pipe`] and [`TryPipe`] traits for easy creation of pipelines.
//!
//! ## Usage
//!
//! ```
//! use aqueduct::{Pipe, TryPipe};
//!
//! # #[derive(Default)]
//! # struct Foo;
//! # impl Foo {
//! #     fn new() -> Self { Self }
//! # }
//! # impl Pipe for Foo {
//! #     type Input = i32;
//! #     type Output = i32;
//! #     fn produce(&mut self, input: i32) -> i32 { input }
//! # }
//! #
//! # type Bar = Foo;
//! # type Baz = Foo;
//! #
//! let mut pipeline = Foo::new()
//!     .pipe(Bar::new())
//!     .pipe_default::<Baz>()
//!     .map(|input| input * 2);
//!
//! let input = 123;
//! let result = pipeline.produce(input);
//! ```

mod pure;
mod result;

pub use pure::Pipe;
pub use result::TryPipe;
