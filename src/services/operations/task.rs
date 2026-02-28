use std::time::Duration;

use reqwest::Method;
use tokio::time::{Instant, sleep};

use crate::client::PveClient;
use crate::core::transport::enc;
use crate::error::PveError;
use crate::models::{TaskLogLine, TaskStatus};
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn task_status(&self, node: &str, upid: &str) -> Result<TaskStatus, PveError> {
        let path = format!("/nodes/{}/tasks/{}/status", enc(node), enc(upid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn task_log(
        &self,
        node: &str,
        upid: &str,
        start: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Vec<TaskLogLine>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("start", start.map(|v| v.to_string()));
        query.insert_opt("limit", limit.map(|v| v.to_string()));

        let path = format!("/nodes/{}/tasks/{}/log", enc(node), enc(upid));
        self.send(Method::GET, &path, Some(&query), None).await
    }

    pub async fn task_log_with(
        &self,
        node: &str,
        upid: &str,
        query: &requests::TaskLogQuery,
    ) -> Result<Vec<TaskLogLine>, PveError> {
        let params = query.to_params();
        let path = format!("/nodes/{}/tasks/{}/log", enc(node), enc(upid));
        self.send(Method::GET, &path, Some(&params), None).await
    }

    pub async fn wait_for_task(
        &self,
        node: &str,
        upid: &str,
        poll_interval: Duration,
        timeout: Option<Duration>,
    ) -> Result<TaskStatus, PveError> {
        let started = Instant::now();

        loop {
            let status = self.task_status(node, upid).await?;
            if status.status == "stopped" {
                if status.exitstatus.as_deref() == Some("OK") {
                    return Ok(status);
                }

                return Err(PveError::TaskFailed {
                    upid: upid.to_string(),
                    exitstatus: status
                        .exitstatus
                        .clone()
                        .unwrap_or_else(|| "UNKNOWN".to_string()),
                });
            }

            if let Some(timeout) = timeout
                && started.elapsed() > timeout
            {
                return Err(PveError::TaskTimeout {
                    upid: upid.to_string(),
                    timeout_secs: timeout.as_secs(),
                });
            }

            sleep(poll_interval).await;
        }
    }

    pub async fn wait_for_task_with_options(
        &self,
        node: &str,
        upid: &str,
        options: &requests::WaitTaskOptions,
    ) -> Result<TaskStatus, PveError> {
        self.wait_for_task(node, upid, options.poll_interval, options.timeout)
            .await
    }
}
