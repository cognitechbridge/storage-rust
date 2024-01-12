use std::io::{Write};
use crate::storage::StorageDownload;
use super::{S3Storage, client};


impl StorageDownload for S3Storage {
    async fn download<W: Write>(&self, writer: &mut W, key: String) -> anyhow::Result<usize>
    {
        let client = client::get_s3_client().await;

        let mut object = client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;

        let mut byte_count = 0_usize;
        while let Some(bytes) = object.body.try_next().await? {
            let bytes_len = bytes.len();
            writer.write_all(&bytes)?;
            byte_count += bytes_len;
        }

        Ok(byte_count)
    }
}