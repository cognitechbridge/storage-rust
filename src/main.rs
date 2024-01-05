mod encryptor;


use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use std::fs::File;
use std::io::{BufWriter};
use encryptor::AsEncryptedIterator;



use std::io::prelude::*;
use std::path::Path;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client as S3Client;
use aws_smithy_types::byte_stream::{ByteStream, Length};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::process;
use aws_config::BehaviorVersion;
use uuid::Uuid;

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

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

    upload().await;

    // println!("{:x?}", &buffer);
}

async fn upload() {
    //let shared_config = aws_config::load_from_env().await;


    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;


    println!("{}",config.region().unwrap());
    let client = S3Client::new(&config);
    let bucket_name = "ctb-test-2";

    let key = "sample.txt".to_string();
    // snippet-start:[rust.example_code.s3.create_multipart_upload]
    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(bucket_name)
        .key(&key)
        .send()
        .await
        .unwrap();
    // snippet-end:[rust.example_code.s3.create_multipart_upload]
    let upload_id = multipart_upload_res.upload_id().unwrap();

    //Create a file of random characters for the upload.
    let mut file = File::create("D:\\1.txt").expect("Could not create sample file.");
    // Loop until the file is 5 chunks.
    while file.metadata().unwrap().len() <= CHUNK_SIZE * 4 {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(256)
            .map(char::from)
            .collect();
        let return_string: String = "\n".to_string();
        file.write_all(rand_string.as_ref())
            .expect("Error writing to file.");
        file.write_all(return_string.as_ref())
            .expect("Error writing to file.");
    }

    let path = Path::new("D:\\1.txt");
    let file_size = tokio::fs::metadata(path)
        .await
        .expect("it exists I swear")
        .len();

    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = CHUNK_SIZE;
        chunk_count -= 1;
    }

    if file_size == 0 {
        panic!("Bad file size.");
    }
    if chunk_count > MAX_CHUNKS {
        panic!("Too many chunks! Try increasing your chunk size.")
    }

    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            CHUNK_SIZE
        };
        let stream = ByteStream::read_from()
            .path(path)
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();
        //Chunk index needs to start at 0, but part numbers start at 1.
        let part_number = (chunk_index as i32) + 1;
        // snippet-start:[rust.example_code.s3.upload_part]
        let upload_part_res = client
            .upload_part()
            .key(&key)
            .bucket(bucket_name)
            .upload_id(upload_id)
            .body(stream)
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
        // snippet-end:[rust.example_code.s3.upload_part]
    }
    // snippet-start:[rust.example_code.s3.upload_part.CompletedMultipartUpload]
    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();
    // snippet-end:[rust.example_code.s3.upload_part.CompletedMultipartUpload]

    // snippet-start:[rust.example_code.s3.complete_multipart_upload]
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


