use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Instant;

struct MeasurableFuture<Fut: Future> {
    inner_future: Fut,
    started_at: Option<Instant>,
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: projecting to a field of this struct is safe
        let measurable_future = unsafe { self.get_unchecked_mut() };

        // Safety: this directly re-polls the inner future and doesn't move it.
        let inner_fut = unsafe { Pin::new_unchecked(&mut measurable_future.inner_future) };

        match inner_fut.poll(cx) {
            Poll::Ready(output) => {
                let duration = measurable_future
                    .started_at
                    .unwrap_or_else(Instant::now)
                    .elapsed();
                println!("Execution time in nanoseconds: {}", duration.as_nanos());
                Poll::Ready(output)
            }
            Poll::Pending => {
                if measurable_future.started_at.is_none() {
                    measurable_future.started_at = Some(Instant::now());
                }
                Poll::Pending
            }
        }
    }
}
