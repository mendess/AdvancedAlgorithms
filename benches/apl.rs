mod util;
use aava::{
    algorithms::apl,
    graphs::{csr::CSR, edge_list::EdgeList, Graph},
    hyper_ball::{
        hyper_ball,
        hyper_counters::{CompactHyperLogLog, HyperLogLog, B},
    },
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use util::*;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("APL");
    for (n, p, e) in make_params() {
        group.throughput(Throughput::Elements(e as u64));
        group.bench_function(BenchmarkId::new("apl", format!("{}_{}_{}", n, p, e)), |b| {
            b.iter_batched(
                || gen_graph::<EdgeList>(n, p),
                |mut graph| apl::apl(&mut graph),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(
            BenchmarkId::new("hyperLogLog", format!("{}_{}_{}", n, p, e)),
            |b| {
                b.iter_batched(
                    || gen_graph::<CSR>(n, p),
                    |graph| {
                        hyper_ball(&graph, || HyperLogLog::new(B::B4));
                    },
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("compact_hyperLogLog", format!("{}_{}_{}", n, p, e)),
            |b| {
                b.iter_batched(
                    || gen_graph::<CSR>(n, p),
                    |graph| {
                        hyper_ball(&graph, || CompactHyperLogLog::new(B::B4, graph.vertices()));
                    },
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
