use crate::{
    graphs::{EdgeListGraph, WEdge},
    util::disjoint_set::*,
};
use rand::{
    distributions::uniform::{UniformInt, UniformSampler},
    thread_rng,
};

fn contract<N, E, D>(
    edges: &mut [WEdge<N, E>],
    ds: &mut D,
    comp: usize,
    cur_node: &mut usize,
) -> usize
where
    D: DisjointSet,
{
    let mut rng = thread_rng();
    let mut cur = *cur_node;
    while cur < edges.len() && ds.components() > comp {
        let i = UniformInt::<usize>::sample_single(cur, edges.len(), &mut rng);
        edges.swap(cur, i);
        if !ds.are_connected(edges[cur].0, edges[cur].1) {
            ds.union(edges[cur].0, edges[cur].1);
        }
        cur += 1;
    }
    *cur_node = cur;
    if comp == 2 {
        edges.iter().filter(|e| !ds.are_connected(e.0, e.1)).count()
    } else {
        0
    }
}

fn min_cut<N, E, D>(edges: &mut [WEdge<N, E>], mut ds: D, current_node: usize) -> usize
where
    D: DisjointSet + Clone,
{
    if ds.components() < 6 {
        let mut cur = current_node;
        contract(edges, &mut ds, 2, &mut cur)
    } else {
        let t = 1 + (ds.components() as f64 / 2.0_f64.sqrt()) as usize;
        let mut updateable_node = current_node;
        let mut ds1 = ds.clone();
        contract(edges, &mut ds1, t, &mut updateable_node);
        let m = min_cut(edges, ds1, updateable_node);
        updateable_node = current_node;
        let mut ds2 = ds.clone();
        contract(edges, &mut ds2, t, &mut updateable_node);
        let m = m.min(min_cut(edges, ds2, updateable_node));
        m
    }
}

pub fn karger_stein<G, N, E>(edges: &mut G) -> usize
where
    G: EdgeListGraph<N, E>,
{
    let n_vert = edges.vertices();
    let edges = edges.as_edges_mut();
    let log = (!n_vert).trailing_zeros();
    let runs = log * log + 2;
    (0..runs)
        .map(|_| min_cut(edges, SimpleDisjointSet::new(n_vert), 0))
        .min()
        .unwrap()
}

fn fast_min_cut<N, E>(
    edges: &mut [WEdge<N, E>],
    ds: &mut UndoDisjointSet,
    current_node: usize,
) -> usize {
    if ds.components() < 6 {
        let mut cur = current_node;
        contract(edges, ds, 2, &mut cur)
    } else {
        let t = 1 + (ds.components() as f64 / 2.0_f64.sqrt()) as usize;
        let mut updateable_node = current_node;
        ds.save_state();
        contract(edges, ds, t, &mut updateable_node);
        let m = fast_min_cut(edges, ds, updateable_node);
        ds.restore_state();
        ds.save_state();
        updateable_node = current_node;
        contract(edges, ds, t, &mut updateable_node);
        let m = m.min(fast_min_cut(edges, ds, updateable_node));
        ds.restore_state();
        m
    }
}

pub fn fast_karger_stein<G, N, E>(edges: &mut G) -> usize
where
    G: EdgeListGraph<N, E>,
{
    let n_vert = edges.vertices();
    let mut edges = edges.as_edges_mut();
    let log = (!n_vert).trailing_zeros();
    let runs = log * log + 2;
    (0..runs)
        .map(|_| fast_min_cut(&mut edges, &mut UndoDisjointSet::new(n_vert), 0))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::graphs::{edge_list::EdgeList, test_graphs, FromEdges};
    use rand::thread_rng;
    #[test]
    fn karger_stein() {
        assert_eq!(
            super::karger_stein(&mut test_graphs::graph_one::<EdgeList>()),
            3
        )
    }

    #[test]
    fn fast_karger_stein() {
        assert_eq!(
            super::fast_karger_stein(&mut test_graphs::graph_one::<EdgeList>()),
            3
        )
    }

    #[test]
    fn karget_stein_random() {
        super::fast_karger_stein(&mut EdgeList::from_edges(
            10,
            test_graphs::random_graph_er(10, 0.2, thread_rng()),
        ));
    }
}
