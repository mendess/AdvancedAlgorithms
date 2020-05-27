mod util;
use aava::{algorithms::clustering_coef::c_coef, graphs::test_graphs::clustered};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use util::*;

pub fn make_params() -> Vec<(usize, usize, usize)> {
    [1_usize, 5, 9]
        .iter()
        .map(|i| i * 100)
        .flat_map(|n| [1, 2, 3, 5, 8, 12, 50].iter().map(move |&o| (n, 10, o)))
        .collect::<Vec<_>>()
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Clustering Coef");
    for (n, d, o) in make_params() {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(
            BenchmarkId::new("c_coef", format!("{}_{}_{}", n, d, o)),
            |b| {
                b.iter_batched(
                    || {
                        let g = clustered(n, d, o, make_rng());
                        let node_indexes = g
                            .neighbourhoods()
                            .map(|(i, n)| (i, n.count()))
                            .filter(|x| x.1 > 2)
                            .map(|i| i.0)
                            .collect::<Vec<_>>();
                        (node_indexes, g)
                    },
                    |(n, g)| c_coef(20, &n, &g, make_rng()),
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(30);
    targets = bench
}
criterion_main!(benches);
