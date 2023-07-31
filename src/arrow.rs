mod chain;
use std::marker::PhantomData;

pub use chain::*;

mod combine;
pub use combine::*;

mod first;
pub use first::*;

mod second;
pub use second::*;

use crate::System;

pub trait ArrowSystem: System{
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

    fn first<T>(self) -> First<Self,T>
    where Self: Sized{
        First{
            system: self,
            _marker: PhantomData::default()
        }
    }

    fn second<T>(self) -> Second<Self,T>
    where Self: Sized{
        Second { 
            system: self,
            _marker: PhantomData::default()
        }
    }
}

impl<S> ArrowSystem for S 
where S: System{}


#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::{from_fn, ArrowSystem, from_async_fn, System};

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