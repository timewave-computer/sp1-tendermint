use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TendermintOutput {
    pub trusted_height: u64,
    pub target_height: u64,
    pub trusted_header_hash: [u8; 32],
    pub target_header_hash: [u8; 32],
}
