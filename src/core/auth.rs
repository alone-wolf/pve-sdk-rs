use reqwest::header::{AUTHORIZATION, COOKIE, HeaderValue};
use reqwest::{Method, RequestBuilder};

use crate::error::PveError;

#[derive(Debug, Clone)]
pub enum Auth {
    None,
    ApiToken(String),
    Ticket {
        ticket: String,
        csrf: Option<String>,
    },
}

pub(crate) fn apply_auth(
    auth: &Auth,
    request: RequestBuilder,
    method: &Method,
) -> Result<RequestBuilder, PveError> {
    match auth {
        Auth::None => Ok(request),
        Auth::ApiToken(token) => {
            let value = format!("PVEAPIToken={token}");
            Ok(request.header(
                AUTHORIZATION,
                HeaderValue::from_str(&value).map_err(|_| {
                    PveError::InvalidArgument("invalid api token header value".to_string())
                })?,
            ))
        }
        Auth::Ticket { ticket, csrf } => {
            let mut request = request.header(COOKIE, format!("PVEAuthCookie={ticket}"));

            let is_write = matches!(
                *method,
                Method::POST | Method::PUT | Method::DELETE | Method::PATCH
            );

            if is_write {
                let csrf = csrf.as_deref().ok_or(PveError::MissingCsrfToken)?;
                request = request.header("CSRFPreventionToken", csrf);
            }

            Ok(request)
        }
    }
}
