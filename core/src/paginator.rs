use crate::client::Client;
use crate::error::Error;
use std::future::Future;
use std::pin::Pin;

type PageFetcher<T> = Box<
    dyn Fn(Client, Option<String>) -> Pin<Box<dyn Future<Output = Result<Page<T>, Error>> + Send>>
        + Send
        + Sync,
>;

/// A page of results from a paginated API call.
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_token: Option<String>,
}

/// Generic paginator for Databricks list APIs.
///
/// Buffers items from each page and yields them one at a time.
pub struct Paginator<T> {
    client: Client,
    fetcher: PageFetcher<T>,
    buffer: Vec<T>,
    next_token: Option<String>,
    done: bool,
}

impl<T: Send + 'static> Paginator<T> {
    pub fn new<F, Fut>(client: Client, fetcher: F) -> Self
    where
        F: Fn(Client, Option<String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Page<T>, Error>> + Send + 'static,
    {
        Self {
            client,
            fetcher: Box::new(move |c, t| Box::pin(fetcher(c, t))),
            buffer: Vec::new(),
            next_token: None,
            done: false,
        }
    }

    /// Get the next item, fetching a new page if needed.
    pub async fn next(&mut self) -> Option<Result<T, Error>> {
        if let Some(item) = self.buffer.pop() {
            return Some(Ok(item));
        }

        if self.done {
            return None;
        }

        match (self.fetcher)(self.client.clone(), self.next_token.take()).await {
            Ok(page) => {
                self.next_token = page.next_token;
                if self.next_token.is_none() {
                    self.done = true;
                }
                self.buffer = page.items;
                self.buffer.reverse(); // so we can pop from the end
                self.buffer.pop().map(Ok)
            }
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }

    /// Collect all remaining items into a Vec.
    pub async fn collect_all(&mut self) -> Result<Vec<T>, Error> {
        let mut results = Vec::new();
        loop {
            match self.next().await {
                Some(Ok(item)) => results.push(item),
                Some(Err(e)) => return Err(e),
                None => return Ok(results),
            }
        }
    }
}
