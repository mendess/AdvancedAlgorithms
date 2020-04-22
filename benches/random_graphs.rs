use aava::graphs::{
    edge_list::EdgeList,
    test_graphs::{random_graph_er, random_graph_er_concrete},
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::SmallRng, SeedableRng};

pub fn random_graph_er_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandomGraphER");
    for percent in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7].iter() {
        group.throughput(Throughput::Elements((percent * (100 as f64)).floor() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(percent), percent, |b, &i| {
            b.iter(|| {
                random_graph_er::<EdgeList, usize, _>(
                    black_box(500),
                    black_box(i),
                    black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                )
            })
        });
    }
    group.finish();
}

pub fn random_graph_er_bench_concrete(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandomGraphERConcrete");
    for percent in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7].iter() {
        group.throughput(Throughput::Elements((percent * (100 as f64)).floor() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(percent), percent, |b, &i| {
            b.iter(|| {
                random_graph_er_concrete::<EdgeList, _>(
                    black_box(500),
                    black_box(i),
                    black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                )
            })
        });
    }
    group.finish();
}

criterion_group!(benches, random_graph_er_bench, random_graph_er_bench_concrete);
criterion_main!(benches);
