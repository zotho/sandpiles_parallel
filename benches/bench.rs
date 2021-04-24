use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};

use sandpiles_parallel::Field;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(346345234);
    let mut sample_field: Field<u32> = Field::new(1000, 1000);
    sample_field
        .data
        .iter_mut()
        .for_each(|cell| *cell = black_box(rng.gen_range(0..=400)));

    let mut sample_field_2 = sample_field.clone();

    c.bench_function("bench slow", |b| b.iter(|| sample_field.slow_update()));

    c.bench_function("bench iter", |b| b.iter(|| sample_field_2.update_iter()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
