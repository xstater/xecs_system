use crate::System;
use std::marker::PhantomData;

pub struct First<S, T> {
    pub(crate) system: S,
    pub(crate) _marker: PhantomData<T>,
}

impl<S, T> System for First<S, T>
where
    S: System,
{
    type In = (S::In, T);

    type Out = (S::Out, T);

    async fn step(&mut self, (input, value): Self::In) -> Self::Out {
        (self.system.step(input).await, value)
    }
}

#[cfg(test)]
mod test {
    use crate::{constant, ArrowSystem, System};

    #[tokio::test]
    async fn basic_test() {
        let mut f = constant(0).first();
        let result = f.step(((), 'c')).await;
        assert_eq!(result, (0, 'c'));
    }
}
