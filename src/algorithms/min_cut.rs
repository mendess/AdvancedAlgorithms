use crate::{
    graphs::{EdgeListGraph, WEdge},
    util::disjoint_set::*,
};
use itertools::Itertools;
use rand::{
    distributions::uniform::{UniformInt, UniformSampler},
    thread_rng,
};

fn contract<E, D>(
    edges: &mut [WEdge<E>],
    ds: &mut D,
    comp: usize,
    cur_node: &mut usize,
) -> Vec<(usize, usize)>
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
    debug_assert_eq!(
        ds.components(),
        comp,
        "Couldn't contract\nGraph: {:?}",
        edges.iter().map(|e| (e.0, e.1)).format(",")
    );
    if comp == 2 {
        edges
            .iter()
            .filter(|e| !ds.are_connected(e.0, e.1))
            .map(|(f, t, _)| (*f, *t))
            .collect()
    } else {
        Default::default()
    }
}

fn min_cut<E, D>(edges: &mut [WEdge<E>], mut ds: D, mut current_node: usize) -> Vec<(usize, usize)>
where
    D: DisjointSet + Clone,
{
    if ds.components() < 6 {
        contract(edges, &mut ds, 2, &mut current_node)
    } else {
        let t = 1 + (ds.components() as f64 / 2.0_f64.sqrt()) as usize;
        let mut current_node_copy = current_node;
        let mut ds1 = ds.clone();
        contract(edges, &mut ds1, t, &mut current_node_copy);
        let m1 = min_cut(edges, ds1, current_node_copy);

        contract(edges, &mut ds, t, &mut current_node);
        let m2 = min_cut(edges, ds, current_node);
        if m1.len() < m2.len() {
            m1
        } else {
            m2
        }
    }
}

pub fn karger_stein<G, F>(edges: &mut G) -> Vec<(usize, usize)>
where
    G: EdgeListGraph,
    F: FindMode,
    SimpleDisjointSet<F>: DisjointSet + Clone,
{
    let n_vert = edges.vertices();
    let edges = edges.as_edges_mut();
    let log = (!n_vert).trailing_zeros();
    let runs = log * log + 2;
    (0..runs)
        .map(|_| min_cut(edges, SimpleDisjointSet::<F>::new(n_vert), 0))
        .min_by_key(|cut| cut.len())
        .unwrap()
}

fn fast_min_cut<E, F>(
    edges: &mut [WEdge<E>],
    ds: &mut UndoDisjointSet<F>,
    current_node: usize,
) -> Vec<(usize, usize)>
where
    F: FindMode,
    UndoDisjointSet<F>: DisjointSet,
{
    if ds.components() < 6 {
        let mut cur = current_node;
        contract(edges, ds, 2, &mut cur)
    } else {
        let t = 1 + (ds.components() as f64 / 2.0_f64.sqrt()) as usize;
        let mut current_node_copy = current_node;
        ds.save_state();
        contract(edges, ds, t, &mut current_node_copy);
        let m1 = fast_min_cut(edges, ds, current_node_copy);
        ds.restore_state();

        // ds.save_state();
        current_node_copy = current_node;
        contract(edges, ds, t, &mut current_node_copy);
        let m2 = fast_min_cut(edges, ds, current_node_copy);
        // ds.restore_state();
        if m1.len() < m2.len() {
            m1
        } else {
            m2
        }
    }
}

pub fn fast_karger_stein<G, F>(edges: &mut G) -> Vec<(usize, usize)>
where
    G: EdgeListGraph,
    F: FindMode,
    UndoDisjointSet<F>: DisjointSet,
{
    let n_vert = edges.vertices();
    let edges = edges.as_edges_mut();
    let log = (!n_vert).trailing_zeros();
    let runs = log * log + 2;
    (0..runs)
        .map(|_| fast_min_cut(edges, &mut UndoDisjointSet::<F>::new(n_vert), 0))
        .min_by_key(|cut| cut.len())
        .unwrap()
}

pub mod count {
    use crate::{
        graphs::{EdgeListGraph, WEdge},
        util::disjoint_set::{DisjointSet, FindMode, SimpleDisjointSet, UndoDisjointSet},
    };
    use itertools::Itertools;
    use rand::{
        distributions::uniform::{UniformInt, UniformSampler},
        thread_rng,
    };

    fn contract_count<E, D>(
        edges: &mut [WEdge<E>],
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
        debug_assert_eq!(
            ds.components(),
            comp,
            "Couldn't contract\nGraph: {:?}",
            edges.iter().map(|e| (e.0, e.1)).format(",")
        );
        if comp == 2 {
            edges.iter().filter(|e| !ds.are_connected(e.0, e.1)).count()
        } else {
            Default::default()
        }
    }

    fn min_cut_count<E, D>(edges: &mut [WEdge<E>], mut ds: D, current_node: usize) -> usize
    where
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
            contract_count(edges, &mut ds, t, &mut updateable_node);
            let m2 = min_cut_count(edges, ds, updateable_node);
            m1.min(m2)
        }
    }

    pub fn karger_stein_count<G, F>(edges: &mut G) -> usize
    where
        G: EdgeListGraph,
        F: FindMode,
        SimpleDisjointSet<F>: DisjointSet + Clone,
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

    fn fast_min_cut_count<E, F>(
        edges: &mut [WEdge<E>],
        ds: &mut UndoDisjointSet<F>,
        current_node: usize,
    ) -> usize
    where
        F: FindMode,
        UndoDisjointSet<F>: DisjointSet,
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

    pub fn fast_karger_stein_count<G, F>(edges: &mut G) -> usize
    where
        G: EdgeListGraph,
        F: FindMode,
        UndoDisjointSet<F>: DisjointSet,
    {
        let n_vert = edges.vertices();
        let edges = edges.as_edges_mut();
        let log = (!n_vert).trailing_zeros();
        let runs = log * log + 2;
        (0..runs)
            .map(|_| fast_min_cut_count(edges, &mut UndoDisjointSet::<F>::new(n_vert), 0))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        graphs::{edge_list::EdgeList, test_graphs, FromEdges},
        util::disjoint_set::{PathCompression, PathHalving, PathSplitting},
    };

    const SUCCSESS_RATE: usize = 7;

    macro_rules! test_fast_karger_stein {
        ($t:ty) => {
            paste::item! {
                #[test]
                #[allow(non_snake_case)]
                fn [<karger_stein_ $t>]() {
                    let succ = check_min_cut(&test_graphs::GRAPH_ONE_MIN_CUT, || {
                        super::karger_stein::<_, $t>(&mut test_graphs::graph_one::<EdgeList>())
                    });
                    assert!(succ > SUCCSESS_RATE, "Got it right {} times", succ)
                }

                #[test]
                #[allow(non_snake_case)]
                fn [<fast_karger_stein_ $t>]() {
                    let succ = check_min_cut(&test_graphs::GRAPH_ONE_MIN_CUT, || {
                        super::fast_karger_stein::<_, $t>(
                            &mut test_graphs::graph_one::<EdgeList>(),
                        )
                    });
                    assert!(succ > SUCCSESS_RATE, "Got it right {} times", succ)
                }
                #[test]
                #[allow(non_snake_case)]
                fn [<karger_stein_random_ $t>]() {
                    let mut g = Vec::new();
                    for i in 0..10 {
                        for j in 0..10 {
                            if i != j {
                                g.push((i, j));
                            }
                        }
                    }
                    let g0 = g
                        .iter()
                        .map(|(a, b)| (a + 10, b + 10))
                        .collect::<Vec<_>>();
                    g.extend(g0);
                    let min_cut = [(1, 11), (2, 12), (3, 13), (4, 14)];
                    for (a, b) in &min_cut {
                        g.push((*a, *b, ))
                    }
                    let succ = check_min_cut(&min_cut, || {
                        super::fast_karger_stein::<_, $t>(
                            &mut EdgeList::from_edges(20, g.clone())
                        )
                    });
                    assert!(succ > SUCCSESS_RATE, "Got it right {} times", succ)
                }
            }
        };
    }

    test_fast_karger_stein!(PathCompression);
    test_fast_karger_stein!(PathHalving);
    test_fast_karger_stein!(PathSplitting);

    #[cfg(test)]
    fn check_min_cut<F>(cut: &[(usize, usize)], attempts: F) -> usize
    where
        F: Fn() -> Vec<(usize, usize)>,
    {
        let mut cut: Vec<_> = cut.to_owned();
        cut.sort();
        (0..10)
            .map(|_| attempts())
            .filter(|attempt| {
                let mut attempt = attempt.iter().map(|(a, b)| (*a, *b)).collect::<Vec<_>>();
                attempt.sort();
                cut == attempt
            })
            .count()
    }

    #[cfg(test)]
    mod count {
        use super::super::count::*;
        use super::SUCCSESS_RATE;
        use crate::{
            graphs::{edge_list::EdgeList, test_graphs, FromEdges},
            util::disjoint_set::{PathCompression, PathHalving, PathSplitting},
        };

        macro_rules! test_fast_karger_stein {
            ($t:ty) => {
                paste::item! {
                    #[test]
                    #[allow(non_snake_case)]
                    fn [<karger_stein_ $t>]() {
                        let succ = check_min_cut(test_graphs::GRAPH_ONE_MIN_CUT.len(), || {
                            karger_stein_count::<_, $t>(&mut test_graphs::graph_one::<EdgeList>())
                        });
                        assert!(succ > SUCCSESS_RATE, "Got it right {} times", succ)
                    }

                    #[test]
                    #[allow(non_snake_case)]
                    fn [<fast_karger_stein_ $t>]() {
                        let succ = check_min_cut(test_graphs::GRAPH_ONE_MIN_CUT.len(), || {
                            fast_karger_stein_count::<_, $t>(
                                &mut test_graphs::graph_one::<EdgeList>()
                            )
                        });
                        assert!(succ > SUCCSESS_RATE, "Got it right {} times", succ)
                    }

                    #[test]
                    #[allow(non_snake_case)]
                    fn [<karger_stein_random_ $t>]() {
                        let mut g = Vec::new();
                        for i in 0..10 {
                            for j in 0..10 {
                                if i != j {
                                    g.push((i, j));
                                }
                            }
                        }
                        let g0 = g
                            .iter()
                            .map(|(a, b)| (a + 10, b + 10))
                            .collect::<Vec<_>>();
                        g.extend(g0);
                        let min_cut = [(1, 11), (2, 12), (3, 13), (4, 14)];
                        for (a, b) in &min_cut {
                            g.push((*a, *b))
                        }
                        let succ = check_min_cut(min_cut.len(), || {
                            fast_karger_stein_count::<_, $t>(
                                &mut EdgeList::from_edges(20, g.clone())
                            )
                        });
                        assert!(succ > SUCCSESS_RATE, "Got it right {} times", succ)
                    }
                }
            };
        }

        test_fast_karger_stein!(PathCompression);
        test_fast_karger_stein!(PathHalving);
        test_fast_karger_stein!(PathSplitting);

        #[cfg(test)]
        fn check_min_cut<F>(cut: usize, attempts: F) -> usize
        where
            F: Fn() -> usize,
        {
            (0..10).filter(|_| cut == attempts()).count()
        }
    }
}
