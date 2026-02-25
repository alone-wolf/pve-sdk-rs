//! Datacenter related request/response types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatacenterConfig {
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct DatacenterConfigUpdateRequest {
    pub keyboard: Option<String>,
    pub language: Option<String>,
    pub migration: Option<String>,
    pub console: Option<String>,
    pub email_from: Option<String>,
    pub max_workers: Option<u32>,
    pub next_id: Option<u32>,
    pub extra: PveParams,
}

impl DatacenterConfigUpdateRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("keyboard", self.keyboard.clone());
        params.insert_opt("language", self.language.clone());
        params.insert_opt("migration", self.migration.clone());
        params.insert_opt("console", self.console.clone());
        params.insert_opt("email-from", self.email_from.clone());
        params.insert_opt("max_workers", self.max_workers.map(|v| v.to_string()));
        params.insert_opt("next-id", self.next_id.map(|v| v.to_string()));
        params.extend(&self.extra);
        params
    }
}

#[cfg(test)]
mod tests {
    use super::DatacenterConfigUpdateRequest;

    #[test]
    fn datacenter_update_config_maps_known_fields() {
        let req = DatacenterConfigUpdateRequest {
            keyboard: Some("en-us".to_string()),
            language: Some("en".to_string()),
            migration: None,
            console: None,
            email_from: Some("noreply@example.com".to_string()),
            max_workers: Some(8),
            next_id: Some(200),
            extra: Default::default(),
        };
        let params = req.to_params();
        assert_eq!(params.get("keyboard"), Some("en-us"));
        assert_eq!(params.get("language"), Some("en"));
        assert_eq!(params.get("email-from"), Some("noreply@example.com"));
        assert_eq!(params.get("max_workers"), Some("8"));
        assert_eq!(params.get("next-id"), Some("200"));
    }
}
