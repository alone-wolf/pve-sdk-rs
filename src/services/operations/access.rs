use reqwest::Method;
use serde_json::Value;

use crate::client::PveClient;
use crate::core::transport::enc;
use crate::error::PveError;
use crate::models::{AccessAcl, AccessGroup, AccessRole, AccessUser, AccessUserToken};
use crate::params::PveParams;
use crate::requests;

impl PveClient {
    pub async fn access_users(&self) -> Result<Vec<AccessUser>, PveError> {
        self.send(Method::GET, "/access/users", None, None).await
    }

    pub async fn access_user(&self, userid: &str) -> Result<AccessUser, PveError> {
        let path = format!("/access/users/{}", enc(userid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn access_create_user(&self, params: &PveParams) -> Result<Value, PveError> {
        self.send(Method::POST, "/access/users", None, Some(params))
            .await
    }

    pub async fn access_create_user_with(
        &self,
        request: &requests::AccessCreateUserRequest,
    ) -> Result<Value, PveError> {
        let params = request.to_params();
        self.access_create_user(&params).await
    }

    pub async fn access_update_user(
        &self,
        userid: &str,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/access/users/{}", enc(userid));
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn access_update_user_with(
        &self,
        userid: &str,
        request: &requests::AccessUpdateUserRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_update_user(userid, &params).await
    }

    pub async fn access_delete_user(&self, userid: &str) -> Result<Value, PveError> {
        let path = format!("/access/users/{}", enc(userid));
        self.send(Method::DELETE, &path, None, None).await
    }

    pub async fn access_groups(&self) -> Result<Vec<AccessGroup>, PveError> {
        self.send(Method::GET, "/access/groups", None, None).await
    }

    pub async fn access_group(&self, groupid: &str) -> Result<AccessGroup, PveError> {
        let path = format!("/access/groups/{}", enc(groupid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn access_create_group(&self, params: &PveParams) -> Result<Value, PveError> {
        self.send(Method::POST, "/access/groups", None, Some(params))
            .await
    }

    pub async fn access_create_group_with(
        &self,
        request: &requests::AccessCreateGroupRequest,
    ) -> Result<Value, PveError> {
        let params = request.to_params();
        self.access_create_group(&params).await
    }

    pub async fn access_update_group(
        &self,
        groupid: &str,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/access/groups/{}", enc(groupid));
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn access_update_group_with(
        &self,
        groupid: &str,
        request: &requests::AccessUpdateGroupRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_update_group(groupid, &params).await
    }

    pub async fn access_delete_group(&self, groupid: &str) -> Result<Value, PveError> {
        let path = format!("/access/groups/{}", enc(groupid));
        self.send(Method::DELETE, &path, None, None).await
    }

    pub async fn access_roles(&self) -> Result<Vec<AccessRole>, PveError> {
        self.send(Method::GET, "/access/roles", None, None).await
    }

    pub async fn access_acl(
        &self,
        path: Option<&str>,
        exact: Option<bool>,
    ) -> Result<Vec<AccessAcl>, PveError> {
        let mut query = PveParams::new();
        query.insert_opt("path", path);
        if let Some(exact) = exact {
            query.insert_bool("exact", exact);
        }
        self.send(Method::GET, "/access/acl", Some(&query), None)
            .await
    }

    pub async fn access_acl_with(
        &self,
        query: &requests::AccessAclQuery,
    ) -> Result<Vec<AccessAcl>, PveError> {
        let params = query.to_params();
        self.send(Method::GET, "/access/acl", Some(&params), None)
            .await
    }

    pub async fn access_set_acl(&self, params: &PveParams) -> Result<(), PveError> {
        validate_acl_params(params)?;
        let _: Value = self
            .send(Method::PUT, "/access/acl", None, Some(params))
            .await?;
        Ok(())
    }

    pub async fn access_set_acl_with(
        &self,
        request: &requests::AccessSetAclRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_set_acl(&params).await
    }

    pub async fn access_delete_acl_with(
        &self,
        request: &requests::AccessDeleteAclRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_set_acl(&params).await
    }

    pub async fn access_user_tokens(&self, userid: &str) -> Result<Vec<AccessUserToken>, PveError> {
        let path = format!("/access/users/{}/token", enc(userid));
        self.send(Method::GET, &path, None, None).await
    }

    pub async fn access_create_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &PveParams,
    ) -> Result<Value, PveError> {
        let path = format!("/access/users/{}/token", enc(userid));
        let mut body = params.clone();
        body.insert("tokenid", tokenid);
        self.send(Method::POST, &path, None, Some(&body)).await
    }

    pub async fn access_create_user_token_with(
        &self,
        userid: &str,
        request: &requests::AccessCreateTokenRequest,
    ) -> Result<Value, PveError> {
        let params = request.to_params();
        self.access_create_user_token(userid, &request.tokenid, &params)
            .await
    }

    pub async fn access_update_user_token(
        &self,
        userid: &str,
        tokenid: &str,
        params: &PveParams,
    ) -> Result<(), PveError> {
        let path = format!("/access/users/{}/token/{}", enc(userid), enc(tokenid));
        let _: Value = self.send(Method::PUT, &path, None, Some(params)).await?;
        Ok(())
    }

    pub async fn access_update_user_token_with(
        &self,
        userid: &str,
        tokenid: &str,
        request: &requests::AccessUpdateTokenRequest,
    ) -> Result<(), PveError> {
        let params = request.to_params();
        self.access_update_user_token(userid, tokenid, &params)
            .await
    }

    pub async fn access_delete_user_token(
        &self,
        userid: &str,
        tokenid: &str,
    ) -> Result<Value, PveError> {
        let path = format!("/access/users/{}/token/{}", enc(userid), enc(tokenid));
        self.send(Method::DELETE, &path, None, None).await
    }
}

fn validate_acl_params(params: &PveParams) -> Result<(), PveError> {
    fn has_non_empty(params: &PveParams, key: &str) -> bool {
        params.get(key).is_some_and(|v| !v.trim().is_empty())
    }

    if !has_non_empty(params, "path") {
        return Err(PveError::InvalidArgument(
            "access acl requires non-empty path".to_string(),
        ));
    }

    let delete_acl = params
        .get("delete")
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
    let has_target = has_non_empty(params, "users")
        || has_non_empty(params, "groups")
        || has_non_empty(params, "tokens");

    if delete_acl {
        let has_roles = has_non_empty(params, "roles");
        if !(has_roles || has_target) {
            return Err(PveError::InvalidArgument(
                "access acl delete requires at least one of roles/users/groups/tokens".to_string(),
            ));
        }
        return Ok(());
    }

    if !has_non_empty(params, "roles") {
        return Err(PveError::InvalidArgument(
            "access acl set requires non-empty roles".to_string(),
        ));
    }
    if !has_target {
        return Err(PveError::InvalidArgument(
            "access acl set requires at least one of users/groups/tokens".to_string(),
        ));
    }
    Ok(())
}
