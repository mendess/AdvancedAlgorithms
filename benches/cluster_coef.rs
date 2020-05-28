mod util;
use aava::{
    algorithms::clustering_coef::c_coef,
    graphs::{csr::CSR, matrix::Adjacency, test_graphs::clustered, FromEdges, Graph},
    util::ToExactSizeIter,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use util::*;

pub fn make_params() -> impl Iterator<Item = ((Adjacency, CSR, Vec<usize>), usize, usize)> {
    (1..9)
        .map(|i| i * 500)
        .flat_map(|n| [1, 2, 3, 5, 8, 12, 50].iter().map(move |&o| (n, 10, o)))
        .map(|(n, d, o)| {
            let adj = clustered::<Adjacency, _>(n, d, o, make_rng());
            let csr = CSR::from_edges(
                n,
                adj.neighbourhoods()
                    .flat_map(|(f, tos)| tos.map(move |t| (f, t.to)))
                    .to_exact_size(adj.edges()),
            );
            let node_indexes = adj
                .neighbourhoods()
                .map(|(i, n)| (i, n.count()))
                .filter(|x| x.1 > 2)
                .map(|i| i.0)
                .collect::<Vec<_>>();
            ((adj, csr, node_indexes), d, o)
        })
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Clustering Coef");
    for ((adj, csr, node_indexes), d, o) in make_params() {
        let n = adj.vertices() + adj.edges();
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::new("adj", format!("{}_{}_{}", n, d, o)), |b| {
            b.iter(|| c_coef(20, black_box(&node_indexes), black_box(&adj), make_rng()))
        });
        group.bench_function(BenchmarkId::new("csr", format!("{}_{}_{}", n, d, o)), |b| {
            b.iter(|| c_coef(20, black_box(&node_indexes), black_box(&csr), make_rng()))
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(30);
    targets = bench
}
criterion_main!(benches);
