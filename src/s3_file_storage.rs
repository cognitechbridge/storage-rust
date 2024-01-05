use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client as S3Client;
use aws_smithy_types::byte_stream::ByteStream;
use aws_config::BehaviorVersion;
use crate::encryptor;

pub async fn upload<I>(iterator: I, key: String) where I: Iterator<Item=encryptor::Result<Vec<u8>>>, {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = S3Client::new(&config);
    let bucket_name = "ctb-test-2";

    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(bucket_name)
        .key(&key)
        .send()
        .await
        .unwrap();
    let upload_id = multipart_upload_res.upload_id().unwrap();


    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    let mut chunk_index = 0;
    for chunk in iterator {
        let byte_stream = ByteStream::from(chunk.unwrap());
        let part_number = (chunk_index as i32) + 1;

        let upload_part_res = client
            .upload_part()
            .key(&key)
            .bucket(bucket_name)
            .upload_id(upload_id)
            .body(byte_stream)
            .part_number(part_number)
            .send()
            .await
            .unwrap();

        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
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
