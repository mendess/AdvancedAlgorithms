use crate::graphs::{EdgeListGraph, Graph, MutableGraph, Vertex};
use rand::{seq::SliceRandom, Rng};

type NodeId<N> = <EdgeList<N> as Graph>::NodeId;
type Edge<N> = super::Edge<NodeId<N>>;

pub struct EdgeList<N: Vertex = usize> {
    edges: Vec<Edge<N>>,
    n_vertices: usize,
}

impl<N> Graph for EdgeList<N>
where
    N: Vertex,
{
    type NodeId = N;

    fn new<I>(n_vertices: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge<N>>,
    {
        Self {
            n_vertices,
            edges: edges.collect(),
        }
    }

    fn vertices(&self) -> usize {
        self.n_vertices
    }

    fn edges(&self) -> usize {
        self.edges.len()
    }

    fn random_edge<R: Rng>(&self, mut rng: R) -> Edge<N> {
        *self.edges.choose(&mut rng).unwrap()
    }
}

impl<N> EdgeListGraph for EdgeList<N>
where
    N: Vertex,
{
    type Edges = Vec<Edge<N>>;

    fn as_edges(&self) -> &[Edge<N>] {
        &self.edges[..]
    }

    fn as_edges_mut(&mut self) -> &mut [Edge<N>] {
        &mut self.edges[..]
    }

    fn into_edges(self) -> Vec<Edge<N>> {
        self.edges
    }
}

impl<N> MutableGraph for EdgeList<N>
where
    N: Vertex,
{
    fn parcial<I>(n_vertices: usize, n_links: usize, edges: I) -> Self
    where
        I: IntoIterator<Item = Edge<N>>,
    {
        let mut elist = Vec::with_capacity(n_links);
        elist.extend(edges);
        Self {
            n_vertices,
            edges: elist,
        }
    }

    fn add_link(&mut self, from: Self::NodeId, to: Self::NodeId) -> bool {
        if self.edges.capacity() == self.edges.len() {
            false
        } else {
            self.edges.push((from, to));
            true
        }
    }
}
