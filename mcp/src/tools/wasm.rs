//! Demo contract WASM helpers.

use crate::format;
use crate::handle;
use base64::Engine;
use mcpkit::prelude::ToolOutput;
use std::path::{Component, Path};

pub const TOOL_NAMES: &[&str] = &[
    "ceps_list_contract_wasms",
    "ceps_read_contract_wasm",
    "ceps_canonical_wasm_paths",
];

pub fn list_contract_wasms() -> ToolOutput {
    let root = handle::wasm_root();
    if !root.is_dir() {
        return format::err(format!(
            "wasm root missing: {} (set CEPS_WASM_ROOT or run make wasm-from-ceps)",
            root.display()
        ));
    }
    let mut files = Vec::new();
    for cep in ["cep18", "cep78", "cep85"] {
        let dir = root.join(cep);
        if !dir.is_dir() {
            continue;
        }
        let Ok(rd) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("wasm") {
                continue;
            }
            let rel = format!("{cep}/{}", entry.file_name().to_string_lossy());
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            files.push(serde_json::json!({ "path": rel, "bytes": size }));
        }
    }
    files.sort_by(|a, b| {
        a["path"]
            .as_str()
            .unwrap_or("")
            .cmp(b["path"].as_str().unwrap_or(""))
    });
    format::json_ok(&serde_json::json!({
        "wasm_root": root.display().to_string(),
        "files": files,
    }))
}

pub fn read_contract_wasm(path: String) -> ToolOutput {
    let root = handle::wasm_root();
    let rel = Path::new(&path);
    if rel.is_absolute() || rel.components().any(|c| matches!(c, Component::ParentDir)) {
        return format::err("path must be relative under CEPS_WASM_ROOT without '..'");
    }
    let full = root.join(rel);
    let bytes = match std::fs::read(&full) {
        Ok(b) => b,
        Err(e) => return format::err(format!("read {}: {e}", full.display())),
    };
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    format::json_ok(&serde_json::json!({
        "path": path,
        "bytes": bytes.len(),
        "base64": b64,
    }))
}

pub fn canonical_wasm_paths() -> ToolOutput {
    format::json_ok(&serde_json::json!({
        "cep18": {
            "install": "cep18/cep18.wasm",
            "test_contract": "cep18/cep18_test_contract.wasm",
        },
        "cep78": {
            "install": "cep78/cep78.wasm",
            "mint_session": "cep78/mint_session.wasm",
            "transfer_session": "cep78/transfer_session.wasm",
            "updated_receipts": "cep78/updated_receipts.wasm",
            "balance_of_session": "cep78/balance_of_session.wasm",
            "owner_of_session": "cep78/owner_of_session.wasm",
            "get_approved_session": "cep78/get_approved_session.wasm",
            "is_approved_for_all_session": "cep78/is_approved_for_all_session.wasm",
        },
        "cep85": {
            "install": "cep85/cep85.wasm",
            "test_contract": "cep85/cep85_test_contract.wasm",
        },
        "note": "Paths are relative to CEPS_WASM_ROOT (default tests/wasm).",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_json_ok() {
        let _ = format!("{:?}", canonical_wasm_paths());
    }
}
