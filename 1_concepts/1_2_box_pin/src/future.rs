use std::time::Instant;

struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<Instant>,
}