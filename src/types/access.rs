//! Access/auth related types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::params::PveParams;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TicketInfo {
    pub username: String,
    pub ticket: String,
    #[serde(rename = "CSRFPreventionToken")]
    pub csrf_prevention_token: String,
    pub clustername: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TicketRequest {
    pub username: String,
    pub password: String,
    pub otp: Option<String>,
    pub realm: Option<String>,
    pub tfa_challenge: Option<String>,
}

impl TicketRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            otp: None,
            realm: None,
            tfa_challenge: None,
        }
    }

    pub fn all(
        username: impl Into<String>,
        password: impl Into<String>,
        otp: Option<String>,
        realm: Option<String>,
        tfa_challenge: Option<String>,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            otp,
            realm,
            tfa_challenge,
        }
    }

    pub fn otp(mut self, otp: impl Into<String>) -> Self {
        self.otp = Some(otp.into());
        self
    }

    pub fn realm(mut self, realm: impl Into<String>) -> Self {
        self.realm = Some(realm.into());
        self
    }

    pub fn tfa_challenge(mut self, tfa_challenge: impl Into<String>) -> Self {
        self.tfa_challenge = Some(tfa_challenge.into());
        self
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("username", self.username.clone());
        params.insert("password", self.password.clone());
        params.insert_opt("otp", self.otp.clone());
        params.insert_opt("realm", self.realm.clone());
        params.insert_opt("tfa-challenge", self.tfa_challenge.clone());
        params
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessUser {
    pub userid: String,
    pub enable: Option<u8>,
    pub expire: Option<u64>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessGroup {
    pub groupid: String,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessRole {
    pub roleid: String,
    pub privs: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessAcl {
    pub path: Option<String>,
    pub ugid: Option<String>,
    pub roleid: Option<String>,
    pub propagate: Option<u8>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessUserToken {
    pub tokenid: String,
    pub comment: Option<String>,
    pub expire: Option<u64>,
    pub enable: Option<u8>,
    pub privsep: Option<u8>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct AccessAclQuery {
    pub path: Option<String>,
    pub exact: Option<bool>,
}

impl AccessAclQuery {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("path", self.path.clone());
        if let Some(exact) = self.exact {
            params.insert_bool("exact", exact);
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct AccessCreateTokenRequest {
    pub tokenid: String,
    pub comment: Option<String>,
    pub expire: Option<u64>,
    pub privsep: Option<bool>,
}

impl AccessCreateTokenRequest {
    pub fn new(tokenid: impl Into<String>) -> Self {
        Self {
            tokenid: tokenid.into(),
            comment: None,
            expire: None,
            privsep: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("tokenid", self.tokenid.clone());
        params.insert_opt("comment", self.comment.clone());
        params.insert_opt("expire", self.expire.map(|v| v.to_string()));
        if let Some(privsep) = self.privsep {
            params.insert_bool("privsep", privsep);
        }
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct AccessUpdateTokenRequest {
    pub comment: Option<String>,
    pub enable: Option<bool>,
    pub expire: Option<u64>,
}

impl AccessUpdateTokenRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("comment", self.comment.clone());
        if let Some(enable) = self.enable {
            params.insert_bool("enable", enable);
        }
        params.insert_opt("expire", self.expire.map(|v| v.to_string()));
        params
    }
}

#[derive(Debug, Clone)]
pub struct AccessCreateUserRequest {
    pub userid: String,
    pub password: Option<String>,
    pub comment: Option<String>,
    pub email: Option<String>,
    pub enable: Option<bool>,
    pub expire: Option<u64>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub groups: Option<String>,
    pub keys: Option<String>,
}

impl AccessCreateUserRequest {
    pub fn new(userid: impl Into<String>) -> Self {
        Self {
            userid: userid.into(),
            password: None,
            comment: None,
            email: None,
            enable: None,
            expire: None,
            firstname: None,
            lastname: None,
            groups: None,
            keys: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("userid", self.userid.clone());
        params.insert_opt("password", self.password.clone());
        params.insert_opt("comment", self.comment.clone());
        params.insert_opt("email", self.email.clone());
        if let Some(enable) = self.enable {
            params.insert_bool("enable", enable);
        }
        params.insert_opt("expire", self.expire.map(|v| v.to_string()));
        params.insert_opt("firstname", self.firstname.clone());
        params.insert_opt("lastname", self.lastname.clone());
        params.insert_opt("groups", self.groups.clone());
        params.insert_opt("keys", self.keys.clone());
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct AccessUpdateUserRequest {
    pub comment: Option<String>,
    pub email: Option<String>,
    pub enable: Option<bool>,
    pub expire: Option<u64>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub groups: Option<String>,
    pub keys: Option<String>,
    pub password: Option<String>,
}

impl AccessUpdateUserRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("comment", self.comment.clone());
        params.insert_opt("email", self.email.clone());
        if let Some(enable) = self.enable {
            params.insert_bool("enable", enable);
        }
        params.insert_opt("expire", self.expire.map(|v| v.to_string()));
        params.insert_opt("firstname", self.firstname.clone());
        params.insert_opt("lastname", self.lastname.clone());
        params.insert_opt("groups", self.groups.clone());
        params.insert_opt("keys", self.keys.clone());
        params.insert_opt("password", self.password.clone());
        params
    }
}

#[derive(Debug, Clone)]
pub struct AccessCreateGroupRequest {
    pub groupid: String,
    pub comment: Option<String>,
}

impl AccessCreateGroupRequest {
    pub fn new(groupid: impl Into<String>) -> Self {
        Self {
            groupid: groupid.into(),
            comment: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("groupid", self.groupid.clone());
        params.insert_opt("comment", self.comment.clone());
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct AccessUpdateGroupRequest {
    pub comment: Option<String>,
}

impl AccessUpdateGroupRequest {
    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert_opt("comment", self.comment.clone());
        params
    }
}

#[derive(Debug, Clone)]
pub struct AccessSetAclRequest {
    pub path: String,
    pub roles: String,
    pub users: Option<String>,
    pub groups: Option<String>,
    pub tokens: Option<String>,
    pub propagate: Option<bool>,
}

impl AccessSetAclRequest {
    pub fn new(path: impl Into<String>, roles: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            roles: roles.into(),
            users: None,
            groups: None,
            tokens: None,
            propagate: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("path", self.path.clone());
        params.insert("roles", self.roles.clone());
        params.insert_opt("users", self.users.clone());
        params.insert_opt("groups", self.groups.clone());
        params.insert_opt("tokens", self.tokens.clone());
        if let Some(propagate) = self.propagate {
            params.insert_bool("propagate", propagate);
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct AccessDeleteAclRequest {
    pub path: String,
    pub roles: Option<String>,
    pub users: Option<String>,
    pub groups: Option<String>,
    pub tokens: Option<String>,
}

impl AccessDeleteAclRequest {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            roles: None,
            users: None,
            groups: None,
            tokens: None,
        }
    }

    pub fn to_params(&self) -> PveParams {
        let mut params = PveParams::new();
        params.insert("path", self.path.clone());
        params.insert_opt("roles", self.roles.clone());
        params.insert_opt("users", self.users.clone());
        params.insert_opt("groups", self.groups.clone());
        params.insert_opt("tokens", self.tokens.clone());
        params.insert_bool("delete", true);
        params
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AccessAclQuery, AccessCreateTokenRequest, AccessCreateUserRequest, AccessDeleteAclRequest,
        AccessSetAclRequest, AccessUpdateTokenRequest, AccessUpdateUserRequest,
    };

    #[test]
    fn access_acl_query_maps_bool() {
        let query = AccessAclQuery {
            path: Some("/".to_string()),
            exact: Some(true),
        };
        let params = query.to_params();
        assert_eq!(params.get("path"), Some("/"));
        assert_eq!(params.get("exact"), Some("1"));
    }

    #[test]
    fn access_create_token_maps_fields() {
        let mut req = AccessCreateTokenRequest::new("ci");
        req.comment = Some("robot".to_string());
        req.privsep = Some(false);
        let params = req.to_params();
        assert_eq!(params.get("tokenid"), Some("ci"));
        assert_eq!(params.get("comment"), Some("robot"));
        assert_eq!(params.get("privsep"), Some("0"));
    }

    #[test]
    fn access_update_token_maps_optional_fields() {
        let req = AccessUpdateTokenRequest {
            comment: Some("rotated".to_string()),
            enable: Some(true),
            expire: Some(3600),
        };
        let params = req.to_params();
        assert_eq!(params.get("comment"), Some("rotated"));
        assert_eq!(params.get("enable"), Some("1"));
        assert_eq!(params.get("expire"), Some("3600"));
    }

    #[test]
    fn access_create_user_maps_fields() {
        let mut req = AccessCreateUserRequest::new("dev@pve");
        req.password = Some("secret".to_string());
        req.enable = Some(true);
        req.groups = Some("devops".to_string());
        let params = req.to_params();
        assert_eq!(params.get("userid"), Some("dev@pve"));
        assert_eq!(params.get("password"), Some("secret"));
        assert_eq!(params.get("enable"), Some("1"));
        assert_eq!(params.get("groups"), Some("devops"));
    }

    #[test]
    fn access_update_user_maps_optional_fields() {
        let req = AccessUpdateUserRequest {
            comment: Some("updated".to_string()),
            email: None,
            enable: Some(false),
            expire: Some(99),
            firstname: None,
            lastname: None,
            groups: None,
            keys: None,
            password: None,
        };
        let params = req.to_params();
        assert_eq!(params.get("comment"), Some("updated"));
        assert_eq!(params.get("enable"), Some("0"));
        assert_eq!(params.get("expire"), Some("99"));
    }

    #[test]
    fn access_set_acl_maps_required_fields() {
        let mut req = AccessSetAclRequest::new("/vms", "PVEVMAdmin");
        req.users = Some("dev@pve".to_string());
        req.propagate = Some(true);
        let params = req.to_params();
        assert_eq!(params.get("path"), Some("/vms"));
        assert_eq!(params.get("roles"), Some("PVEVMAdmin"));
        assert_eq!(params.get("users"), Some("dev@pve"));
        assert_eq!(params.get("propagate"), Some("1"));
    }

    #[test]
    fn access_delete_acl_sets_delete_flag() {
        let mut req = AccessDeleteAclRequest::new("/vms");
        req.users = Some("dev@pve".to_string());
        let params = req.to_params();
        assert_eq!(params.get("path"), Some("/vms"));
        assert_eq!(params.get("users"), Some("dev@pve"));
        assert_eq!(params.get("delete"), Some("1"));
    }
}
