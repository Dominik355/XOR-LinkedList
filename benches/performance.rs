use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::collections::LinkedList as StdLinkedList;
use std::hint::black_box;
use xor_ll::LinkedList as XorLinkedList;

fn bench_push_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_front");

    for &n in &[1_000u32, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(format!("xor_front_{}", n), &n, |b, &n| {
            b.iter_batched(
                || XorLinkedList::new(),
                |mut list| {
                    for i in 0..n {
                        list.push_front(black_box(i));
                    }
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(format!("std_front_{}", n), &n, |b, &n| {
            b.iter_batched(
                || StdLinkedList::new(),
                |mut list| {
                    for i in 0..n {
                        list.push_front(black_box(i));
                    }
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_push_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_back");

    for &n in &[1_000u32, 10_000, 100_000] {
        group.bench_with_input(format!("xor_back_{}", n), &n, |b, &n| {
            b.iter_batched(
                || XorLinkedList::new(),
                |mut list| {
                    for i in 0..n {
                        list.push_back(black_box(i));
                    }
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(format!("std_back_{}", n), &n, |b, &n| {
            b.iter_batched(
                || StdLinkedList::new(),
                |mut list| {
                    for i in 0..n {
                        list.push_back(black_box(i));
                    }
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_pop_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_front");

    for &n in &[1_000u32, 10_000, 100_000] {
        group.bench_with_input(format!("xor_pop_front_{}", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut list = XorLinkedList::new();
                    for i in 0..n {
                        list.push_back(i);
                    }
                    list
                },
                |mut list| {
                    while list.pop_front().is_some() {}
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(format!("std_pop_front_{}", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut list = StdLinkedList::new();
                    for i in 0..n {
                        list.push_back(i);
                    }
                    list
                },
                |mut list| {
                    while list.pop_front().is_some() {}
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_pop_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_back");

    for &n in &[1_000u32, 10_000, 100_000] {
        group.bench_with_input(format!("xor_pop_back_{}", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut list = XorLinkedList::new();
                    for i in 0..n {
                        list.push_back(i);
                    }
                    list
                },
                |mut list| {
                    while list.pop_back().is_some() {}
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(format!("std_pop_back_{}", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut list = StdLinkedList::new();
                    for i in 0..n {
                        list.push_back(i);
                    }
                    list
                },
                |mut list| {
                    while list.pop_back().is_some() {}
                    black_box(list);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_push_front,
    bench_push_back,
    bench_pop_front,
    bench_pop_back
);
criterion_main! {
    benches
}
