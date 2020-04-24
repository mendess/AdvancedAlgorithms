use aava::graphs::{
    csr::GraphCSR, edge_list::EdgeList, matrix::Adjacency, test_graphs::random_graph_er,
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{distributions::Distribution, rngs::SmallRng, SeedableRng};
use rand_distr::Binomial;
use std::convert::TryInto;

fn make_rng() -> SmallRng {
    SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)
}

pub fn make_graphs(c: &mut Criterion) {
    let mut group = c.benchmark_group("GraphCreation");
    let params = [1_usize, 5, 9]
        .iter()
        .map(|i| i * 100)
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
        group.bench_with_input(
            BenchmarkId::new("edge_list", format!("{}_{}_{}", n, p, e)),
            &(n, p),
            |b, &i| {
                b.iter_with_large_drop(|| {
                    random_graph_er::<EdgeList, usize, _>(
                        i.0,
                        i.1,
                        black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                    )
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("adjacency", format!("{}_{}_{}", n, p, e)),
            &(n, p),
            |b, &i| {
                b.iter_with_large_drop(|| {
                    random_graph_er::<Adjacency, usize, _>(
                        i.0,
                        i.1,
                        black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                    )
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("csr", format!("{}_{}_{}", n, p, e)),
            &(n, p),
            |b, &i| {
                b.iter_with_large_drop(|| {
                    random_graph_er::<GraphCSR, usize, _>(
                        i.0,
                        i.1,
                        black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                    )
                })
            },
        );
    }
    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = make_graphs
}
criterion_main!(benches);
