use crate::System;

pub struct Combine<S1,S2> {
    pub(crate) system_1: S1,
    pub(crate) system_2: S2
}

impl<S1,S2> System for Combine<S1,S2> 
where
    S1: System,
    S2: System{
    type In = (S1::In, S2::In);

    type Out = (S1::Out, S2::Out);

    async fn step(&mut self, (in1, in2): Self::In) -> Self::Out {

        let f1 = self.system_1.step(in1);
        let f2 = self.system_2.step(in2);
        std::future::join!(f1,f2).await
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::{from_fn, System, ArrowSystem};


    #[tokio::test]
    async fn basic() {
        let f1 = from_fn(|x| x + 1);
        let f2 = from_fn(|x| x);
        let mut f = f1.combine(f2);

        let mut rng = rand::thread_rng();
        for _ in 0..100_000 {
            let x = rng.gen_range(u32::MIN..u32::MAX);
            let (r1,r2) = f.step((x,x)).await;
            assert_eq!(r1, x + 1);
            assert_eq!(r2, x);
        }
    }
}