mod storage_s3_download;
mod storage_s3_upload;
mod client;

pub struct S3Storage {
    bucket_name: String,
    chunk_size: usize,
}

impl S3Storage {
    pub fn new(bucket_name: &str, chunk_size: usize) -> Self {
        Self {
            bucket_name: String::from(bucket_name),
            chunk_size,
        }
    }
}