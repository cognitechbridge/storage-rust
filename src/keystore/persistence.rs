use anyhow::Result;


pub trait SerializedPersistKeyStore {
    fn save_key(&self, key_id: &str, nonce: &str, key: &str, tag: &str) -> Result<()>;
    fn get_key(&self, key_id: &str) -> Result<Option<(String, String)>>;
    fn get_with_tag(&self, tag: &str) -> Result<Option<(String, String, String)>>;
}


