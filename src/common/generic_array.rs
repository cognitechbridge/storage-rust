use anyhow::{Result, bail};
use generic_array::{ArrayLength, GenericArray};
use uuid::Uuid;


pub trait GenericArrayFrom<N, T> where N: ArrayLength<T> {
    fn try_from_vec(vec: Vec<T>) -> Result<Self <>> where Self: Sized;
}

impl<N, T> GenericArrayFrom<N, T> for GenericArray<T, N> where N: ArrayLength<T>, T: Default {
    fn try_from_vec(vec: Vec<T>) -> Result<Self> {
        if vec.len() != N::to_usize() {
            bail!("Decoded size doesn't match expected size.")
        }
        let mut arr: GenericArray<T, N> = Default::default();
        arr.iter_mut().zip(vec).for_each(|(place, element)| *place = element);
        Ok(arr)
    }
}


pub trait ToGenericArray<N: ArrayLength<u8>> {
    fn to_generic_array(self) -> GenericArray<u8, N>;
}

impl<N: ArrayLength<u8>> ToGenericArray<N> for Uuid {
    fn to_generic_array(self) -> GenericArray<u8, N> {
        let mut arr: GenericArray<u8, N> = Default::default();
        assert!(N::to_usize() >= 16, "N must be at least 16 to hold a UUID");
        arr[..16].copy_from_slice(self.as_bytes());
        arr
    }
}


impl<N: ArrayLength<u8>> ToGenericArray<N> for GenericArray<u8, N> {
    fn to_generic_array(self) -> GenericArray<u8, N> {
        self
    }
}