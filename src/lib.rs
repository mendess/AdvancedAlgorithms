pub mod graph;
use graph::GraphCSR;

pub fn main() {
    let mut g = GraphCSR::new(5, 5 * 5);
    for i in 0..5 {
        for j in 0..5 {
            g.add_link(i, j);
        }
    }
    println!("{:?}", g);
}
