use std::marker::PhantomData;

use crate::System;

pub struct Duplicate<T> {
    _marker: PhantomData<T>,
}

impl<T> System for Duplicate<T>
where
    T: Clone,
{
    type In = T;

    type Out = (T, T);

    async fn step(&mut self, input: Self::In) -> Self::Out {
        (input.clone(), input)
    }
}

pub struct DuplicateBy<T, OutContainer> {
    count: usize,
    _marker1: PhantomData<T>,
    _marker2: PhantomData<OutContainer>,
}

impl<T, OutContainer> System for DuplicateBy<T, OutContainer>
where
    T: Clone,
    OutContainer: FromIterator<T>,
{
    type In = T;

    type Out = OutContainer;

    async fn step(&mut self, input: Self::In) -> Self::Out {
        OutContainer::from_iter(std::iter::repeat(input).take(self.count))
    }
}

pub fn duplicate<T: Clone>() -> Duplicate<T> {
    Duplicate {
        _marker: PhantomData::default(),
    }
}

pub fn duplicate_by<T, OutContainer>(count: usize) -> DuplicateBy<T, OutContainer>
where
    T: Clone,
    OutContainer: FromIterator<T>,
{
    DuplicateBy {
        count,
        _marker1: PhantomData::default(),
        _marker2: PhantomData::default(),
    }
}

#[cfg(test)]
mod test {
    use crate::{constant, ArrowSystem, duplicate, System, duplicate_by};

    #[tokio::test]
    async fn duplicate_test() {
        let mut f = constant("test".to_string())
            .chain(duplicate());
        let result = f.step(()).await;
        assert_eq!(result, ("test".to_string(),"test".to_string()));
    }

    #[tokio::test]
    async fn duplicate_by_test() {
        let mut f = constant("test".to_string())
            .chain(duplicate_by(10));
        let results: Vec<_> = f.step(()).await;
        for result in results {
            assert_eq!(result, "test".to_string());
        }
    }
}
