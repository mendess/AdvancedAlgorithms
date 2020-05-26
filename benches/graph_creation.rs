mod util;
use aava::graphs::{
    csr::CSR, edge_list::EdgeList, matrix::Adjacency, test_graphs::random_graph_er, Edge, FromEdges,
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use util::*;

fn gen_edges(n: usize, p: f64) -> (usize, Vec<Edge>) {
    (n, random_graph_er(n, p, make_rng()))
}

macro_rules! bench_graph {
    ($group:expr, $t:ty, $n:expr, $p:expr, $e:expr) => {
        $group.bench_function(
            BenchmarkId::new(stringify!($t), format!("{}_{}_{}", $n, $p, $e)),
            |b| {
                b.iter_batched(
                    || gen_edges($n, $p),
                    |(nn, edges)| <$t>::from_edges(nn, edges),
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

pub fn make_graphs(c: &mut Criterion) {
    let mut group = c.benchmark_group("GraphCreation");
    for (n, p, e) in make_params() {
        group.throughput(Throughput::Elements(e as u64));
        bench_graph!(group, EdgeList, n, p, e);
        bench_graph!(group, Adjacency, n, p, e);
        bench_graph!(group, CSR, n, p, e);
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = make_graphs
}
criterion_main!(benches);
