use crate::System;

pub struct OptionChain<S1, S2> {
    system1: S1,
    system2: S2,
}

impl<S1, S2> System for OptionChain<S1, S2>
where
    S1: OptionSystem,
    S2: OptionSystem<In = S1::Some>,
{
    type In = S1::In;

    type Out = S2::Out;

    async fn step(&mut self, input: Self::In) -> Self::Out {
        let result = self.system1.step(input).await?;
        self.system2.step(result).await
    }
}

pub trait OptionSystem: System<Out = Option<Self::Some>> {
    type Some;

    fn option_chain<S2>(self, system: S2) -> OptionChain<Self, S2>
    where
        Self: Sized,
        S2: OptionSystem<In = Self::Some>,
    {
        OptionChain {
            system1: self,
            system2: system,
        }
    }
}

impl<S, T> OptionSystem for S
where
    S: System<Out = Option<T>>,
{
    type Some = T;
}

#[cfg(test)]
mod test {
    use crate::{System, constant, sum, from_fn, option::OptionSystem, ArrowSystem};

    struct IsEven {}

    impl System for IsEven {
        type In = i32;

        type Out = Option<i32>;

        async fn step(&mut self, input: Self::In) -> Self::Out {
            if input % 2 == 0 {
                Some(input)
            } else {
                None
            }
        }
    }

    #[tokio::test]
    async fn basic_test() {
        let mut f = constant(1)
            .chain(sum())
            .chain(IsEven{})
            .option_chain(from_fn(|x| Some(x)));
        for i in 1..100 {
            let x = 
                if i % 2 == 0 { Some(i) } else { None };
            let y = f.step(()).await;
            assert_eq!(x,y);
        }
    }
}
