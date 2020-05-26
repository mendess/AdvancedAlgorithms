use aava::{
    algorithms::min_cut,
    graphs::{edge_list::EdgeList, test_graphs::random_graph_er, FromEdges},
    util::disjoint_set::{PathCompression, PathHalving, PathSplitting, UndoDisjointSet},
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use rand::{distributions::Distribution, rngs::SmallRng, SeedableRng};
use rand_distr::Binomial;
use std::convert::TryInto;

fn make_rng() -> SmallRng {
    SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)
}

fn gen_graph(n: usize, p: f64) -> EdgeList {
    EdgeList::from_edges(n, random_graph_er(n, p, make_rng()))
}

macro_rules! bench_karger_stein {
    ($group:expr, $t:ty, $n:expr, $p: expr, $e:expr) => {
        $group.bench_function(
            BenchmarkId::new(
                &format!("fast_karger_stein {}", stringify!($t)),
                format!("{}_{}_{}", $n, $p, $e),
            ),
            |b| {
                b.iter_batched(
                    || gen_graph($n, $p),
                    |mut graph| min_cut::fast_karger_stein::<_, UndoDisjointSet<$t>>(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
        $group.bench_function(
            BenchmarkId::new(
                &format!("fast_karger_stein_count_{}", stringify!($t)),
                format!("{}_{}_{}", $n, $p, $e),
            ),
            |b| {
                b.iter_batched(
                    || gen_graph($n, $p),
                    |mut graph| {
                        min_cut::count::fast_karger_stein_count::<_, UndoDisjointSet<$t>>(
                            &mut graph,
                        )
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    };
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("MinCut");
    let params = [1_usize, 5, 9]
        .iter()
        .map(|i| i * 10)
        .flat_map(|n| [0.4, 0.5, 0.7].iter().map(move |&p| (n, p)))
        .chain(std::iter::once((100, 0.5)))
        .map(|(n, p)| {
            let dist = Binomial::new((n * (n - 1) / 2).try_into().unwrap(), p).unwrap();
            let e = dist.sample(&mut make_rng());
            (n, p, e)
        })
        .map(|(n, p, e)| (n, p, e))
        .collect::<Vec<_>>();
    for (n, p, e) in params {
        group.throughput(Throughput::Elements(e as u64));
        group.bench_function(
            BenchmarkId::new("karger_stein", format!("{}_{}_{}", n, p, e)),
            |b| {
                b.iter_batched(
                    || gen_graph(n, p),
                    |mut graph| min_cut::karger_stein(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("karger_stein_count", format!("{}_{}_{}", n, p, e)),
            |b| {
                b.iter_batched(
                    || gen_graph(n, p),
                    |mut graph| min_cut::count::karger_stein_count(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
        bench_karger_stein!(group, PathCompression, n, p, e);
        bench_karger_stein!(group, PathSplitting, n, p, e);
        bench_karger_stein!(group, PathHalving, n, p, e);
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(30);
    targets = bench
}
criterion_main!(benches);
