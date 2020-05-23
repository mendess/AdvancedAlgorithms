use aava::{
    algorithms::apl,
    graphs::{csr::CSR, edge_list::EdgeList, test_graphs::random_graph_er, FromEdges, Graph},
    hyper_ball::{
        hyper_ball,
        hyper_counters::{CompactHyperLogLog, HyperLogLog, B},
    },
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use rand::{distributions::Distribution, rngs::SmallRng, SeedableRng};
use rand_distr::Binomial;
use std::convert::TryInto;

fn make_rng() -> SmallRng {
    SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)
}

fn gen_graph<G>(n: usize, p: f64) -> G
where
    G: FromEdges,
    G: Graph<NodeWeight = (), EdgeWeight = ()>,
{
    G::from_edges(n, random_graph_er(n, p, make_rng()))
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("APL");
    let params = [1_usize, 5, 9]
        .iter()
        .map(|i| i * 10)
        .flat_map(|n| [0.4, 0.5, 0.7].iter().map(move |&p| (n, p)))
        .chain(std::iter::once((500, 0.5)))
        .map(|(n, p)| {
            let dist = Binomial::new((n * (n - 1) / 2).try_into().unwrap(), p).unwrap();
            let e = dist.sample(&mut make_rng());
            (n, p, e)
        })
        .map(|(n, p, e)| (n, p, e))
        .collect::<Vec<_>>();
    for (n, p, e) in params {
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
