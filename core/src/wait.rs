use crate::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};

type PollFn<T> =
    Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<Poll<T>, Error>> + Send>> + Send + Sync>;

/// The result of polling a long-running operation.
pub enum Poll<T> {
    /// The operation is still running. Includes an optional progress message.
    Pending(Option<String>),
    /// The operation completed successfully.
    Done(T),
}

/// Waiter for a long-running operation, polling until completion.
pub struct Wait<T> {
    poll_fn: PollFn<T>,
    interval: Duration,
    timeout: Duration,
}

impl<T: Send + 'static> Wait<T> {
    pub fn new<F, Fut>(poll_fn: F, interval: Duration, timeout: Duration) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Poll<T>, Error>> + Send + 'static,
    {
        Self {
            poll_fn: Box::new(move || Box::pin(poll_fn())),
            interval,
            timeout,
        }
    }

    /// Wait for the operation to complete.
    pub async fn result(self) -> Result<T, Error> {
        self.on_progress(|_| {}).await
    }

    /// Wait for the operation to complete, calling `callback` on progress updates.
    pub async fn on_progress(self, callback: impl Fn(&str)) -> Result<T, Error> {
        let start = Instant::now();

        loop {
            match (self.poll_fn)().await? {
                Poll::Done(val) => return Ok(val),
                Poll::Pending(msg) => {
                    if let Some(msg) = &msg {
                        callback(msg);
                    }
                }
            }

            if start.elapsed() > self.timeout {
                return Err(Error::Timeout("operation timed out".into()));
            }

            tokio::time::sleep(self.interval).await;
        }
    }
}
