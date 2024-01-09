use std::io::Read;
use encrypt_file::EncryptedFileGenerator;
use encrypt_iterator::ToEncryptedIterator;
use types::Key;

mod encrypt_file;
mod decrypt_file;
mod encrypt_iterator;
mod utils;
mod core;
mod constants;
mod types;



pub trait ToEncryptedStream<Y> where Y: Read {
    fn to_encrypted_stream(self, key: Key, chunk_size: usize) -> Y;
}

pub trait ToPlainStream<Y> where Y: Read {
    fn to_plain_stream(self, key: Key) -> Y;
}

impl<T: Read> ToEncryptedStream<EncryptedFileGenerator<T>> for T {
    fn to_encrypted_stream(self, key: Key, chunk_size: usize) -> EncryptedFileGenerator<T> {
        return self
            .to_encrypted_iterator(key, chunk_size)
            .to_encrypted_file_generator();
    }
}
