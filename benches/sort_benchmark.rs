#[macro_use]
extern crate criterion;
extern crate base_sort;

use base_sort::BaseSort;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("buble sort (not sorted array)", |b| {
        b.iter(|| {
            (1..1000).rev().collect::<Vec<i32>>().bubble_sort();
        });
    });

    c.bench_function("buble sort (sorted array)", |b| {
        b.iter(|| {
            (1..1000).collect::<Vec<i32>>().bubble_sort();
        });
    });

    c.bench_function("comb sort (not sorted array)", |b| {
        b.iter(|| {
            (1..1000).rev().collect::<Vec<i32>>().comb_sort();
        });
    });

    c.bench_function("comb sort (sorted array)", |b| {
        b.iter(|| {
            (1..1000).collect::<Vec<i32>>().comb_sort();
        });
    });

    c.bench_function("inerted sort (not sorted array)", |b| {
        b.iter(|| {
            (1..1000).rev().collect::<Vec<i32>>().insertion_sort();
        });
    });

    c.bench_function("inerted sort (sorted array)", |b| {
        b.iter(|| {
            (1..1000).collect::<Vec<i32>>().insertion_sort();
        });
    });

    c.bench_function("shell sort (not sorted array)", |b| {
        b.iter(|| {
            (1..1000).rev().collect::<Vec<i32>>().shell_sort();
        });
    });

    c.bench_function("shell sort (sorted array)", |b| {
        b.iter(|| {
            (1..1000).collect::<Vec<i32>>().shell_sort();
        });
    });

    c.bench_function("shake sort (not sorted array)", |b| {
        b.iter(|| {
            (1..1000).rev().collect::<Vec<i32>>().shake_sort();
        });
    });

    c.bench_function("shake sort (sorted array)", |b| {
        b.iter(|| {
            (1..1000).collect::<Vec<i32>>().shake_sort();
        });
    });

    c.bench_function("odd_even sort (not sorted array)", |b| {
        b.iter(|| {
            (1..1000).rev().collect::<Vec<i32>>().odd_even_sort();
        });
    });

    c.bench_function("odd_even sort (sorted array)", |b| {
        b.iter(|| {
            (1..1000).collect::<Vec<i32>>().odd_even_sort();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
