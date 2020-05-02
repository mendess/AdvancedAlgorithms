pub mod csr;
pub mod edge_list;
pub mod matrix;
pub mod test_graphs;

pub trait Graph {
    type NodeWeight;
    type EdgeWeight;
    fn vertices(&self) -> usize;
    fn edges(&self) -> usize;
}

pub type Edge = WEdge<(), ()>;
pub type WEdge<N, E> = (usize, usize, N, E);

pub trait FromEdges: Graph {
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        I: IntoIterator<IntoIter = Iter, Item = WEdge<Self::NodeWeight, Self::EdgeWeight>>,
        Iter: ExactSizeIterator<Item = WEdge<Self::NodeWeight, Self::EdgeWeight>>;
}

pub trait EdgeListGraph: Graph {
    type Edges: IntoIterator<Item = WEdge<Self::NodeWeight, Self::EdgeWeight>>;

    fn as_edges(&self) -> &[WEdge<Self::NodeWeight, Self::EdgeWeight>];
    fn as_edges_mut(&mut self) -> &mut [WEdge<Self::NodeWeight, Self::EdgeWeight>];
    fn into_edges(self) -> Self::Edges;
}

impl<'g, G> Graph for &'g G
where
    G: Graph,
{
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;

    fn vertices(&self) -> usize {
        (*self).vertices()
    }

    fn edges(&self) -> usize {
        (*self).edges()
    }
}

#[macro_export]
macro_rules! graph {
    ( $graph:ty = ($n_vertices:expr $(, _)?) { $($from:expr => $to:expr);*$(;)? }) => (
        <$graph as $crate::graphs::FromEdges>::from_edges(
            $n_vertices,
            [$(($from, $to, (), ()),)*].iter().map(|&x| x)
        )
    );
}

#[cfg(test)]
mod test {
    use crate::graph;
    use crate::graphs::{matrix::Adjacency, Graph};
    #[test]
    fn test_macro1() {
        let graph = graph![Adjacency = (3) {
            0 => 1;
            0 => 2;
            1 => 0;
        }];
        assert_eq!(graph.edges(), 3);
    }
}
