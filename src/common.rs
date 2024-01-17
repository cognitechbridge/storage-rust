use aead::Aead;
use crypto_common::{KeyInit, KeySizeUser};

pub trait Crypto: KeySizeUser + KeyInit + Aead {}

impl<T> Crypto for T where T: KeySizeUser + KeyInit + Aead {}