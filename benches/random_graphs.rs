mod util;
use aava::graphs::test_graphs::random_graph_er;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use util::*;

pub fn random_graph_er_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandomGraphER");
    for (n, p, e) in make_params() {
        group.throughput(Throughput::Elements(e as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_{}_{}", n, p, e)),
            &(n, p),
            |b, &i| b.iter_with_large_drop(|| random_graph_er(i.0, i.1, black_box(make_rng()))),
        );
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = random_graph_er_bench
}
criterion_main!(benches);
