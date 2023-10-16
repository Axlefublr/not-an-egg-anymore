use std::future::Future;
use std::thread;
use std::time::Duration;

use future::MeasurableFuture;

mod future;
mod traits;

#[tokio::main]
async fn main() {
    let to_measure = MeasurableFuture::new(example());
    let (_, duration) = to_measure.await;
    println!("Took {} nanoseconds", duration.as_nanos());
}

async fn example() -> impl Future<Output = ()> {
    async {
        thread::sleep(Duration::from_millis(3));
    }
}