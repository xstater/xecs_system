use std::{marker::PhantomData, future::Future};
use crate::System;


pub struct FromAsyncFn<F, In, Out> {
    f: F,
    _marker1: PhantomData<In>,
    _marker2: PhantomData<Out>,
}

impl<F, In, Fut,Out> System for FromAsyncFn<F, In, Out>
where
    F: FnMut(In) -> Fut + 'static,
    Fut: Future<Output = Out> + 'static,
    In: 'static,
    Out: 'static,
{
    type In = In;
    type Out = Out;

    async fn step(&mut self, input: In) -> Self::Out {
        (self.f)(input).await
    }
}

pub fn from_async_fn<F, In,Fut,Out>(f: F) -> FromAsyncFn<F, In, Out>
where
    F: FnMut(In) -> Fut + 'static,
    Fut: Future<Output = Out> + 'static,
    In: 'static,
    Out: 'static,
{
    FromAsyncFn {
        f,
        _marker1: PhantomData::default(),
        _marker2: PhantomData::default(),
    }
}