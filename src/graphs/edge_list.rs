use crate::graphs::{EdgeListGraph, FromEdges, Graph, WEdge};

pub struct EdgeList<N = (), E = ()> {
    edges: Vec<WEdge<N, E>>,
    n_vertices: usize,
}

impl<N, E> Graph for EdgeList<N, E> {
    type NodeWeight = N;
    type EdgeWeight = E;
    fn vertices(&self) -> usize {
        self.n_vertices
    }

    fn edges(&self) -> usize {
        self.edges.len()
    }
}

impl<N, E> FromEdges for EdgeList<N, E> {
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        I: IntoIterator<IntoIter = Iter, Item = WEdge<N, E>>,
        Iter: ExactSizeIterator<Item = WEdge<N, E>>,
    {
        Self {
            n_vertices: n,
            edges: list.into_iter().collect(),
        }
    }
}

impl<N, E> EdgeListGraph for EdgeList<N, E> {
    type Edges = Vec<WEdge<Self::NodeWeight, Self::EdgeWeight>>;

    fn as_edges(&self) -> &[WEdge<N, E>] {
        &self.edges[..]
    }

    fn as_edges_mut(&mut self) -> &mut [WEdge<N, E>] {
        &mut self.edges[..]
    }

    fn into_edges(self) -> Self::Edges {
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
