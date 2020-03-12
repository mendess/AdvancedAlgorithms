pub mod graph;
use graph::{csr::GraphCSR, StaticGraph};

pub fn main() {
    let g = GraphCSR::new(
        5,
        25,
        (0..5).flat_map(|from| (0..5).into_iter().map(move |to| (from, to))),
    );
    println!("{:?}", g);
}
