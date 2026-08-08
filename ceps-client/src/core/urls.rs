//! URL normalization for RPC and SSE endpoints.

use crate::error::{CepError, Result};
use url::Url;

/// Normalize an RPC URL: require http(s), append `/rpc` when missing.
pub fn normalize_rpc_url(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CepError::InvalidUrl("RPC URL is empty".into()));
    }
    let parsed = Url::parse(trimmed)
        .map_err(|e| CepError::InvalidUrl(format!("RPC URL parse error: {e}")))?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(CepError::InvalidUrl(
            "RPC URL must use http or https".into(),
        ));
    }
    let mut out = trimmed.trim_end_matches('/').to_string();
    if !out.ends_with("/rpc") {
        out.push_str("/rpc");
    }
    Ok(out)
}

/// Normalize an SSE URL: require http(s), append `/events` when missing.
pub fn normalize_sse_url(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CepError::InvalidUrl("SSE URL is empty".into()));
    }
    let parsed = Url::parse(trimmed)
        .map_err(|e| CepError::InvalidUrl(format!("SSE URL parse error: {e}")))?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err(CepError::InvalidUrl(
            "SSE URL must use http or https".into(),
        ));
    }
    let mut out = trimmed.trim_end_matches('/').to_string();
    if !out.ends_with("/events") {
        out.push_str("/events");
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpc_appends_suffix() {
        assert_eq!(
            normalize_rpc_url("http://127.0.0.1:11101").unwrap(),
            "http://127.0.0.1:11101/rpc"
        );
        assert_eq!(
            normalize_rpc_url("http://127.0.0.1:11101/rpc").unwrap(),
            "http://127.0.0.1:11101/rpc"
        );
    }

    #[test]
    fn sse_appends_suffix() {
        assert_eq!(
            normalize_sse_url("http://127.0.0.1:18101").unwrap(),
            "http://127.0.0.1:18101/events"
        );
    }
}
