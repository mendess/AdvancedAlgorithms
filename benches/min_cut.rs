use aava::{
    algorithms::min_cut::karger_stein,
    graphs::{
        edge_list::EdgeList,
        test_graphs::{random_graph_er, random_graph_er_concrete},
    },
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::SmallRng, SeedableRng};

pub fn karger_stein_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("karger");
    let mut rng = SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
    for percent in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7].iter() {
        group.throughput(Throughput::Elements((percent * (100 as f64)).floor() as u64));
        let g = random_graph_er::<EdgeList, usize, _>(500, *percent, &mut rng);
        group.bench_with_input(BenchmarkId::from_parameter(percent), &g, |b, i| {
            b.iter(|| {
                karger_stein::karger_stein(i, black_box(500))
            })
        });
    }
    group.finish();
}

criterion_group!(benches, karger_stein_bench);
criterion_main!(benches);
