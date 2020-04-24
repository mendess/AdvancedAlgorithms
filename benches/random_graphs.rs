use aava::graphs::{
    edge_list::EdgeList,
    test_graphs::{random_graph_er, random_graph_er_concrete},
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::SmallRng, SeedableRng};

pub fn random_graph_er_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandomGraphER");
    for &n in [100_usize, 200, 300, 400, 500, 600, 700, 800, 900].iter() {
        for &percent in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7].iter() {
            let v_e = percent * (n as f64).floor();
            group.throughput(Throughput::Elements(v_e as u64));
            group.bench_with_input(BenchmarkId::new("generic", v_e), &(n, percent), |b, &i| {
                b.iter(|| {
                    random_graph_er::<EdgeList, usize, _>(
                        black_box(i.0),
                        black_box(i.1),
                        black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                    )
                })
            });
            group.bench_with_input(BenchmarkId::new("concrete", v_e), &(n, percent), |b, &i| {
                b.iter(|| {
                    random_graph_er_concrete::<EdgeList, _>(
                        black_box(i.0),
                        black_box(i.1),
                        black_box(SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)),
                    )
                })
            });
        }
    }
    group.finish();
}

criterion_group!(benches, random_graph_er_bench);
criterion_main!(benches);
