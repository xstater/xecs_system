use std::marker::PhantomData;

use crate::System;

pub struct Constant<In,T> {
    value: T,
    _marker: PhantomData<In>
}

impl<In,T> System for Constant<In,T> 
where T: Clone{
    type In = In;

    type Out = T;

    async fn step(&mut self, _: Self::In) -> Self::Out {
        self.value.clone()
    }
}

pub fn constant<In,T: Clone>(value: T) -> Constant<In,T>{
    Constant {value, _marker: PhantomData::default()}
}