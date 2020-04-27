use crate::graphs::*;
pub fn apl<G>(g: &G) -> f64
where
    G: EdgeListGraph,
{
    let n = g.vertices();
    let mut distances = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        distances[i][i] = 0;
    }
    for p in g.as_edges() {
        distances[p.0][p.1] = 1;
        distances[p.1][p.0] = 1;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distances[i][k] != usize::MAX && distances[k][j] != usize::MAX {
                    distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                }
            }
        }
    }
    distances.iter().flatten().sum::<usize>() as f64 / ((n * (n - 1)) as f64 / 2.0)
}

#[cfg(test)]
mod tests {
    use crate::graphs::{edge_list::EdgeList, test_graphs};
    #[test]
    fn apl0() {
        let apl = super::apl(&test_graphs::graph_one::<EdgeList>());
        assert!(apl >= 3.145 && apl <= 3.18)
    }
}
