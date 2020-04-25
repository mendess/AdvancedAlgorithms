#![warn(rust_2018_idioms)]
pub mod algorithms;
pub mod graphs;
pub mod util;
use graphs::{csr::CSR, FromEdges, ToExactSizeIter};

pub fn main() {
    let g = CSR::from_edges(
        5,
        (0..5)
            .flat_map(|from| (0..5).into_iter().map(move |to| (from, to, (), ())))
            .to_exact_size(25),
    );
    println!("{:?}", g);
}

#[cfg(test)]
mod test {
    use crate::graph;
    use crate::graphs::{matrix::Adjacency, Graph};
    #[test]
    fn test_macro1() {
        let graph = graph![Adjacency = (3) {
            0 => 1;
            0 => 2;
            1 => 0;
        }];
        assert_eq!(graph.edges(), 3);
    }
}
