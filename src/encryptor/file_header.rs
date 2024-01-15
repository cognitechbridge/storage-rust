use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::encryptor::utils::type_name_of;
use super::constants::*;

#[derive(Serialize, Deserialize)]
pub enum EncryptionFileHeaderVersion {
    V1
}

#[derive(Serialize, Deserialize)]
pub struct EncryptionFileHeader<C> {
    pub version: EncryptionFileHeaderVersion,
    pub alg: String,
    pub client_id: String,
    pub file_id: String,
    pub chunk_size: u64,
    pub recovery: String,
    #[serde(skip)]
    pub phantom: PhantomData<C>,
}

impl<C> Default for EncryptionFileHeader<C> {
    fn default() -> EncryptionFileHeader<C> {
        EncryptionFileHeader {
            version: EncryptionFileHeaderVersion::V1,
            alg: type_name_of::<C>(),
            chunk_size: DEFAULT_CHUNK_SIZE,
            client_id: "".to_string(),
            file_id: "".to_string(),
            recovery: "".to_string(),
            phantom: PhantomData,
        }
    }
}