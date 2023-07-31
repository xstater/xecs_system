use std::marker::PhantomData;
use crate::System;

pub struct Second<S,T> {
    pub(crate) system: S,
    pub(crate) _marker: PhantomData<T>
}

impl<S,T> System for Second<S,T>
where S: System{
    type In = (T, S::In);

    type Out = (T, S::Out);

    async fn step(&mut self, (value, input): Self::In) -> Self::Out {
        (value, self.system.step(input).await)
    }
}

#[cfg(test)]
mod test {
    use crate::{constant, ArrowSystem, System};

    #[tokio::test]
    async fn basic_test() {
        let mut f = constant(0).second();
        let result = f.step(('c', ())).await;
        assert_eq!(result, ('c', 0));
    }
}
