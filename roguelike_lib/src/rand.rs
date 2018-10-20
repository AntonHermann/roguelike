use rand::*;
use arrayref::array_ref;
use sha2::{Digest};

pub fn create_rng(seed: &str) -> StdRng {
    let mut hasher = sha2::Sha256::default();
    hasher.input(seed.as_bytes());
    let hash = format!("{:x}", hasher.result());
    let seed = array_ref!(hash.as_bytes(), 0, 32);
    rand::SeedableRng::from_seed(*seed)
}