use std::io::Read;
use anyhow::anyhow;
use aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_smithy_types::byte_stream::ByteStream;

use super::{S3Storage, client};
use super::super::StorageUpload;

impl StorageUpload for S3Storage {
    async fn upload<R: Read>(& self, reader: &mut R, key: &str) -> anyhow::Result<()>
        where
            R: Read,
    {
        let client = client::get_s3_client().await;
        let bucket_name = &self.bucket_name;

        let multipart_upload_res: CreateMultipartUploadOutput = client
            .create_multipart_upload()
            .bucket(bucket_name)
            .key(key)
            .send()
            .await?;
        let upload_id = multipart_upload_res.upload_id().ok_or(anyhow!("S3 upload id not returned"))?;


        let mut upload_parts: Vec<CompletedPart> = Vec::new();
        let mut chunk_index = 1;
        let mut buffer = vec![0; self.chunk_size];

        while let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            let byte_stream = ByteStream::from(buffer[..bytes_read].to_vec());
            let upload_part_res = client
                .upload_part()
                .key(key)
                .bucket(&self.bucket_name)
                .upload_id(upload_id)
                .body(byte_stream)
                .part_number(chunk_index)
                .send()
                .await?;

            upload_parts.push(
                CompletedPart::builder()
                    .e_tag(upload_part_res.e_tag.unwrap_or_default())
                    .part_number(chunk_index)
                    .build(),
            );
            chunk_index += 1;
        }

        let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
            .set_parts(Some(upload_parts))
            .build();

        let _complete_multipart_upload_res = client
            .complete_multipart_upload()
            .bucket(&self.bucket_name)
            .key(key)
            .multipart_upload(completed_multipart_upload)
            .upload_id(upload_id)
            .send()
            .await?;
        Ok(())
    }
}