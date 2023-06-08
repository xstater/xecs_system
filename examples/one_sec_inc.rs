#![feature(async_closure)]

use std::time::Duration;

use xecs_system::{constant, System, sum, from_fn, from_async_fn};

#[tokio::main]
async fn main(){
    let mut f = constant(0)
        .chain(from_fn(|x| x + 1))
        .chain(sum())
        .chain(from_async_fn(async move |x| {
            tokio::time::sleep(Duration::from_secs(1)).await;
            x
        }))
        .chain(from_fn(|x| println!("{x}s")));

    for _ in 0..10 {
        f.step(()).await;
    }
}