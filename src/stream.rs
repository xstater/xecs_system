use crate::System;

pub trait Stream: System<In = ()> {
}


impl<S> Stream for S 
where S: System<In = ()>{

}