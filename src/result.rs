use crate::System;

pub struct ResultChain<S1, S2> {
    system1: S1,
    system2: S2,
}

impl<S1, S2> System for ResultChain<S1, S2>
where
    S1: ResultSystem,
    S2: ResultSystem<In = S1::Ok, Error = S1::Error>,
{
    type In = S1::In;

    type Out = S2::Out;

    async fn step(&mut self, input: Self::In) -> Self::Out {
        let result = self.system1.step(input).await?;
        self.system2.step(result).await
    }
}

pub trait ResultSystem: System<Out = Result<Self::Ok,Self::Error>> {
    type Ok;
    type Error;

    fn result_chain<S2>(self, system: S2) -> ResultChain<Self,S2>
    where
        Self: Sized,
        S2: ResultSystem<In = Self::Ok, Error = Self::Error>,
    {
        ResultChain { 
            system1: self,
            system2: system,
        }
    }
}

impl<S, T, E> ResultSystem for S
where
    S: System<Out = Result<T,E>>
{
    type Ok = T;

    type Error = E;
}

#[cfg(test)]
mod test{
    use crate::{from_fn, constant, System, sum, ResultSystem, ArrowSystem};

    #[tokio::test]
    async fn basic() {
        let mut f = constant(1)
            .chain(sum())
            .chain(from_fn(|x| Ok(x)))
            .result_chain(from_fn(|x| 
                if x % 2 == 0 { 
                    Ok(x) 
                } else { 
                    Err(x) 
                }))
            .chain(from_fn(|result: Result<i32, i32>| result.map_err(|x| format!("{x} is not an even number"))));
        for i in 1..100 {
            let x = 
                if i % 2 == 0 { Ok(i) } else { Err( format!("{i} is not an even number")) };
            let y = f.step(()).await;
            assert_eq!(x,y);
        }
    }
}
