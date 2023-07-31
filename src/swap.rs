use std::marker::PhantomData;

use crate::System;

pub struct Swap<A,B> {
    _marker1: PhantomData<A>,
    _marker2: PhantomData<B>
}

impl<A,B> System for Swap<A,B> {
    type In = (A,B);

    type Out = (B,A);

    async fn step(&mut self, (a,b): Self::In) -> Self::Out {
        (b,a)
    }
}

pub fn swap<A,B>() -> Swap<A,B> {
    Swap { _marker1: PhantomData::default(), _marker2: PhantomData::default() }
}

#[cfg(test)]
mod test {
    use crate::{constant, ArrowSystem, swap, System};

    #[tokio::test]
    async fn basic_test() {
        let mut f = constant((1,2))
            .chain(swap());
        let result = f.step(()).await;
        assert_eq!(result, (2,1));
    }
}