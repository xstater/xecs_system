use std::marker::PhantomData;
use crate::System;

pub struct FromFn<F, In, Out> {
    f: F,
    _marker1: PhantomData<In>,
    _marker2: PhantomData<Out>,
}

impl<F, In, Out> System for FromFn<F, In, Out>
where
    F: FnMut(In) -> Out + 'static,
    In: 'static,
    Out: 'static,
{
    type In = In;
    type Out = Out;

    async fn step(&mut self, input: In) -> Self::Out {
        (self.f)(input)
    }
}

pub fn from_fn<F, In, Out>(f: F) -> FromFn<F, In, Out>
where
    F: FnMut(In) -> Out + 'static,
    In: 'static,
    Out: 'static,
{
    FromFn {
        f,
        _marker1: PhantomData::default(),
        _marker2: PhantomData::default(),
    }
}
