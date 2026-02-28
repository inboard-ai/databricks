use crate::error::Error;
use std::future::Future;
use std::time::Duration;

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct Policy {
    pub max_retries: u32,
    pub timeout: Duration,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            timeout: Duration::from_secs(300),
        }
    }
}

impl Policy {
    /// Execute the given async operation with retries on retryable errors.
    ///
    /// Uses exponential backoff with jitter: `min(2^attempt, 30) + random(50ms..750ms)`.
    /// Respects `retry_after_secs` from 429 responses.
    ///
    /// Without the `hyper` feature, executes a single attempt (no retries).
    pub async fn execute<F, Fut, T>(&self, mut op: F) -> Result<T, Error>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        #[cfg(not(feature = "hyper"))]
        {
            op().await
        }

        #[cfg(feature = "hyper")]
        {
            let start = std::time::Instant::now();

            for attempt in 0..=self.max_retries {
                match op().await {
                    Ok(val) => return Ok(val),
                    Err(err) if attempt < self.max_retries && err.is_retryable() => {
                        let delay = backoff_delay(attempt, err.retry_after_secs());
                        if start.elapsed() + delay > self.timeout {
                            return Err(err);
                        }
                        tokio::time::sleep(delay).await;
                    }
                    Err(err) => return Err(err),
                }
            }

            unreachable!()
        }
    }
}

#[cfg(feature = "hyper")]
fn backoff_delay(attempt: u32, retry_after_secs: Option<u64>) -> Duration {
    if let Some(secs) = retry_after_secs {
        return Duration::from_secs(secs);
    }

    let base_secs = (1u64 << attempt).min(30);
    let jitter_ms = rand::Rng::gen_range(&mut rand::thread_rng(), 50..750);
    Duration::from_secs(base_secs) + Duration::from_millis(jitter_ms)
}
