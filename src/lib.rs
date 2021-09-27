//! [`Pipe`] and [`TryPipe`] traits for easy creation of pipelines.
//!
//! ## Usage
//!
//! ```
//! use aqueduct::{Pipe, TryPipe};
//!
//! let mut pipeline = Foo::new()
//!     .chain(Bar::new())
//!     .chain_default::<Baz>()
//!     .map(|input| input * 2);
//!
//! let input = 123;
//! let result = pipeline.run(input);
//! ```

mod pure;
mod result;

pub use pure::Pipe;
pub use result::TryPipe;
