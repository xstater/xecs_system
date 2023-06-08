use std::{marker::PhantomData, fmt::Debug };

use crate::System;

pub struct Trace<T> {
    prefix: String,
    _marker: PhantomData<T>
}

impl<T> System for Trace<T> 
where T: Debug{
    type In = T;
    type Out = T;

    async fn step(&mut self, input: Self::In) -> Self::Out {
        println!("{}{:?}", &self.prefix, &input);
        input
    }
}

pub fn trace<T: Debug>() -> Trace<T>{
    Trace {
        prefix: "".to_string(),
        _marker: PhantomData::default()
    }
}

pub fn trace_with<T: Debug, S: ToString>(prefix: S) -> Trace<T>{
    Trace { prefix: prefix.to_string(), _marker: PhantomData::default() }
}