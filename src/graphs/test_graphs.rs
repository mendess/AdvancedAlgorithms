use super::Graph;
use crate::graph;
// use rand_distr::Binomial;

pub fn graph_one<G: Graph<NodeId = usize>>() -> G {
    graph!(G = (10) {
        0 => 1;
        0 => 2;
        0 => 3;
        0 => 4;
        1 => 0;
        1 => 2;
        1 => 3;
        1 => 4;
        2 => 0;
        2 => 1;
        2 => 3;
        2 => 4;
        3 => 0;
        3 => 1;
        3 => 2;
        3 => 4;
        4 => 0;
        4 => 1;
        4 => 2;
        4 => 3;

        5 => 6;
        5 => 7;
        5 => 8;
        5 => 9;
        6 => 5;
        6 => 7;
        6 => 8;
        6 => 9;
        7 => 5;
        7 => 6;
        7 => 8;
        7 => 9;
        8 => 5;
        8 => 6;
        8 => 7;
        8 => 9;
        9 => 5;
        9 => 6;
        9 => 7;
        9 => 8;
        // the min cut
        2 => 6;
        4 => 5;
        3 => 7;
    })
}

///// G(N,P) - Evdos Ronmi
/////
/////        (N)(N - 1)
///// E[N] = ---------- P
/////            2
/////
///// G(N,M) <=> G(N,P)
/////
/////
//fn random_graph<G: Graph<NodeId=usize>>(n: usize) -> G {
//    let dist = Binomial::new(n, (n * (n - 1)) / 2);

//}
