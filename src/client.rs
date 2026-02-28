use std::time::Duration;

use reqwest::{Method, RequestBuilder, multipart};
use url::Url;

use crate::client_option::{ClientAuth, ClientOption, validate_api_token_format};
pub use crate::core::auth::Auth;
use crate::core::auth::apply_auth;
use crate::core::transport::{
    build_base_url, build_http_client, execute as transport_execute, join_api_url,
};
use crate::error::PveError;
use crate::models::{TicketInfo, VersionInfo};
use crate::params::PveParams;
use crate::requests;

#[derive(Debug, Clone)]
pub struct PveClient {
    base_url: Url,
    http: reqwest::Client,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    auth: Auth,
}

impl PveClient {
    pub const DEFAULT_PORT: u16 = 8006;

    pub async fn from_option(option: ClientOption) -> Result<Self, PveError> {
        let parsed = build_base_url(&option.host, option.port, option.https)?;
        let http = build_http_client(option.insecure_tls, option.timeout, option.connect_timeout)?;
        let mut client = Self {
            base_url: parsed,
            http,
            timeout: option.timeout,
            connect_timeout: option.connect_timeout,
            auth: Auth::None,
        };

        match option.auth {
            ClientAuth::None => {}
            ClientAuth::ApiToken(token) => {
                validate_api_token_format(&token)?;
                client.auth = Auth::ApiToken(token);
            }
            ClientAuth::ApiTokenPartial {
                user,
                realm,
                token_id: tokenid,
                secret,
            } => {
                let user = user.trim();
                let realm = realm.trim();
                let tokenid = tokenid.trim();
                let secret = secret.trim();
                if user.is_empty() || realm.is_empty() || tokenid.is_empty() || secret.is_empty() {
                    return Err(PveError::InvalidArgument(
                        "api token partial fields must be non-empty".to_string(),
                    ));
                }
                let token = format!("{user}@{realm}!{tokenid}={secret}");
                validate_api_token_format(&token)?;
                client.auth = Auth::ApiToken(token);
            }
            ClientAuth::Ticket { ticket, csrf } => {
                client.auth = Auth::Ticket { ticket, csrf };
            }
            ClientAuth::Password {
                username,
                password,
                otp,
                realm,
                tfa_challenge,
            } => {
                let ticket = client
                    .request_ticket(
                        &username,
                        &password,
                        otp.as_deref(),
                        realm.as_deref(),
                        tfa_challenge.as_deref(),
                    )
                    .await?;
                client.auth = Auth::Ticket {
                    ticket: ticket.ticket,
                    csrf: Some(ticket.csrf_prevention_token),
                };
            }
        }

        Ok(client)
    }

    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    pub fn set_auth(&mut self, auth: Auth) {
        self.auth = auth;
    }

    pub fn set_tls_insecure(self, insecure: bool) -> Result<Self, PveError> {
        let http = build_http_client(insecure, self.timeout, self.connect_timeout)?;
        Ok(Self { http, ..self })
    }

    pub async fn connect(&self) -> Result<(), PveError> {
        let _: VersionInfo = self.connect_with_version().await?;
        Ok(())
    }

    pub async fn connect_with_version(&self) -> Result<VersionInfo, PveError> {
        self.version().await
    }

    pub async fn request_ticket(
        &self,
        username: &str,
        password: &str,
        otp: Option<&str>,
        realm: Option<&str>,
        tfa_challenge: Option<&str>,
    ) -> Result<TicketInfo, PveError> {
        let mut params = PveParams::new();
        params.insert("username", username);
        params.insert("password", password);
        params.insert_opt("otp", otp);
        params.insert_opt("realm", realm);
        params.insert_opt("tfa-challenge", tfa_challenge);

        self.send(Method::POST, "/access/ticket", None, Some(&params))
            .await
    }

    pub async fn request_ticket_with(
        &self,
        request: &requests::TicketRequest,
    ) -> Result<TicketInfo, PveError> {
        let params = request.to_params();
        self.send(Method::POST, "/access/ticket", None, Some(&params))
            .await
    }

    pub(crate) async fn send<T>(
        &self,
        method: Method,
        path: &str,
        query: Option<&PveParams>,
        form: Option<&PveParams>,
    ) -> Result<T, PveError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.url(path)?;
        let mut request = self.http.request(method.clone(), url);

        if let Some(query) = query
            && !query.is_empty()
        {
            request = request.query(&query.0);
        }

        request = self.apply_auth(request, &method)?;

        if let Some(form) = form {
            request = request.form(&form.0);
        }

        self.execute(request).await
    }

    pub(crate) async fn send_multipart<T>(
        &self,
        method: Method,
        path: &str,
        form: multipart::Form,
    ) -> Result<T, PveError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.url(path)?;
        let request = self.apply_auth(self.http.request(method.clone(), url), &method)?;
        let request = request.multipart(form);
        self.execute(request).await
    }

    async fn execute<T>(&self, request: RequestBuilder) -> Result<T, PveError>
    where
        T: serde::de::DeserializeOwned,
    {
        transport_execute(request).await
    }

    fn apply_auth(
        &self,
        request: RequestBuilder,
        method: &Method,
    ) -> Result<RequestBuilder, PveError> {
        apply_auth(&self.auth, request, method)
    }

    fn url(&self, path: &str) -> Result<Url, PveError> {
        join_api_url(&self.base_url, path)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use reqwest::Method;
    use reqwest::header::{AUTHORIZATION, COOKIE};
    use url::Url;

    use super::{Auth, PveClient};
    use crate::client_option::{ClientAuth, ClientOption};
    use crate::core::transport::{build_base_url, normalize_api_path};
    use crate::error::PveError;
    use crate::params::PveParams;
    use crate::requests;

    fn client_with_auth(auth: Auth) -> PveClient {
        PveClient {
            base_url: Url::parse("https://pve.example.com:8006/").expect("base url"),
            http: reqwest::Client::new(),
            timeout: None,
            connect_timeout: None,
            auth,
        }
    }

    #[test]
    fn normalize_path_adds_api_prefix() {
        assert_eq!(normalize_api_path("/nodes"), "/api2/json/nodes");
        assert_eq!(normalize_api_path("nodes"), "/api2/json/nodes");
        assert_eq!(
            normalize_api_path("/api2/json/version"),
            "/api2/json/version"
        );
    }

    #[test]
    fn build_base_url_from_host_and_port() {
        let url = build_base_url("pve.example.com", 8006, true).expect("must parse");
        assert_eq!(url.as_str(), "https://pve.example.com:8006/");
    }

    #[test]
    fn build_base_url_rejects_embedded_port() {
        let err = build_base_url("pve.example.com:8006", 8006, true).expect_err("must fail");
        assert!(err.to_string().contains("use ClientOption::port()"));
    }

    #[test]
    fn build_base_url_rejects_embedded_path() {
        let err = build_base_url("pve.example.com/api2/json", 8006, true).expect_err("must fail");
        assert!(err.to_string().contains("must not include path"));
    }

    #[test]
    fn build_base_url_supports_ipv6_without_brackets() {
        let url = build_base_url("2001:db8::1", 8006, true).expect("must parse");
        assert_eq!(url.as_str(), "https://[2001:db8::1]:8006/");
    }

    #[tokio::test]
    async fn client_option_chain_builds_client() {
        let client = ClientOption::new("pve.example.com")
            .port(8443)
            .https(false)
            .insecure_tls(false)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(3))
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        assert_eq!(
            client.url("/version").expect("url").as_str(),
            "http://pve.example.com:8443/api2/json/version"
        );
        assert!(matches!(client.auth(), Auth::ApiToken(_)));
    }

    #[tokio::test]
    async fn client_option_all_builds_client() {
        let options = ClientOption::all("pve.example.com", 9443, true, true, ClientAuth::None);
        let client = PveClient::from_option(options).await.expect("must build");
        assert_eq!(
            client.url("/nodes").expect("url").as_str(),
            "https://pve.example.com:9443/api2/json/nodes"
        );
    }

    #[tokio::test]
    async fn client_option_all_with_timeouts_builds_client() {
        let options = ClientOption::all_with_timeouts(
            "pve.example.com",
            9443,
            true,
            true,
            Some(Duration::from_secs(10)),
            Some(Duration::from_secs(3)),
            ClientAuth::None,
        );
        let client = PveClient::from_option(options).await.expect("must build");
        assert_eq!(
            client.url("/nodes").expect("url").as_str(),
            "https://pve.example.com:9443/api2/json/nodes"
        );
    }

    #[tokio::test]
    async fn client_option_api_token_partial_builds_token() {
        let client = ClientOption::new("pve.example.com")
            .api_token_partial("root", "pam", "ci", "secret")
            .build()
            .await
            .expect("must build");
        match client.auth() {
            Auth::ApiToken(token) => assert_eq!(token, "root@pam!ci=secret"),
            _ => panic!("expected api token auth"),
        }
    }

    #[tokio::test]
    async fn client_option_api_token_rejects_invalid_format() {
        let err = ClientOption::new("pve.example.com")
            .api_token("invalid-token")
            .build()
            .await
            .expect_err("must fail");
        assert!(err.to_string().contains("PVE_API_TOKEN format invalid"));
    }

    #[test]
    fn apply_auth_sets_api_token_header() {
        let client = client_with_auth(Auth::ApiToken("root@pam!ci=secret".to_string()));
        let request = client.http.request(
            Method::GET,
            "https://pve.example.com:8006/api2/json/version",
        );
        let request = client
            .apply_auth(request, &Method::GET)
            .expect("must apply auth")
            .build()
            .expect("request");
        let header = request
            .headers()
            .get(AUTHORIZATION)
            .expect("authorization header")
            .to_str()
            .expect("utf8");
        assert_eq!(header, "PVEAPIToken=root@pam!ci=secret");
    }

    #[test]
    fn apply_auth_ticket_get_does_not_require_csrf() {
        let client = client_with_auth(Auth::Ticket {
            ticket: "PVE:ticket-value".to_string(),
            csrf: None,
        });
        let request = client.http.request(
            Method::GET,
            "https://pve.example.com:8006/api2/json/version",
        );
        let request = client
            .apply_auth(request, &Method::GET)
            .expect("must apply auth")
            .build()
            .expect("request");
        assert!(request.headers().get("CSRFPreventionToken").is_none());
        let cookie = request
            .headers()
            .get(COOKIE)
            .expect("cookie header")
            .to_str()
            .expect("utf8");
        assert_eq!(cookie, "PVEAuthCookie=PVE:ticket-value");
    }

    #[test]
    fn apply_auth_ticket_write_requires_csrf() {
        let client = client_with_auth(Auth::Ticket {
            ticket: "PVE:ticket-value".to_string(),
            csrf: None,
        });
        let request = client
            .http
            .request(Method::POST, "https://pve.example.com:8006/api2/json/nodes");
        let err = client
            .apply_auth(request, &Method::POST)
            .expect_err("must reject missing csrf");
        assert!(matches!(err, PveError::MissingCsrfToken));
    }

    #[test]
    fn apply_auth_ticket_write_sets_csrf_header() {
        let client = client_with_auth(Auth::Ticket {
            ticket: "PVE:ticket-value".to_string(),
            csrf: Some("csrf-token-value".to_string()),
        });
        let request = client
            .http
            .request(Method::POST, "https://pve.example.com:8006/api2/json/nodes");
        let request = client
            .apply_auth(request, &Method::POST)
            .expect("must apply auth")
            .build()
            .expect("request");
        let csrf = request
            .headers()
            .get("CSRFPreventionToken")
            .expect("csrf header")
            .to_str()
            .expect("utf8");
        assert_eq!(csrf, "csrf-token-value");
    }

    #[test]
    fn pve_params_handles_bool() {
        let mut params = PveParams::new();
        params.insert_bool("full", true);
        params.insert_bool("onboot", false);

        assert_eq!(params.get("full"), Some("1"));
        assert_eq!(params.get("onboot"), Some("0"));
    }

    #[tokio::test]
    async fn access_set_acl_rejects_missing_subject() {
        let client = ClientOption::new("pve.example.com")
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        let request = requests::AccessSetAclRequest::new("/vms", "PVEVMAdmin");
        let err = client
            .access_set_acl_with(&request)
            .await
            .expect_err("must reject missing subject");
        assert!(matches!(err, PveError::InvalidArgument(_)));
    }

    #[tokio::test]
    async fn access_delete_acl_rejects_missing_targets() {
        let client = ClientOption::new("pve.example.com")
            .api_token("root@pam!ci=token")
            .build()
            .await
            .expect("must build");

        let request = requests::AccessDeleteAclRequest::new("/vms");
        let err = client
            .access_delete_acl_with(&request)
            .await
            .expect_err("must reject missing target");
        assert!(matches!(err, PveError::InvalidArgument(_)));
    }
}
