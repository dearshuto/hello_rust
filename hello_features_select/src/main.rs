use std::time::Duration;

use futures::future::Either;
use futures::{FutureExt, StreamExt};

#[tokio::main]
async fn main() {
    let f1 = async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        "future-1"
    };

    let f2 = async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        "future-2"
    };

    let res = futures::select! {
        res1 = f1.fuse() => res1,
        res2 = f2.fuse() => res2,
    };

    let mut stream0 = tokio_stream::iter(&[1, 2, 3]);
    let mut stream1 = tokio_stream::iter(&[1, 2, 3]);

    let mut x = futures::stream::select(stream0, stream1);

    let a = x.next();
    while let Some(item) = x.next() {
        match item {
            Either::Left(0) => {}
            Either::Right(0) => {}
        }
    }

    println!("{res}");
}
