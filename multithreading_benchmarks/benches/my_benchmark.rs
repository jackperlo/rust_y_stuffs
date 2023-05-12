use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solution_finder::{search_for_operations_no_threads, create_n_threads};
mod solution_finder;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Brute Force NO THREADS, items=[1,2,3,4,5]:", |b| b.iter(|| (search_for_operations_no_threads(black_box(&[1,2,3,4,5])))));

    c.bench_function("Brute Force 2 THREADS, items=[1,2,3,4,5]:", |b| b.iter(|| (create_n_threads(2, vec![1,2,3,4,5]))));

    c.bench_function("Brute Force 3 THREADS, items=[1,2,3,4,5]: ", |b| b.iter(|| (create_n_threads(3, vec![1,2,3,4,5]))));

    c.bench_function("Brute Force 5 THREADS, items=[1,2,3,4,5]: ", |b| b.iter(|| (create_n_threads(5, vec![1,2,3,4,5]))));

    c.bench_function("Brute Force 10 THREADS, items=[1,2,3,4,5]: ", |b| b.iter(|| (create_n_threads(10, vec![1,2,3,4,5]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);