//! CEP-78 install argument placeholders (expanded with mode matrix later).

use crate::types::EventsMode78;

/// Required install fields for CEP-78.
#[derive(Debug, Clone)]
pub struct InstallArgs {
    /// Collection name.
    pub collection_name: String,
    /// Collection symbol.
    pub collection_symbol: String,
    /// Total token supply.
    pub total_token_supply: u64,
    /// Ownership mode (`u8`).
    pub ownership_mode: u8,
    /// NFT metadata kind (`u8`).
    pub nft_metadata_kind: u8,
    /// Identifier mode (`u8`).
    pub identifier_mode: u8,
    /// Metadata mutability (`u8`).
    pub metadata_mutability: u8,
    /// Optional events mode.
    pub events_mode: Option<EventsMode78>,
}

impl InstallArgs {
    /// Required fields constructor.
    pub fn new(
        collection_name: impl Into<String>,
        collection_symbol: impl Into<String>,
        total_token_supply: u64,
        ownership_mode: u8,
        nft_metadata_kind: u8,
        identifier_mode: u8,
        metadata_mutability: u8,
    ) -> Self {
        Self {
            collection_name: collection_name.into(),
            collection_symbol: collection_symbol.into(),
            total_token_supply,
            ownership_mode,
            nft_metadata_kind,
            identifier_mode,
            metadata_mutability,
            events_mode: None,
        }
    }
}
