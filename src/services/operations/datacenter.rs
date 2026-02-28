use reqwest::Method;
use serde_json::Value;

use crate::client::PveClient;
use crate::error::PveError;
use crate::models::DatacenterConfig;
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn datacenter_config(&self) -> Result<DatacenterConfig, PveError> {
        self.send(Method::GET, "/cluster/options", None, None).await
    }

    pub async fn datacenter_update_config(&self, params: &PveParams) -> Result<(), PveError> {
        let _: Value = self
            .send(Method::PUT, "/cluster/options", None, Some(params))
            .await?;
        Ok(())
    }

    pub async fn datacenter_update_config_with(
        &self,
        request: &requests::DatacenterConfigUpdateRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.datacenter_update_config(&params).await
    }
}
