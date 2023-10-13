use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::time::Instant;
use pin_project::pin_project;

#[pin_project]
struct MeasurableFuture<Fut: Future> {
    #[pin]
    inner_future: Fut,
    started_at: Option<Instant>,
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        let start = this.started_at.get_or_insert_with(Instant::now);
        let inner_poll = this.inner_future.as_mut().poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}
