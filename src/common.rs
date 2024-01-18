use aead::{Aead, AeadCore};
use crypto_common::{KeyInit, KeySizeUser};
use generic_array::GenericArray;

pub trait Crypto: KeySizeUser + KeyInit + Aead {}
impl<T> Crypto for T where T: KeySizeUser + KeyInit + Aead {}


pub type Key<C> = GenericArray<u8, <C as KeySizeUser>::KeySize>;
pub type Nonce<C> = GenericArray<u8, <C as AeadCore>::NonceSize>;

