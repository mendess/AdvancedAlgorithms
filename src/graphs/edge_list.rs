use crate::graphs::{Edge, EdgeListGraph, FromEdges, Graph, GraphWeighted, WEdge};

pub struct EdgeList<N = (), E = ()> {
    edges: Vec<WEdge<N, E>>,
    n_vertices: usize,
}

impl<N, E> Graph for EdgeList<N, E> {
    fn vertices(&self) -> usize {
        self.n_vertices
    }

    fn edges(&self) -> usize {
        self.edges.len()
    }
}

impl<N, E> GraphWeighted for EdgeList<N, E> {
    type NodeWeight = N;
    type EdgeWeight = E;
}

impl FromEdges for EdgeList<(), ()> {
    fn from_edges<I>(n: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge>,
    {
        Self {
            n_vertices: n,
            edges: edges.map(|e| (e.0, e.1, (), ())).collect(),
        }
    }
}

impl<N, E> EdgeListGraph<N, E> for EdgeList<N, E> {
    type Edges = Vec<WEdge<N, E>>;

    fn as_edges(&self) -> &[WEdge<N, E>] {
        &self.edges[..]
    }

    fn as_edges_mut(&mut self) -> &mut [WEdge<N, E>] {
        &mut self.edges[..]
    }

    fn into_edges(self) -> Vec<WEdge<N, E>> {
        self.edges
    }
}

impl EdgeList<(), ()> {
    pub fn add_link(&mut self, from: usize, to: usize) -> bool {
        if self.edges.capacity() == self.edges.len() {
            false
        } else {
            self.edges.push((from, to, (), ()));
            true
        }
    }
}

impl<N, E> EdgeList<N, E> {
    pub fn add_link_weights(&mut self, from: usize, to: usize, n: N, e: E) -> bool {
        if self.edges.capacity() == self.edges.len() {
            false
        } else {
            self.edges.push((from, to, n, e));
            true
        }
    }
}
