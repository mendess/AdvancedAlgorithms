#![warn(rust_2018_idioms)]
pub mod algorithms;
pub mod graphs;
pub mod util;
use graphs::{csr::CSR, FromEdges};
use util::ToExactSizeIter;

pub fn main() {
    let g = CSR::from_edges(
        5,
        (0..5)
            .flat_map(|from| (0..5).into_iter().map(move |to| (from, to)))
            .to_exact_size(25),
    );
    println!("{:?}", g);

    let d = 10;
    let graphs = (2..9)
        .map(|i| i * 100000)
        .flat_map(|i| [1, 2, 3, 5, 8, 12, 50].iter().copied().map(move |o| (i, o)))
        .map(|(n, o)| {
            (
                n,
                o,
                graphs::test_graphs::clustered(n, d, o, rand::thread_rng()),
            )
        })
        .inspect(|(n, o, _)| eprintln!("Built g {{ n: {}, d: {}, o: {} }}", n, d, o));
    for (n, o, g) in graphs {
        eprintln!(
            "For n: {}, d: {}, o: {}, cc: {}",
            n,
            d,
            o,
            algorithms::clustering_coef::c_coef(
                10,
                &g.neighbourhoods()
                    .map(|(i, n)| (i, n.count()))
                    .filter(|x| x.1 > 2)
                    .map(|i| i.0)
                    .collect::<Vec<_>>(),
                &g,
                rand::thread_rng(),
            )
        )
    }
}
