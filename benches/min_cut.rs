use aava::{
    algorithms::min_cut,
    graphs::{edge_list::EdgeList, test_graphs::random_graph_er, FromEdges},
};
use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use rand::{distributions::Distribution, rngs::SmallRng, SeedableRng};
use rand_distr::Binomial;
use std::convert::TryInto;

fn make_rng() -> SmallRng {
    SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)
}

fn gen_graph(n: usize, p: f64) -> EdgeList {
    EdgeList::from_edges(n, random_graph_er(n, p, black_box(make_rng())))
}

pub fn bench0(c: &mut Criterion) {
    let mut group = c.benchmark_group("MinCut");
    let params = [1_usize, 5, 9]
        .iter()
        .map(|i| i * 10)
        .flat_map(|n| [0.4, 0.5, 0.7].iter().map(move |&p| (n, p)))
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
            BenchmarkId::new("fast_karger_stein", format!("{}_{}_{}", n, p, e)),
            |b| {
                b.iter_batched(
                    || gen_graph(n, p),
                    |mut graph| min_cut::fast_karger_stein(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

pub fn bench1(c: &mut Criterion) {
    let mut group = c.benchmark_group("MinCutCount");
    let params = [1_usize, 5, 9]
        .iter()
        .map(|i| i * 10)
        .flat_map(|n| [0.4, 0.5, 0.7].iter().map(move |&p| (n, p)))
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
                    |mut graph| min_cut::count::karger_stein_count(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
        group.bench_function(
            BenchmarkId::new("fast_karger_stein", format!("{}_{}_{}", n, p, e)),
            |b| {
                b.iter_batched(
                    || gen_graph(n, p),
                    |mut graph| min_cut::count::fast_karger_stein_count(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench0, bench1
}
criterion_main!(benches);
