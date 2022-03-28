use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_hello_world::simple_matrix::SimpleMatrix;
use rust_hello_world::simple_matrix2::SimpleMatrix2;
use rust_hello_world::rayon_matrix::RayonMatrix;
use rust_hello_world::gen_data;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    const N: usize = 500;
    const M: usize = 1000;
    const K: usize = 500;

    let simple_a = SimpleMatrix::from(N, M, gen_data(N, M, &mut rng));
    let simple_b = SimpleMatrix::from(M, K, gen_data(M, K, &mut rng));

    let simple_a2 = SimpleMatrix2::from(N, M, gen_data(N, M, &mut rng));
    let simple_b2 = SimpleMatrix2::from(M, K, gen_data(M, K, &mut rng));

    let rayon_a = RayonMatrix::from(N, M, gen_data(N, M, &mut rng));
    let rayon_b = RayonMatrix::from(M, K, gen_data(M, K, &mut rng));

    let mut simple_group = c.benchmark_group("single-core-group");
    simple_group.sample_size(10);
    simple_group.bench_function("gemm - single core - no extract",
                     |b| b.iter(|| black_box(&simple_a * &simple_b)));

    simple_group.bench_function("gemm - single core - extracted variable",
                     |b| b.iter(|| black_box(&simple_a2 * &simple_b2)));
    simple_group.finish();

    let mut par_group = c.benchmark_group("multi-core-group");
    par_group.sample_size(50);
    par_group.bench_function("gemm - multi core",
                    |b| b.iter(|| black_box(&rayon_a * &rayon_b)));
    par_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
