use aava::util::bit_array::BitArray;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

const ELEMS: usize = 1434;
pub fn iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("BitArray");
    group.throughput(Throughput::Elements(ELEMS as u64));
    for (r_size, v) in (1..8).map(|w| {
        (w, {
            let num_max = 1 << w;
            let mut v = BitArray::new(w, ELEMS);
            for n in 0..ELEMS {
                v.set(n as usize, (n as u8) % num_max);
            }
            v
        })
    }) {
        group.bench_with_input(
            BenchmarkId::new("iterator", format!("register_size_{}", r_size)),
            &v,
            |b, v| {
                b.iter(|| {
                    v.iter().for_each(|i| {
                        black_box(i);
                    })
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("loop", format!("register_size_{}", r_size)),
            &v,
            |b, v| {
                b.iter(|| {
                    for i in 0..ELEMS {
                        black_box(v.get(i));
                    }
                })
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = iteration
}
criterion_main!(benches);
