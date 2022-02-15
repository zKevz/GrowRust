pub mod color;
pub mod error;
pub mod math;
pub mod mem;
pub mod variant;
pub mod variant_function;
pub mod variantlist;

use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

pub fn hash(data: &[u8]) -> u32 {
    let mut hash = 0x55555555;

    for val in data.iter() {
        hash = ((hash >> 27) + (hash << 5)) + (*val as u32);
    }

    hash
}

pub fn random<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}
