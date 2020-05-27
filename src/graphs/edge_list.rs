use crate::graphs::{EdgeListGraph, WFromEdges, Graph, WEdge};

pub struct EdgeList<E = ()> {
    edges: Vec<WEdge<E>>,
    n_vertices: usize,
}

impl<E> Graph for EdgeList<E> {
    type EdgeWeight = E;
    fn vertices(&self) -> usize {
        self.n_vertices
    }

    fn edges(&self) -> usize {
        self.edges.len()
    }
}

impl<E> WFromEdges for EdgeList<E> {
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        I: IntoIterator<IntoIter = Iter, Item = WEdge<E>>,
        Iter: ExactSizeIterator<Item = WEdge<E>>,
    {
        Self {
            n_vertices: n,
            edges: list.into_iter().collect(),
        }
    }
}

impl<E> EdgeListGraph for EdgeList<E> {
    type Edges = Vec<WEdge<Self::EdgeWeight>>;

    fn as_edges(&self) -> &[WEdge<E>] {
        &self.edges[..]
    }

    fn as_edges_mut(&mut self) -> &mut [WEdge<E>] {
        &mut self.edges[..]
    }

    fn into_edges(self) -> Self::Edges {
        self.edges
    }
}

impl EdgeList<()> {
    pub fn add_link(&mut self, from: usize, to: usize) -> bool {
        if self.edges.capacity() == self.edges.len() {
            false
        } else {
            self.edges.push((from, to, ()));
            true
        }
    }
}

impl<E> EdgeList<E> {
    pub fn add_link_weights(&mut self, from: usize, to: usize, e: E) -> bool {
        if self.edges.capacity() == self.edges.len() {
            false
        } else {
            self.edges.push((from, to, e));
            true
        }
    }
}
