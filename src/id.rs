use std::marker::PhantomData;

use crate::System;

pub struct Id<T> {
    _marker: PhantomData<T>
}

impl<T> System for Id<T> {
    type In = T;

    type Out = T;

    async fn step(&mut self, input: Self::In) -> Self::Out {
        input
    }
}

pub fn id<T>() -> Id<T> {
    Id { _marker: PhantomData::default(), }
}