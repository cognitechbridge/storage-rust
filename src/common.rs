mod generic_array;
pub mod utils;


use ::generic_array::GenericArray;
use aead::{Aead, AeadCore};
use crypto_common::{KeyInit, KeySizeUser};

pub use generic_array::{
    GenericArrayFrom,
    ToGenericArray
};

pub trait Crypto: KeySizeUser + KeyInit + Aead {}

impl<T> Crypto for T where T: KeySizeUser + KeyInit + Aead {}


pub type Key<C> = GenericArray<u8, <C as KeySizeUser>::KeySize>;
pub type Nonce<C> = GenericArray<u8, <C as AeadCore>::NonceSize>;

