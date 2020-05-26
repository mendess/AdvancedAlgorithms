mod util;
use aava::{
    algorithms::min_cut,
    util::disjoint_set::{PathCompression, PathHalving, PathSplitting},
};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use util::*;

macro_rules! bench_karger_stein {
    ($group:expr, $t:ty, $n:expr, $p: expr, $e:expr) => {
        $group.bench_function(
            BenchmarkId::new(
                &format!("karger_stein {}", stringify!($t)),
                format!("{}_{}_{}", $n, $p, $e)
            ),
            |b| {
                b.iter_batched(
                    || gen_edge_list($n, $p),
                    |mut graph| min_cut::karger_stein::<_, $t>(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
        $group.bench_function(
            BenchmarkId::new(
                &format!("fast_karger_stein {}", stringify!($t)),
                format!("{}_{}_{}", $n, $p, $e),
            ),
            |b| {
                b.iter_batched(
                    || gen_edge_list($n, $p),
                    |mut graph| min_cut::fast_karger_stein::<_, $t>(&mut graph),
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

macro_rules! bench_karger_stein_count {
    ($group:expr, $t:ty, $n:expr, $p: expr, $e:expr) => {
        $group.bench_function(
            BenchmarkId::new(
                &format!("karger_stein_count_{}", stringify!($t)),
                format!("{}_{}_{}", $n, $p, $e)
            ),
            |b| {
                b.iter_batched(
                    || gen_edge_list($n, $p),
                    |mut graph| min_cut::count::karger_stein_count::<_, $t>(&mut graph),
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
                    || gen_edge_list($n, $p),
                    |mut graph| {
                        min_cut::count::fast_karger_stein_count::<_, $t>(&mut graph)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    };
}

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("MinCut");
    for (n, p, e) in make_params() {
        group.throughput(Throughput::Elements(e as u64));
        bench_karger_stein!(group, PathCompression, n, p, e);
        bench_karger_stein!(group, PathSplitting, n, p, e);
        bench_karger_stein!(group, PathHalving, n, p, e);
    }
    group.finish();
}

pub fn bench_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("MinCutCount");
    for (n, p, e) in make_params() {
        group.throughput(Throughput::Elements(e as u64));
        bench_karger_stein_count!(group, PathCompression, n, p, e);
        bench_karger_stein_count!(group, PathSplitting, n, p, e);
        bench_karger_stein_count!(group, PathHalving, n, p, e);
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(30);
    targets = bench, bench_count
}
criterion_main!(benches);
