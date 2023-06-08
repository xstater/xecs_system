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

mod constant;
pub use constant::*;

mod debug;
pub use debug::*;

pub trait System {
    type In;
    type Out;

    async fn step(&mut self, input: Self::In) -> Self::Out;


    fn chain<S2, O2>(self, s2: S2) -> Chain<Self, S2>
    where
        Self: Sized,
        S2: System<In = Self::Out, Out = O2> {
        Chain {
            system_1: self,
            system_2: s2,
        }
    }

    fn combine<S2>(self, s2: S2) -> Combine<Self,S2> 
    where
        Self: Sized,
        S2: System{
        Combine { system_1: self, system_2: s2 }
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::{from_fn, System, from_async_fn};

    #[tokio::test]
    async fn chain_fn_test(){
        let mut f = from_fn(|x| x + 1).chain(from_fn(|x| x - 1));
        let mut rng = rand::thread_rng();
        for _ in 0..100_000 {
            let x = rng.gen_range(std::u8::MIN..std::u8::MAX);
            assert_eq!(x, f.step(x).await)
        }
    }

    #[tokio::test]
    async fn chain_async_test() {
        async fn id(x: u32) -> u32 {
            x
        }

        let mut f = from_fn(|x| x + 1).chain(from_async_fn(id));
        let mut rng = rand::thread_rng();
        for _ in 0..100_000 {
            let x = rng.gen_range(std::u32::MIN..std::u32::MAX);
            assert_eq!(x + 1, f.step(x).await)
        }
    }
}