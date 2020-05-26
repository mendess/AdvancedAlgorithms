#![allow(dead_code)]

use aava::graphs::{edge_list::EdgeList, test_graphs::random_graph_er, FromEdges, Graph};
use rand::{rngs::SmallRng, SeedableRng};

pub fn make_rng() -> SmallRng {
    SmallRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64)
}

pub fn gen_graph<G>(n: usize, p: f64) -> G
where
    G: FromEdges,
    G: Graph<NodeWeight = (), EdgeWeight = ()>,
{
    G::from_edges(n, random_graph_er(n, p, make_rng()))
}

pub fn gen_edge_list(n: usize, p: f64) -> EdgeList {
    gen_graph::<EdgeList>(n, p)
}

pub fn make_params() -> Vec<(usize, f64, usize)> {
    [1_usize, 5, 9]
        .iter()
        .map(|i| i * 10)
        .flat_map(|n| [0.4, 0.5, 0.7].iter().map(move |&p| (n, p)))
        .chain(std::iter::once((100, 0.5)))
        .map(|(n, p)| {
            let g = gen_edge_list(n, p);
            (n, p, g.vertices() + g.edges())
        })
        .map(|(n, p, e)| (n, p, e))
        .collect::<Vec<_>>()
}
