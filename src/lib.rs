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
