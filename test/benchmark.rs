use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fiolet_core::manifold::kl_divergence;

fn bench_kl_computation(c: &mut Criterion) {
    let p = vec![0.5; 4096];
    let q = vec![0.3; 4096];
    
    c.bench_function("kl_divergence_4096", |b| {
        b.iter(|| kl_divergence(black_box(&p), black_box(&q)))
    });
}

criterion_group!(benches, bench_kl_computation);
criterion_main!(benches);
