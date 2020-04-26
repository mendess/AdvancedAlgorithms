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
) -> Vec<WEdge<N, E>>
where
    N: Clone,
    E: Clone,
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
        edges
            .iter()
            .filter(|e| !ds.are_connected(e.0, e.1))
            .cloned()
            .collect()
    } else {
        Default::default()
    }
}

fn min_cut<N, E, D>(edges: &mut [WEdge<N, E>], mut ds: D, current_node: usize) -> Vec<WEdge<N, E>>
where
    N: Clone,
    E: Clone,
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
        let m1 = min_cut(edges, ds1, updateable_node);

        updateable_node = current_node;
        let mut ds2 = ds;
        contract(edges, &mut ds2, t, &mut updateable_node);
        let m2 = min_cut(edges, ds2, updateable_node);
        if m1.len() < m2.len() {
            m1
        } else {
            m2
        }
    }
}

pub fn karger_stein<G>(edges: &mut G) -> Vec<WEdge<G::NodeWeight, G::EdgeWeight>>
where
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone,
    G: EdgeListGraph,
{
    let n_vert = edges.vertices();
    let edges = edges.as_edges_mut();
    let log = (!n_vert).trailing_zeros();
    let runs = log * log + 2;
    (0..runs)
        .map(|_| min_cut(edges, SimpleDisjointSet::new(n_vert), 0))
        .min_by_key(|cut| cut.len())
        .unwrap()
}

fn fast_min_cut<N, E>(
    edges: &mut [WEdge<N, E>],
    ds: &mut UndoDisjointSet,
    current_node: usize,
) -> Vec<WEdge<N, E>>
where
    N: Clone,
    E: Clone,
{
    if ds.components() < 6 {
        let mut cur = current_node;
        contract(edges, ds, 2, &mut cur)
    } else {
        let t = 1 + (ds.components() as f64 / 2.0_f64.sqrt()) as usize;
        let mut updateable_node = current_node;
        ds.save_state();
        contract(edges, ds, t, &mut updateable_node);
        let m1 = fast_min_cut(edges, ds, updateable_node);
        ds.restore_state();

        ds.save_state();
        updateable_node = current_node;
        contract(edges, ds, t, &mut updateable_node);
        let m2 = fast_min_cut(edges, ds, updateable_node);
        ds.restore_state();
        if m1.len() < m2.len() {
            m1
        } else {
            m2
        }
    }
}

pub fn fast_karger_stein<G>(edges: &mut G) -> Vec<WEdge<G::NodeWeight, G::EdgeWeight>>
where
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone,
    G: EdgeListGraph,
{
    let n_vert = edges.vertices();
    let edges = edges.as_edges_mut();
    let log = (!n_vert).trailing_zeros();
    let runs = log * log + 2;
    (0..runs)
        .map(|_| fast_min_cut(edges, &mut UndoDisjointSet::new(n_vert), 0))
        .min_by_key(|cut| cut.len())
        .unwrap()
}

pub mod count {
    use crate::{
        graphs::{EdgeListGraph, WEdge},
        util::disjoint_set::*,
    };
    use rand::{
        distributions::uniform::{UniformInt, UniformSampler},
        thread_rng,
    };

    fn contract_count<N, E, D>(
        edges: &mut [WEdge<N, E>],
        ds: &mut D,
        comp: usize,
        cur_node: &mut usize,
    ) -> usize
    where
        N: Clone,
        E: Clone,
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
            Default::default()
        }
    }

    fn min_cut_count<N, E, D>(edges: &mut [WEdge<N, E>], mut ds: D, current_node: usize) -> usize
    where
        N: Clone,
        E: Clone,
        D: DisjointSet + Clone,
    {
        if ds.components() < 6 {
            let mut cur = current_node;
            contract_count(edges, &mut ds, 2, &mut cur)
        } else {
            let t = 1 + (ds.components() as f64 / 2.0_f64.sqrt()) as usize;
            let mut updateable_node = current_node;
            let mut ds1 = ds.clone();
            contract_count(edges, &mut ds1, t, &mut updateable_node);
            let m1 = min_cut_count(edges, ds1, updateable_node);
            updateable_node = current_node;
            let mut ds2 = ds.clone();
            contract_count(edges, &mut ds2, t, &mut updateable_node);
            let m2 = min_cut_count(edges, ds2, updateable_node);
            if m1 < m2 {
                m1
            } else {
                m2
            }
        }
    }

    pub fn karger_stein_count<G>(edges: &mut G) -> usize
    where
        G::NodeWeight: Clone,
        G::EdgeWeight: Clone,
        G: EdgeListGraph,
    {
        let n_vert = edges.vertices();
        let edges = edges.as_edges_mut();
        let log = (!n_vert).trailing_zeros();
        let runs = log * log + 2;
        (0..runs)
            .map(|_| min_cut_count(edges, SimpleDisjointSet::new(n_vert), 0))
            .min()
            .unwrap()
    }

    fn fast_min_cut_count<N, E>(
        edges: &mut [WEdge<N, E>],
        ds: &mut UndoDisjointSet,
        current_node: usize,
    ) -> usize
    where
        N: Clone,
        E: Clone,
    {
        if ds.components() < 6 {
            let mut cur = current_node;
            contract_count(edges, ds, 2, &mut cur)
        } else {
            let t = 1 + (ds.components() as f64 / 2.0_f64.sqrt()) as usize;
            let mut updateable_node = current_node;
            ds.save_state();
            contract_count(edges, ds, t, &mut updateable_node);
            let m1 = fast_min_cut_count(edges, ds, updateable_node);
            ds.restore_state();
            ds.save_state();
            updateable_node = current_node;
            contract_count(edges, ds, t, &mut updateable_node);
            let m2 = fast_min_cut_count(edges, ds, updateable_node);
            ds.restore_state();
            m1.min(m2)
        }
    }

    pub fn fast_karger_stein_count<G>(edges: &mut G) -> usize
    where
        G::NodeWeight: Clone,
        G::EdgeWeight: Clone,
        G: EdgeListGraph,
    {
        let n_vert = edges.vertices();
        let edges = edges.as_edges_mut();
        let log = (!n_vert).trailing_zeros();
        let runs = log * log + 2;
        (0..runs)
            .map(|_| fast_min_cut_count(edges, &mut UndoDisjointSet::new(n_vert), 0))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::{edge_list::EdgeList, test_graphs, FromEdges};
    use rand::thread_rng;
    #[test]
    fn karger_stein() {
        let cut = super::karger_stein(&mut test_graphs::graph_one::<EdgeList>())
            .into_iter()
            .map(|e| (e.0, e.1))
            .collect::<Vec<_>>();
        assert!(
            test_graphs::GRAPH_ONE_MIN_CUT
                .iter()
                .all(|e| cut.iter().any(|e1| e == e1))
                && test_graphs::GRAPH_ONE_MIN_CUT.len() == cut.len(),
            "Different min cut. Expected: {:?} Got: {:?}",
            test_graphs::GRAPH_ONE_MIN_CUT,
            cut
        );
    }

    #[test]
    fn fast_karger_stein() {
        let cut = super::fast_karger_stein(&mut test_graphs::graph_one::<EdgeList>())
            .into_iter()
            .map(|e| (e.0, e.1))
            .collect::<Vec<_>>();
        assert!(
            test_graphs::GRAPH_ONE_MIN_CUT
                .iter()
                .all(|e| cut.iter().any(|e1| e == e1))
                && test_graphs::GRAPH_ONE_MIN_CUT.len() == cut.len(),
            "Different min cut. Expected: {:?} Got: {:?}",
            test_graphs::GRAPH_ONE_MIN_CUT,
            cut
        );
    }

    #[test]
    fn karget_stein_random() {
        super::fast_karger_stein(&mut EdgeList::from_edges(
            10,
            test_graphs::random_graph_er(10, 0.2, thread_rng()),
        ));
    }
}
