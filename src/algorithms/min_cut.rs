use crate::graphs::ContractableGraph;
use std::num::NonZeroUsize;

fn possible_min_cut<G: ContractableGraph>(mut graph: G) -> usize {
    let mut vertices = graph.vertices();
    let mut rng = rand::thread_rng();
    while vertices > 2 {
        graph.contract(graph.random_edge(&mut rng));
        vertices -= 1;
    }
    graph.edges()
}

pub fn min_cut<G: ContractableGraph + Clone>(graph: G, boosting: NonZeroUsize) -> usize {
    (0..boosting.get())
        .map(|_| possible_min_cut(graph.clone()))
        .max()
        .expect("Expected bool")
}

pub mod karger_stein {
    use crate::{
        graphs::{Edge, EdgeListGraph},
        util::disjoint_set::*,
    };
    use rand::{
        distributions::uniform::{UniformInt, UniformSampler},
        thread_rng,
    };

    fn contract<N, D>(edges: &mut [Edge<N>], ds: &mut D, comp: usize, cur_node: &mut usize) -> usize
    where
        D: DisjointSet,
        N: Into<usize> + Copy,
    {
        let mut rng = thread_rng();
        let mut cur = *cur_node;
        while cur < edges.len() && ds.components() > comp {
            let i = UniformInt::<usize>::sample_single(cur, edges.len(), &mut rng);
            edges.swap(cur, i);
            if !ds.are_connected(edges[cur].0.into(), edges[cur].1.into()) {
                ds.union(edges[cur].0.into(), edges[cur].1.into());
            }
            cur += 1;
        }
        *cur_node = cur;
        if comp == 2 {
            edges
                .iter()
                .filter(|e| !ds.are_connected(e.0.into(), e.1.into()))
                .count()
        } else {
            0
        }
    }

    fn min_cut<N, D>(edges: &mut [Edge<N>], mut ds: D, current_node: usize) -> usize
    where
        D: DisjointSet + Clone,
        N: Into<usize> + Copy,
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

    pub fn karger<G>(edges: &mut G, n_nodes: usize) -> usize
    where
        G: EdgeListGraph,
        G::NodeId: Into<usize> + Copy,
    {
        let mut edges = edges.as_edges_mut();
        let log = (!n_nodes).trailing_zeros();
        let runs = log * log + 2;
        (0..runs)
            .map(|_| min_cut(&mut edges, SimpleDisjointSet::new(n_nodes), 0))
            .min()
            .unwrap()
    }

    fn fast_min_cut<N>(
        edges: &mut [Edge<N>],
        ds: &mut UndoDisjointSet,
        current_node: usize,
    ) -> usize
    where
        N: Into<usize> + Copy,
    {
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

    pub fn karger_stein<G>(edges: &mut G, n_nodes: usize) -> usize
    where
        G: EdgeListGraph,
        G::NodeId: Into<usize> + Copy,
    {
        let mut edges = edges.as_edges_mut();
        let log = (!n_nodes).trailing_zeros();
        let runs = log * log + 2;
        (0..runs)
            .map(|_| fast_min_cut(&mut edges, &mut UndoDisjointSet::new(n_nodes), 0))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::{edge_list::EdgeList, test_graphs};
    use rand::thread_rng;
    #[test]
    fn karger_stein() {
        assert_eq!(
            super::karger_stein::karger_stein(&mut test_graphs::graph_one::<EdgeList>(), 10),
            3
        )
    }

    #[test]
    fn karger() {
        assert_eq!(
            super::karger_stein::karger(&mut test_graphs::graph_one::<EdgeList>(), 10),
            3
        )
    }

    #[test]
    fn karget_stein_random() {
        super::karger_stein::karger_stein(
            &mut test_graphs::random_graph_er::<EdgeList, usize, _>(10, 0.2, thread_rng()),
            10,
        );
    }
}
