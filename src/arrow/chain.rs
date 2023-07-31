use crate::System;


pub struct Chain<S1,S2> {
    pub(crate) system_1: S1,
    pub(crate) system_2: S2
}


impl<S1,S2> System for Chain<S1,S2> 
where
    S1: System<Out = S2::In>,
    S2: System{

    type In = S1::In;
    type Out = S2::Out;

    async fn step(&mut self, input: Self::In) -> Self::Out{
        let s1 = self.system_1.step(input).await;
        let s2 = self.system_2.step(s1).await;
        s2
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::{from_fn, System, ArrowSystem};

    #[tokio::test]
    async fn basic() {
        let f1 = from_fn(|x| x + 1);
        let f2 = from_fn(|x: u32| x.to_string());

        let mut f = f1.chain(f2);

        let mut rng = rand::thread_rng();
        for _ in 0..100_000 {
            let x = rng.gen_range(0..std::u32::MAX - 1);
            let res = f.step(x).await;
            assert_eq!(res, (x + 1).to_string());
        }
    }

}