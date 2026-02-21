use std::env;
use std::time::Duration;

use crate::client::PveClient;
use crate::error::PveError;

#[derive(Debug, Clone)]
pub enum ClientAuth {
    None,
    ApiToken(String),
    ApiTokenPartial {
        user: String,
        realm: String,
        token_id: String,
        secret: String,
    },
    Ticket {
        ticket: String,
        csrf: Option<String>,
    },
    Password {
        username: String,
        password: String,
        otp: Option<String>,
        realm: Option<String>,
        tfa_challenge: Option<String>,
    },
}

impl ClientAuth {
    pub fn from_env() -> Result<Self, PveError> {
        let method = required_env("PVE_AUTH_METHOD")?;
        match method.as_str() {
            "API_TOKEN" => {
                let token = required_env("PVE_API_TOKEN")?;
                validate_api_token_format(&token)?;
                Ok(Self::ApiToken(token))
            }
            "API_TOKEN_PARTIAL" => {
                let user = required_env("PVE_API_TOKEN_USER")?;
                let realm = required_env("PVE_API_TOKEN_REALM")?;
                let token_id = required_env("PVE_API_TOKEN_ID")?;
                let secret = required_env("PVE_API_TOKEN_SECRET")?;
                Ok(Self::ApiTokenPartial {
                    user,
                    realm,
                    token_id,
                    secret,
                })
            }
            "USERNAME_PASSWORD" => {
                let username = required_env("PVE_USERNAME")?;
                let password = required_env("PVE_PASSWORD")?;
                let otp = optional_env("PVE_OTP");
                let realm = optional_env("PVE_REALM");
                let tfa_challenge = optional_env("PVE_TFA_CHALLENGE");
                Ok(Self::Password {
                    username,
                    password,
                    otp,
                    realm,
                    tfa_challenge,
                })
            }
            other => Err(PveError::InvalidArgument(format!(
                "unsupported PVE_AUTH_METHOD={other}, expected API_TOKEN | API_TOKEN_PARTIAL | USERNAME_PASSWORD"
            ))),
        }
    }
}

pub(crate) fn validate_api_token_format(token: &str) -> Result<(), PveError> {
    let (user_realm, token_part) = token.split_once('!').ok_or_else(|| {
        PveError::InvalidArgument(
            "PVE_API_TOKEN format invalid, expected <user>@<realm>!<tokenid>=<secret>".to_string(),
        )
    })?;
    let (user, realm) = user_realm.split_once('@').ok_or_else(|| {
        PveError::InvalidArgument(
            "PVE_API_TOKEN format invalid, expected <user>@<realm>!<tokenid>=<secret>".to_string(),
        )
    })?;
    let (token_id, secret) = token_part.split_once('=').ok_or_else(|| {
        PveError::InvalidArgument(
            "PVE_API_TOKEN format invalid, expected <user>@<realm>!<tokenid>=<secret>".to_string(),
        )
    })?;

    if user.is_empty() || realm.is_empty() || token_id.is_empty() || secret.is_empty() {
        return Err(PveError::InvalidArgument(
            "PVE_API_TOKEN format invalid, expected <user>@<realm>!<tokenid>=<secret>".to_string(),
        ));
    }

    Ok(())
}

fn required_env(name: &str) -> Result<String, PveError> {
    let value =
        env::var(name).map_err(|_| PveError::InvalidArgument(format!("missing env var {name}")))?;
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(PveError::InvalidArgument(format!(
            "env var {name} must not be empty"
        )));
    }
    Ok(value)
}

fn optional_env(name: &str) -> Option<String> {
    env::var(name).ok().and_then(|v| {
        let v = v.trim().to_string();
        if v.is_empty() { None } else { Some(v) }
    })
}

#[derive(Debug, Clone)]
pub struct ClientOption {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) https: bool,
    pub(crate) insecure_tls: bool,
    pub(crate) timeout: Option<Duration>,
    pub(crate) connect_timeout: Option<Duration>,
    pub(crate) auth: ClientAuth,
}

impl ClientOption {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port: PveClient::DEFAULT_PORT,
            https: true,
            insecure_tls: true,
            timeout: None,
            connect_timeout: None,
            auth: ClientAuth::None,
        }
    }

    pub fn all(
        host: impl Into<String>,
        port: u16,
        https: bool,
        insecure_tls: bool,
        auth: ClientAuth,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            https,
            insecure_tls,
            timeout: None,
            connect_timeout: None,
            auth,
        }
    }

    pub fn all_with_timeouts(
        host: impl Into<String>,
        port: u16,
        https: bool,
        insecure_tls: bool,
        timeout: Option<Duration>,
        connect_timeout: Option<Duration>,
        auth: ClientAuth,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            https,
            insecure_tls,
            timeout,
            connect_timeout,
            auth,
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn https(mut self, https: bool) -> Self {
        self.https = https;
        self
    }

    pub fn insecure_tls(mut self, insecure_tls: bool) -> Self {
        self.insecure_tls = insecure_tls;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = Some(connect_timeout);
        self
    }

    pub fn auth(mut self, auth: ClientAuth) -> Self {
        self.auth = auth;
        self
    }

    pub fn auth_none(mut self) -> Self {
        self.auth = ClientAuth::None;
        self
    }

    pub fn api_token(mut self, token: impl Into<String>) -> Self {
        self.auth = ClientAuth::ApiToken(token.into());
        self
    }

    pub fn api_token_partial(
        mut self,
        user: impl Into<String>,
        realm: impl Into<String>,
        tokenid: impl Into<String>,
        secret: impl Into<String>,
    ) -> Self {
        self.auth = ClientAuth::ApiTokenPartial {
            user: user.into(),
            realm: realm.into(),
            token_id: tokenid.into(),
            secret: secret.into(),
        };
        self
    }

    pub fn ticket(mut self, ticket: impl Into<String>, csrf: impl Into<String>) -> Self {
        self.auth = ClientAuth::Ticket {
            ticket: ticket.into(),
            csrf: Some(csrf.into()),
        };
        self
    }

    pub fn ticket_without_csrf(mut self, ticket: impl Into<String>) -> Self {
        self.auth = ClientAuth::Ticket {
            ticket: ticket.into(),
            csrf: None,
        };
        self
    }

    pub fn password(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.auth = ClientAuth::Password {
            username: username.into(),
            password: password.into(),
            otp: None,
            realm: None,
            tfa_challenge: None,
        };
        self
    }

    pub fn password_with_opts(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
        otp: Option<String>,
        realm: Option<String>,
        tfa_challenge: Option<String>,
    ) -> Self {
        self.auth = ClientAuth::Password {
            username: username.into(),
            password: password.into(),
            otp,
            realm,
            tfa_challenge,
        };
        self
    }

    pub async fn build(self) -> Result<PveClient, PveError> {
        PveClient::from_option(self).await
    }

    pub async fn build_and_connect(self) -> Result<PveClient, PveError> {
        let client = self.build().await?;
        client.connect().await?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{LazyLock, Mutex};

    use super::ClientAuth;

    static ENV_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    const KEYS: &[&str] = &[
        "PVE_AUTH_METHOD",
        "PVE_API_TOKEN",
        "PVE_API_TOKEN_USER",
        "PVE_API_TOKEN_REALM",
        "PVE_API_TOKEN_ID",
        "PVE_API_TOKEN_SECRET",
        "PVE_USERNAME",
        "PVE_PASSWORD",
        "PVE_OTP",
        "PVE_REALM",
        "PVE_TFA_CHALLENGE",
    ];

    fn clear_vars() {
        for key in KEYS {
            // SAFETY: tests serialize env mutations with a global mutex in this module.
            unsafe { std::env::remove_var(key) };
        }
    }

    #[test]
    fn from_env_parses_api_token() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        clear_vars();
        // SAFETY: guarded by ENV_LOCK
        unsafe {
            std::env::set_var("PVE_AUTH_METHOD", "API_TOKEN");
            std::env::set_var("PVE_API_TOKEN", "root@pam!ci=secret");
        }

        let auth = ClientAuth::from_env().expect("must parse");
        match auth {
            ClientAuth::ApiToken(token) => assert_eq!(token, "root@pam!ci=secret"),
            _ => panic!("expected api token"),
        }
    }

    #[test]
    fn from_env_parses_api_token_partial() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        clear_vars();
        // SAFETY: guarded by ENV_LOCK
        unsafe {
            std::env::set_var("PVE_AUTH_METHOD", "API_TOKEN_PARTIAL");
            std::env::set_var("PVE_API_TOKEN_USER", "root");
            std::env::set_var("PVE_API_TOKEN_REALM", "pam");
            std::env::set_var("PVE_API_TOKEN_ID", "ci");
            std::env::set_var("PVE_API_TOKEN_SECRET", "secret");
        }

        let auth = ClientAuth::from_env().expect("must parse");
        match auth {
            ClientAuth::ApiTokenPartial {
                user,
                realm,
                token_id,
                secret,
            } => {
                assert_eq!(user, "root");
                assert_eq!(realm, "pam");
                assert_eq!(token_id, "ci");
                assert_eq!(secret, "secret");
            }
            _ => panic!("expected api token partial"),
        }
    }

    #[test]
    fn from_env_parses_username_password_with_optional_fields() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        clear_vars();
        // SAFETY: guarded by ENV_LOCK
        unsafe {
            std::env::set_var("PVE_AUTH_METHOD", "USERNAME_PASSWORD");
            std::env::set_var("PVE_USERNAME", "root@pam");
            std::env::set_var("PVE_PASSWORD", "secret");
            std::env::set_var("PVE_OTP", "123456");
            std::env::set_var("PVE_REALM", "pam");
            std::env::set_var("PVE_TFA_CHALLENGE", "challenge");
        }

        let auth = ClientAuth::from_env().expect("must parse");
        match auth {
            ClientAuth::Password {
                username,
                password,
                otp,
                realm,
                tfa_challenge,
            } => {
                assert_eq!(username, "root@pam");
                assert_eq!(password, "secret");
                assert_eq!(otp.as_deref(), Some("123456"));
                assert_eq!(realm.as_deref(), Some("pam"));
                assert_eq!(tfa_challenge.as_deref(), Some("challenge"));
            }
            _ => panic!("expected username/password"),
        }
    }

    #[test]
    fn from_env_fails_for_invalid_method() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        clear_vars();
        // SAFETY: guarded by ENV_LOCK
        unsafe { std::env::set_var("PVE_AUTH_METHOD", "INVALID") };
        let err = ClientAuth::from_env().expect_err("must fail");
        assert!(err.to_string().contains("unsupported PVE_AUTH_METHOD"));
    }

    #[test]
    fn from_env_fails_for_missing_required_var() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        clear_vars();
        // SAFETY: guarded by ENV_LOCK
        unsafe { std::env::set_var("PVE_AUTH_METHOD", "API_TOKEN") };
        let err = ClientAuth::from_env().expect_err("must fail");
        assert!(err.to_string().contains("missing env var PVE_API_TOKEN"));
    }

    #[test]
    fn from_env_fails_for_bad_api_token_format() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        clear_vars();
        // SAFETY: guarded by ENV_LOCK
        unsafe {
            std::env::set_var("PVE_AUTH_METHOD", "API_TOKEN");
            std::env::set_var("PVE_API_TOKEN", "just-secret");
        }
        let err = ClientAuth::from_env().expect_err("must fail");
        assert!(err.to_string().contains("PVE_API_TOKEN format invalid"));
    }

    #[test]
    fn from_env_fails_for_api_token_with_empty_secret() {
        let _guard = ENV_LOCK.lock().expect("env lock");
        clear_vars();
        // SAFETY: guarded by ENV_LOCK
        unsafe {
            std::env::set_var("PVE_AUTH_METHOD", "API_TOKEN");
            std::env::set_var("PVE_API_TOKEN", "root@pam!ci=");
        }
        let err = ClientAuth::from_env().expect_err("must fail");
        assert!(err.to_string().contains("PVE_API_TOKEN format invalid"));
    }
}
