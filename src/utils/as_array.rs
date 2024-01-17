use generic_array::{ArrayLength, GenericArray};
use uuid::Uuid;

pub trait AsArray<N: ArrayLength<u8>> {
    fn as_array(&self) -> GenericArray<u8, N>;
}


impl<N: ArrayLength<u8>> AsArray<N> for Uuid {
    fn as_array(&self) -> GenericArray<u8, N> {
        let mut arr: GenericArray<u8, N> = Default::default();
        assert!(N::to_usize() >= 16, "N must be at least 16 to hold a UUID");
        arr[..16].copy_from_slice(self.as_bytes());
        arr
    }
}


impl<N: ArrayLength<u8>> AsArray<N> for GenericArray<u8, N> {
    fn as_array(&self) -> GenericArray<u8, N> {
        self.clone()
    }
}