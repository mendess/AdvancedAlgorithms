pub mod graph;
pub mod algorithms;
use graph::{csr::GraphCSR, Graph};

pub fn main() {
    let g = GraphCSR::new(
        5,
        25,
        (0..5).flat_map(|from| (0..5).into_iter().map(move |to| (from, to))),
    );
    println!("{:?}", g);
}
