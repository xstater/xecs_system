use std::ops::AddAssign;

use crate::System;

pub struct Sum<T> {
    value: T
}

impl<T> System for Sum<T> 
where T: AddAssign + Clone{
    type In = T;

    type Out = T;

    async fn step(&mut self, input: Self::In) -> Self::Out {
        self.value += input;
        self.value.clone()
    }
}

pub fn sum_from<T>(init: T) -> Sum<T> 
where T: AddAssign + Clone{
    Sum{ value: init }
}

pub fn sum<T>() -> Sum<T> 
where T: AddAssign + Clone + Default{
    sum_from(T::default())
}

#[cfg(test)]
mod test {
    use crate::{from_fn, System, sum};

    #[tokio::test]
    async fn basic() {
        let mut f = from_fn(|x| x + 1).chain(sum());
        let mut sum = 0;
        for _ in 0..100_000 {
            let x = f.step(0).await;
            sum += 1;
            assert_eq!(x, sum);
        }
    }
}