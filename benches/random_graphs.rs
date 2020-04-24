use aava::graphs::{
    edge_list::EdgeList,
    test_graphs::{random_graph_er, random_graph_er_concrete},
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::SmallRng, SeedableRng};

pub fn random_graph_er_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandomGraphER");
    let params = (1..=9)
        .map(|i| i * 100)
        .zip(&[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7])
        .map(|(n, &p)| (n, p, (n as f64) * p))
        .map(|(n, p, e)| (n, p, e.floor() as usize))
        .collect::<Vec<_>>();
    for (n, p, e) in params {
        group.throughput(Throughput::Elements(e as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("generic_{}_{}_{}", n, p, e)),
            &(n, p),
            |b, &i| {
                b.iter(|| {
                    random_graph_er::<EdgeList, usize, _>(
                        i.0,
                        i.1,
                        black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                    )
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("concrete_{}_{}_{}", n, p, e)),
            &(n, p), |b, &i| {
            b.iter(|| {
                random_graph_er_concrete::<EdgeList, _>(
                    i.0,
                    i.1,
                    black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                )
            })
        });
    }
    group.finish();
}

criterion_group!(benches, random_graph_er_bench);
criterion_main!(benches);
