// ACL management example (with SDK preflight validation)
//
// Minimal env (.env or export):
// PVE_HOST=10.0.0.2
// PVE_PORT=8006
// PVE_INSECURE_TLS=true
// PVE_AUTH_METHOD=API_TOKEN
// PVE_API_TOKEN="root@pam!ci=token-secret"
//
// Dry-run only (default):
//   cargo run --example access_acl_manage
//
// Apply ACL (will call PVE API):
// PVE_ACL_APPLY=1
// PVE_ACL_PATH=/vms
// PVE_ACL_ROLES=PVEVMAdmin
// PVE_ACL_USERS=devops@pve
// # Optional: PVE_ACL_GROUPS, PVE_ACL_TOKENS
// # Optional rollback:
// # PVE_ACL_DELETE_AFTER_SET=1
//
// Run: cargo run --example access_acl_manage

use std::env;

use pve_sdk_rs::PveError;
use pve_sdk_rs::types::access::{AccessDeleteAclRequest, AccessSetAclRequest};

mod common;
use common::{build_client_from_env, env_bool, env_required};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = build_client_from_env().await?;

    client.connect().await?;

    // Show SDK-side ACL validation with an invalid request.
    let invalid = AccessSetAclRequest::new("/vms", "PVEVMAdmin");
    match client.access().set_acl_with(&invalid).await {
        Err(PveError::InvalidArgument(msg)) => {
            println!("validation works (expected): {msg}");
        }
        Err(err) => return Err(err.into()),
        Ok(_) => {
            return Err("unexpected success for invalid acl set request".into());
        }
    }

    if !env_bool("PVE_ACL_APPLY", false) {
        println!("dry-run done. set PVE_ACL_APPLY=1 to perform real ACL writes.");
        return Ok(());
    }

    let path = env_required("PVE_ACL_PATH")?;
    let roles = env_required("PVE_ACL_ROLES")?;
    let users = env::var("PVE_ACL_USERS")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    let groups = env::var("PVE_ACL_GROUPS")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    let tokens = env::var("PVE_ACL_TOKENS")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());

    if users.is_none() && groups.is_none() && tokens.is_none() {
        return Err(
            "at least one of PVE_ACL_USERS/PVE_ACL_GROUPS/PVE_ACL_TOKENS is required".into(),
        );
    }

    let mut set = AccessSetAclRequest::new(path, roles);
    set.users = users;
    set.groups = groups;
    set.tokens = tokens;
    set.propagate = Some(env_bool("PVE_ACL_PROPAGATE", true));

    client.access().set_acl_with(&set).await?;
    println!("acl set ok");

    if env_bool("PVE_ACL_DELETE_AFTER_SET", false) {
        let mut delete = AccessDeleteAclRequest::new(set.path.clone());
        delete.roles = Some(set.roles.clone());
        delete.users = set.users.clone();
        delete.groups = set.groups.clone();
        delete.tokens = set.tokens.clone();

        client.access().delete_acl_with(&delete).await?;
        println!("acl delete rollback ok");
    }

    Ok(())
}
