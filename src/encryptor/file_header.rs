use serde::{Deserialize, Serialize};
use super::constants::*;

#[derive(Serialize, Deserialize)]
pub enum EncryptionFileHeaderVersion {
    V1
}

#[derive(Serialize, Deserialize)]
pub struct EncryptionFileHeader {
    pub version: EncryptionFileHeaderVersion,
    pub client_id: String,
    pub file_id: String,
    pub chunk_size: u64,
    pub recovery: String,
}

impl Default for EncryptionFileHeader {
    fn default() -> EncryptionFileHeader {
        EncryptionFileHeader {
            version: EncryptionFileHeaderVersion::V1,
            chunk_size: DEFAULT_CHUNK_SIZE,
            client_id: "".to_string(),
            file_id: "".to_string(),
            recovery: "".to_string(),
        }
    }
}