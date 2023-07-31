use crate::System;

pub trait Sink: System<Out = ()> {
    
}

impl<S> Sink for S 
where S: System<Out = ()>{

}