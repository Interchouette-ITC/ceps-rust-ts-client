//! CEP MCP tool implementations.

pub mod cep18;
pub mod cep78;
pub mod cep85;
pub mod cep95;
pub mod meta;
pub mod params;
pub mod wasm;

/// Stable tool name inventory for `ceps_help` / `ceps_list_tools`.
pub fn registered_tool_names() -> Vec<&'static str> {
    let mut names = Vec::new();
    names.extend(meta::TOOL_NAMES);
    names.extend(wasm::TOOL_NAMES);
    names.extend(cep18::TOOL_NAMES);
    names.extend(cep78::TOOL_NAMES);
    names.extend(cep85::TOOL_NAMES);
    names.extend(cep95::TOOL_NAMES);
    names
}

pub fn tool_groups() -> &'static [(&'static str, &'static str)] {
    &[
        ("meta", "help, endpoints, tool list"),
        ("wasm", "demo contract WASM list/read helpers"),
        ("cep18", "CEP-18 fungible install/query/mutate"),
        ("cep78", "CEP-78 NFT install/query/mutate (+ sessions)"),
        ("cep85", "CEP-85 multi-token install/query/mutate"),
        ("cep95", "CEP-95 NFT install/query/mutate (Odra tip)"),
    ]
}
