use std::future::Future;
use std::io::{Read, Write};
use anyhow::Result;
pub mod s3;

pub trait StorageUpload {
    fn upload<R: Read>(& self, reader: &mut R, key: &str) -> impl Future<Output = Result<()>>;
}

pub trait StorageDownload {
    fn download<W: Write>(& self, writer: &mut W, key: &str) -> impl Future<Output = Result<usize>>;
}