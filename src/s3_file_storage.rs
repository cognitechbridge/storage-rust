use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::{Client as S3Client};
use aws_smithy_types::byte_stream::ByteStream;
use aws_config::BehaviorVersion;
use std::io::{Read, Write};

const CHUNK_SIZE: usize = 10 * 1024 * 1024;
const BUCKET_NAME: &str = "ctb-test-2";


pub async fn client() -> S3Client {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = S3Client::new(&config);
    return client;
}

pub async fn upload<R>(reader: &mut R, key: String)
    where
        R: Read,
{
    let client = client().await;
    let bucket_name = BUCKET_NAME;

    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(bucket_name)
        .key(&key)
        .send()
        .await
        .unwrap();
    let upload_id = multipart_upload_res.upload_id().unwrap();


    let mut upload_parts: Vec<CompletedPart> = Vec::new();
    let mut chunk_index = 1;
    let mut buffer = vec![0; CHUNK_SIZE];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let byte_stream = ByteStream::from(buffer[..bytes_read].to_vec());
        let upload_part_res = client
            .upload_part()
            .key(&key)
            .bucket(bucket_name)
            .upload_id(upload_id)
            .body(byte_stream)
            .part_number(chunk_index)
            .send()
            .await
            .unwrap();

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
        .bucket(bucket_name)
        .key(&key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
}

pub async fn download<W>(writer: &mut W, key: String) -> Result<usize, anyhow::Error>
    where
        W: Write,
{
    let client = client().await;

    let mut object = client
        .get_object()
        .bucket(BUCKET_NAME)
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