use std::marker::PhantomData;
use crate::System;

pub struct Count<T> {
    count: usize,
    _marker: PhantomData<T>
}


impl<T> System for Count<T> {
    type In = T;

    type Out = usize;

    async fn step(&mut self, _: Self::In) -> Self::Out {
        self.count += 1;
        self.count
    }
}

pub fn count<T>() -> Count<T>{
    Count { count: 0, _marker: PhantomData::default() }
}

#[cfg(test)]
mod test {
    use crate::{from_fn, System, count, ArrowSystem};

    #[tokio::test]
    async fn basic() {
        let mut f = from_fn(|x| x).chain(count());
        for i in 1..=100_000 {
            let count = f.step(()).await;
            assert_eq!(i, count)
        }
    }
}