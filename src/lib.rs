use rand::prelude::ThreadRng;
use rand::Rng;

pub mod simple_matrix;
pub mod rayon_matrix;
pub mod simple_matrix2;

pub fn gen_data(n: usize, m: usize, rng: &mut ThreadRng) -> Vec<f32> {
    (0..n * m).map(|_| rng.gen()).collect()
}
