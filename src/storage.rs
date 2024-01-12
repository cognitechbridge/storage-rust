use std::io::{Read, Write};
pub mod s3;

pub trait StorageUpload {
    async fn upload<R: Read>(& self, reader: &mut R, key: String) -> anyhow::Result<()>;
}

pub trait StorageDownload {
    async fn download<W: Write>(& self, writer: &mut W, key: String) -> anyhow::Result<usize>;
}