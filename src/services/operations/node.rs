use reqwest::Method;
use serde_json::Value;

use crate::client::PveClient;
use crate::core::transport::enc;
use crate::error::PveError;
use crate::models::{NetworkInterface, NodeTask};
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn node_status(&self, node: &str) -> Result<Value, PveError> {
        let path = format!("/nodes/{}/status", enc(node));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn node_tasks(
        &self,
        node: &str,
        query: &PveParams,
    ) -> Result<Vec<NodeTask>, PveError> {
        let path = format!("/nodes/{}/tasks", enc(node));
        self.send(Method::GET, &path, Some(query), None).await
    }

    pub async fn node_tasks_with(
        &self,
        node: &str,
        query: &requests::NodeTasksQuery,
    ) -> Result<Vec<NodeTask>, PveError> {
        let params = query.to_params();
        self.node_tasks(node, &params).await
    }

    pub async fn node_network(
        &self,
        node: &str,
        interface_type: Option<&str>,
    ) -> Result<Vec<NetworkInterface>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("type", interface_type);
        let path = format!("/nodes/{}/network", enc(node));
        self.send(Method::GET, &path, Some(&query), None).await
    }

    pub async fn node_network_with(
        &self,
        node: &str,
        query: &requests::NodeNetworkQuery,
    ) -> Result<Vec<NetworkInterface>, PveError> {
        let params = query.to_params();
        let path = format!("/nodes/{}/network", enc(node));
        self.send(Method::GET, &path, Some(&params), None).await
    }
}
