use crate::types::{Empty, EmptyResponse, Request, Response, ResultData, StatementState};
use databricks_core::{Client, Error};
use std::time::Duration;
use tokio::time::sleep;

const PATH: &str = "/api/2.0/sql/statements";

pub struct Statements {
    client: Client,
}

impl Statements {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn execute(&self, request: &Request) -> Result<Response, Error> {
        self.client.post(PATH, request).await
    }

    pub async fn get(&self, statement_id: &str) -> Result<Response, Error> {
        let path = format!("{}/{}", PATH, statement_id);
        self.client.get(&path).await
    }

    pub async fn cancel(&self, statement_id: &str) -> Result<(), Error> {
        let path = format!("{}/{}/cancel", PATH, statement_id);
        let _: EmptyResponse = self.client.post(&path, &Empty {}).await?;
        Ok(())
    }

    /// Execute and poll until completion
    pub async fn execute_wait(
        &self,
        request: &Request,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Response, Error> {
        let response = self.execute(request).await?;

        if response.status.state.is_terminal() {
            return check_response(response);
        }

        self.wait(&response.statement_id, poll_interval, timeout)
            .await
    }

    /// Get a result chunk by index
    pub async fn get_result_chunk(
        &self,
        statement_id: &str,
        chunk_index: i32,
    ) -> Result<ResultData, Error> {
        let path = format!("{}/{}/result/chunks/{}", PATH, statement_id, chunk_index);
        self.client.get(&path).await
    }

    /// Poll until the statement reaches a terminal state
    pub async fn wait(
        &self,
        statement_id: &str,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Response, Error> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(Error::Timeout("Statement execution timed out".into()));
            }

            sleep(poll_interval).await;

            let response = self.get(statement_id).await?;

            if response.status.state.is_terminal() {
                return check_response(response);
            }
        }
    }
}

fn check_response(response: Response) -> Result<Response, Error> {
    match response.status.state {
        StatementState::Succeeded => Ok(response),
        StatementState::Failed => {
            let msg = response
                .status
                .error
                .and_then(|e| e.message)
                .unwrap_or_else(|| "Unknown error".into());
            Err(Error::Other(msg))
        }
        StatementState::Canceled => Err(Error::Other("Statement was canceled".into())),
        StatementState::Closed => Err(Error::Other("Statement was closed".into())),
        _ => Ok(response), // Shouldn't happen for terminal states
    }
}
