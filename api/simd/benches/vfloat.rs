use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use simd::VF32x4;

fn bench_op(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut rng = rand::thread_rng();
    let mut group = c.benchmark_group("vsimd");

    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        let size4 = 4 * size;
        let mut v0 = VF32x4::rand(*size, &mut rng);
        let v1 = VF32x4::rand(*size, &mut rng);

        group.throughput(Throughput::Elements(size4 as u64));
        group.bench_with_input(BenchmarkId::new("add", size4), size, |b, _| {
            b.iter(|| VF32x4::add_assign(&mut v0, &v1));
        });
        group.bench_with_input(BenchmarkId::new("mul", size4), size, |b, _| {
            b.iter(|| VF32x4::mul_assign(&mut v0, &v1));
        });
        group.bench_with_input(BenchmarkId::new("div", size4), size, |b, _| {
            b.iter(|| VF32x4::div_assign(&mut v0, &v1));
        });
        group.bench_with_input(BenchmarkId::new("dot", size4), size, |b, _| {
            b.iter(|| VF32x4::dot(&v0, &v1));
        });
        group.bench_with_input(BenchmarkId::new("rev_half", size4), size, |b, _| {
            b.iter(|| VF32x4::rev_half(&mut v0));
        });
        group.bench_with_input(BenchmarkId::new("rev", size4), size, |b, _| {
            b.iter(|| VF32x4::rev(&mut v0));
        });
        group.bench_with_input(BenchmarkId::new("normalize", size4), size, |b, _| {
            b.iter(|| VF32x4::normalize(&mut v0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_op);
criterion_main!(benches);
