//! Resolve and load demo on-chain CEP contract WASM bytes from disk.
//!
//! Bytes live under [`ENV_WASM_ROOT`] (or discovered defaults such as
//! `tests/wasm/`). Short names like `cep18` resolve to
//! `{root}/cep18/cep18.wasm`. This module does not embed contract bytecode.

use crate::error::{CEPError, Result};
use std::path::{Component, Path, PathBuf};

/// Environment variable for the demo contract WASM root directory.
pub const ENV_WASM_ROOT: &str = "CEPS_WASM_ROOT";

/// Resolve the demo WASM root directory.
///
/// Order: [`ENV_WASM_ROOT`] if set; else the first existing directory among
/// `/opt/ceps/tests/wasm`, `tests/wasm`, `./tests/wasm`; else `tests/wasm`.
#[must_use]
pub fn wasm_root() -> PathBuf {
    if let Ok(p) = std::env::var(ENV_WASM_ROOT) {
        let trimmed = p.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    for candidate in ["/opt/ceps/tests/wasm", "tests/wasm", "./tests/wasm"] {
        let p = PathBuf::from(candidate);
        if p.is_dir() {
            return p;
        }
    }
    PathBuf::from("tests/wasm")
}

/// Resolve `name` to a filesystem path under `root`.
///
/// Accepts:
/// - empty → error
/// - path containing `/` or `\` (absolute, or relative under `root`)
/// - file under `root` (`cep18.wasm` or `cep18/cep18.wasm`)
/// - short alias (`cep18` → `root/cep18/cep18.wasm`, then `root/cep18.wasm`)
///
/// Relative paths must not contain `..` components that escape `root`.
///
/// # Errors
///
/// Returns [`CEPError::InvalidArgument`] for an empty name or a path that
/// escapes `root`. Returns [`CEPError::WasmNotFound`] when no candidate exists.
pub fn resolve_path(root: &Path, name: &str) -> Result<PathBuf> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CEPError::InvalidArgument("wasm name is empty".into()));
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    if name.contains('/') || name.contains('\\') {
        let as_path = PathBuf::from(name);
        if as_path.is_absolute() {
            if path_has_parent(&as_path) {
                return Err(CEPError::InvalidArgument(format!(
                    "wasm path must not contain '..': {name}"
                )));
            }
            candidates.push(as_path);
        } else {
            if path_escapes_root(root, &as_path) {
                return Err(CEPError::InvalidArgument(format!(
                    "wasm path escapes root {}: {name}",
                    root.display()
                )));
            }
            candidates.push(root.join(&as_path));
        }
    } else {
        candidates.push(root.join(name));
        if !name.ends_with(".wasm") {
            candidates.push(root.join(format!("{name}.wasm")));
            candidates.push(root.join(name).join(format!("{name}.wasm")));
        }
    }

    for path in &candidates {
        if path.is_file() {
            return Ok(path.clone());
        }
    }

    Err(CEPError::WasmNotFound {
        name: name.to_string(),
        tried: candidates.iter().map(|p| p.display().to_string()).collect(),
    })
}

/// Resolve `name` under [`wasm_root`].
///
/// # Errors
///
/// Same as [`resolve_path`].
pub fn resolve(name: &str) -> Result<PathBuf> {
    resolve_path(&wasm_root(), name)
}

/// Read WASM bytes for `name` under `root`.
///
/// # Errors
///
/// Returns resolve errors, or [`CEPError::Io`] when the file cannot be read.
pub fn load_from(root: &Path, name: &str) -> Result<Vec<u8>> {
    let path = resolve_path(root, name)?;
    std::fs::read(&path).map_err(CEPError::from)
}

/// Read WASM bytes for `name` under [`wasm_root`].
///
/// # Errors
///
/// Same as [`load_from`].
pub fn load(name: &str) -> Result<Vec<u8>> {
    load_from(&wasm_root(), name)
}

fn path_has_parent(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

fn path_escapes_root(root: &Path, relative: &Path) -> bool {
    if !path_has_parent(relative) {
        return false;
    }
    let joined = root.join(relative);
    let Ok(canon_root) = root.canonicalize() else {
        // Root may not exist yet in unit tests; reject any `..` in the relative path.
        return true;
    };
    match joined.canonicalize() {
        Ok(canon) => !canon.starts_with(&canon_root),
        Err(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn resolve_alias_cep18_dir() {
        let dir = tempfile_dir();
        let nested = dir.join("cep18");
        fs::create_dir_all(&nested).unwrap();
        let wasm = nested.join("cep18.wasm");
        let mut f = fs::File::create(&wasm).unwrap();
        writeln!(f, "fake").unwrap();
        drop(f);

        let path = resolve_path(&dir, "cep18").unwrap();
        assert_eq!(path, wasm);
    }

    #[test]
    fn resolve_relative_under_root() {
        let dir = tempfile_dir();
        let nested = dir.join("cep78");
        fs::create_dir_all(&nested).unwrap();
        let wasm = nested.join("mint_session.wasm");
        fs::write(&wasm, b"x").unwrap();

        let path = resolve_path(&dir, "cep78/mint_session.wasm").unwrap();
        assert_eq!(path, wasm);
    }

    #[test]
    fn resolve_missing_name() {
        let dir = tempfile_dir();
        let err = resolve_path(&dir, "cep18").unwrap_err();
        match err {
            CEPError::WasmNotFound { name, tried } => {
                assert_eq!(name, "cep18");
                assert!(!tried.is_empty());
            }
            other => panic!("expected WasmNotFound, got {other:?}"),
        }
    }

    #[test]
    fn resolve_empty_name() {
        let dir = tempfile_dir();
        let err = resolve_path(&dir, "  ").unwrap_err();
        assert!(matches!(err, CEPError::InvalidArgument(_)));
    }

    #[test]
    fn resolve_parent_escape_rejected() {
        let dir = tempfile_dir();
        let err = resolve_path(&dir, "../outside.wasm").unwrap_err();
        assert!(matches!(err, CEPError::InvalidArgument(_)));
    }

    #[test]
    fn load_from_reads_bytes() {
        let dir = tempfile_dir();
        let nested = dir.join("cep18");
        fs::create_dir_all(&nested).unwrap();
        fs::write(nested.join("cep18.wasm"), b"wasm-bytes").unwrap();
        let bytes = load_from(&dir, "cep18").unwrap();
        assert_eq!(bytes, b"wasm-bytes");
    }

    fn tempfile_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "ceps-wasm-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
