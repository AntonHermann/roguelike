use rand::*;
use arrayref::array_ref;
use sha2::{Digest};
use crate::*;

pub fn create_rng(seed: &str) -> StdRng {
    let mut hasher = sha2::Sha256::default();
    hasher.input(seed.as_bytes());
    let hash = format!("{:x}", hasher.result());
    let seed = array_ref!(hash.as_bytes(), 0, 32);
    rand::SeedableRng::from_seed(*seed)
}

pub fn random_pos(max_x: i32, max_y: i32, rng: &mut StdRng) -> Cell {
    let x = rng.gen_range(0, max_x);
    let y = rng.gen_range(0, max_y);

    (x, y)
}