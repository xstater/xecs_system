use crate::System;

pub struct Feedback<S,T> {
    pub(crate)system : S,
    pub(crate)value: Option<T>
}

impl<S,T,A,B> System for Feedback<S,T> 
where S: System<In = (A, T), Out = (B, T)>{
    type In = A;

    type Out = B;

    async fn step(&mut self, input: Self::In) -> Self::Out {
        let value = self.value.take().unwrap_or_else(|| unreachable!("Cannot be here"));
        let (output, new_value) = self.system.step((input, value)).await;
        self.value.replace(new_value);
        output
    }
}

#[cfg(test)]
mod test {
    use crate::{from_fn, ArrowSystem, System};

    #[tokio::test]
    async fn basic_test(){
        let mut f = from_fn(|(input, count)| (input + count, count + 1))
            .feedback(0);
        let max = 100_000;
        for count in 0..max {
            let result = f.step(0).await;
            assert_eq!(count, result);
        }
    }
}