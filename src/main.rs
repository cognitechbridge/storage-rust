mod encryptor;


use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use std::fs::File;
use std::io::Write;
use encryptor::AsEncryptedIterator;



use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client as S3Client;
use aws_smithy_types::byte_stream::ByteStream;
use aws_config::BehaviorVersion;

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    // let file = File::create("D:\\File2.rtf").unwrap();
    // let mut writer = BufWriter::new(file);
    //
    // encryptor::process_encrypted_data(
    //     File::open("D:\\File.rtf").unwrap().to_encrypted_iterator(key, nonce, 100 * 1024),
    //     &mut writer, nonce,
    //     key);

    // let mut file = File::create("D:\\Sample.txt").expect("Could not create sample file.");
    // // Loop until the file is 5 chunks.
    // while file.metadata().unwrap().len() <= CHUNK_SIZE * 4 {
    //     let rand_string = "CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB ";
    //     let return_string: String = "\n".to_string();
    //     file.write_all(rand_string.as_ref())
    //         .expect("Error writing to file.");
    //     file.write_all(return_string.as_ref())
    //         .expect("Error writing to file.");
    // }

    // upload(File::open("D:\\Sample.txt").unwrap().to_encrypted_iterator(key, nonce, CHUNK_SIZE as usize),
    //        "Hi2.txt".to_string()
    // ).await;



    // println!("{:x?}", &buffer);
}

async fn upload<I>(iterator: I, key: String) where I: Iterator<Item=encryptor::Result<Vec<u8>>>, {
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
