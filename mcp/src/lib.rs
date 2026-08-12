//! MCP sidecar library for `ceps-rust-ts-client`.

pub mod format;
pub mod handle;
pub mod server;
pub mod tool_args;
pub mod tools;

#[cfg(test)]
mod live_tests;

pub use server::{run, run_http, DEFAULT_HTTP_LISTEN};

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Package name.
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_semverish() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.contains('.'));
    }

    #[test]
    fn registers_shared_client_and_ces_tools() {
        let names = tools::registered_tool_names();
        for required in [
            "ceps_put_transaction",
            "ceps_wait_transaction",
            "ceps_ces_parse_execution",
            "ceps_ces_parse_transaction",
            "ceps_ces_collect",
        ] {
            assert!(
                names.contains(&required),
                "missing MCP tool {required} in {names:?}"
            );
        }
    }
}
