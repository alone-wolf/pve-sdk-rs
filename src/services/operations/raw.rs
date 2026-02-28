use reqwest::Method;
use serde_json::Value;

use crate::client::PveClient;
use crate::error::PveError;
use crate::params::PveParams;

impl PveClient {
    pub async fn raw_json(
        &self,
        method: Method,
        path: &str,
        query: Option<&PveParams>,
        form: Option<&PveParams>,
    ) -> Result<Value, PveError> {
        self.send(method, path, query, form).await
    }

    pub async fn raw_get(&self, path: &str, query: Option<&PveParams>) -> Result<Value, PveError> {
        self.raw_json(Method::GET, path, query, None).await
    }

    pub async fn raw_post(&self, path: &str, form: Option<&PveParams>) -> Result<Value, PveError> {
        self.raw_json(Method::POST, path, None, form).await
    }

    pub async fn raw_put(&self, path: &str, form: Option<&PveParams>) -> Result<Value, PveError> {
        self.raw_json(Method::PUT, path, None, form).await
    }

    pub async fn raw_delete(
        &self,
        path: &str,
        query: Option<&PveParams>,
    ) -> Result<Value, PveError> {
        self.raw_json(Method::DELETE, path, query, None).await
    }
}
