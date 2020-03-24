use crate::graphs::MutableGraph;
use std::num::NonZeroUsize;

fn possible_min_cut<G: MutableGraph>(mut graph: G) -> usize
where
    G: std::fmt::Debug,
{
    let mut vertices = graph.vertices();
    let mut rng = rand::thread_rng();
    while vertices > 2 {
        graph.contract(graph.random_edge(&mut rng));
        vertices -= 1;
    }
    graph.edges()
}

pub fn min_cut<G: MutableGraph + Clone>(graph: G, boosting: NonZeroUsize) -> usize
where
    G: std::fmt::Debug,
{
    (0..boosting.get())
        .map(|_| possible_min_cut(graph.clone()))
        .max()
        .expect("Expected bool")
}

#[cfg(test)]
mod test {
    use crate::graphs::{matrix::Adjacency, test_graphs};
    use std::num::NonZeroUsize;
    #[test]
    fn min_cut() {
        assert_eq!(
            super::min_cut(
                test_graphs::graph_one::<Adjacency>(),
                NonZeroUsize::new(10).unwrap()
            ),
            3
        );
    }
}
