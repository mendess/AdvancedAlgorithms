#![allow(dead_code)]

use aava::graphs::{edge_list::EdgeList, test_graphs::random_graph_er, FromEdges, Graph};
use rand::{rngs::SmallRng, SeedableRng};

pub fn make_rng() -> SmallRng {
    SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EED)
}

pub fn gen_graph<G>(n: usize, p: f64) -> G
where
    G: FromEdges,
{
    G::from_edges(n, random_graph_er(n, p, make_rng()))
}

pub fn gen_edge_list(n: usize, p: f64) -> EdgeList {
    gen_graph::<EdgeList>(n, p)
}

pub fn make_params() -> impl Iterator<Item = (usize, f64, usize)> {
    [
        (20, 0.6),
        (30, 0.4),
        (30, 0.8),
        (40, 0.6),
        (70, 0.2),
        (40, 0.8),
        (50, 0.6),
        (50, 0.8),
        (60, 0.6),
        (60, 0.8),
        (70, 0.6),
        (70, 0.8),
        (80, 0.8),
    ]
    .iter()
    .copied()
    .map(|(n, p)| {
        let g = gen_edge_list(n, p);
        (n, p, g.vertices() + g.edges())
    })
}
