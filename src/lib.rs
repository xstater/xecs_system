#![feature(async_fn_in_trait)]
#![feature(future_join)]

mod into;
pub use into::*;

mod arrow;
pub use arrow::*;

mod count;
pub use count::*;

mod sum;
pub use sum::*;

mod swap;
pub use swap::*;

mod constant;
pub use constant::*;

mod debug;
pub use debug::*;

mod option;
pub use option::*;

mod result;
pub use result::*;

mod duplicate;
pub use duplicate::*;

pub trait System {
    type In;
    type Out;

    async fn step(&mut self, input: Self::In) -> Self::Out;
}
