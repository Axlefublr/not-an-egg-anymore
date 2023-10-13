use std::future::Future;
use std::process::Output;
use std::thread;
use std::time::Duration;

use future::MeasurableFuture;

mod future;
mod traits;

fn main() {
    let to_measure = MeasurableFuture::new(example());
    // Imagine that we're using an async runtime to do this
    // Not included because it's not the point of the example
    let (_, duration) = to_measure.await;
    println!("Took {} nanoseconds", duration.as_nanos());
}

async fn example() -> impl Future<Output = ()> {
    async {
        thread::sleep(Duration::from_millis(3));
    }
}
