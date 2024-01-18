use anyhow::{Result, bail};
use generic_array::{ArrayLength, GenericArray};


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