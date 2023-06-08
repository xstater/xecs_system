#![cfg(test)]

use rand::Rng;

use crate::{from_fn, System};

#[tokio::test]
async fn basic_test() {
    let mut rng = rand::thread_rng();
    let mut f = from_fn(|x| x + 1 );

    for _ in 0..10_000 {
        let x = rng.gen_range(0..u32::MAX - 1);
        assert_eq!(f.step(x).await, x + 1);
    }

    // let mut f = from_async_fn(async |x| x + 1 );

    // for _ in 0..10_000 {
    //     let x = rng.gen_range(0..u32::MAX - 1);
    //     assert_eq!(f.step(x).await, x + 1);
    // }
}
