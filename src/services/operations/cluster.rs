use reqwest::Method;

use crate::client::PveClient;
use crate::error::PveError;
use crate::models::{ClusterResource, ClusterStatusItem, NodeSummary};
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn nodes(&self) -> Result<Vec<NodeSummary>, PveError> {
        self.send(Method::GET, "/nodes", None, None).await
    }

    pub async fn cluster_status(&self) -> Result<Vec<ClusterStatusItem>, PveError> {
        self.send(Method::GET, "/cluster/status", None, None).await
    }

    pub async fn cluster_resources(
        &self,
        resource_type: Option<&str>,
    ) -> Result<Vec<ClusterResource>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("type", resource_type);
        self.send(Method::GET, "/cluster/resources", Some(&query), None)
            .await
    }

    pub async fn cluster_resources_with(
        &self,
        query: &requests::ClusterResourcesQuery,
    ) -> Result<Vec<ClusterResource>, PveError> {
        let params = query.to_params();
        self.send(Method::GET, "/cluster/resources", Some(&params), None)
            .await
    }

    pub async fn cluster_next_id(&self, vmid: Option<u32>) -> Result<u32, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("vmid", vmid.map(|v| v.to_string()));
        self.send(Method::GET, "/cluster/nextid", Some(&query), None)
            .await
    }
}
