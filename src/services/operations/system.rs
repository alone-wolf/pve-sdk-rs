use reqwest::Method;

use crate::client::PveClient;
use crate::error::PveError;
use crate::models::VersionInfo;

impl PveClient {
    pub async fn version(&self) -> Result<VersionInfo, PveError> {
        self.send(Method::GET, "/version", None, None).await
    }
}
