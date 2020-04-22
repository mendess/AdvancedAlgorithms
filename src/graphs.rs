pub mod csr;
pub mod edge_list;
pub mod matrix;
pub mod test_graphs;

use rand::Rng;

pub type Edge<N> = (N, N);

pub trait Graph {
    type NodeId: Clone;

    fn new<I>(n_vertices: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge<Self::NodeId>>;

    fn vertices(&self) -> usize;
    fn edges(&self) -> usize;
    fn random_edge<R: Rng>(&self, rng: R) -> Edge<Self::NodeId>;
}

pub trait EdgeListGraph: Graph {
    fn as_edges(&self) -> &[Edge<Self::NodeId>];
    fn as_edges_mut(&mut self) -> &mut [Edge<Self::NodeId>];
    fn into_edges(self) -> Vec<Edge<Self::NodeId>>;
}

pub trait MutableGraph: Graph {
    fn parcial<I>(n_vertices: usize, n_links: usize, edges: I) -> Self
    where
        I: IntoIterator<Item = Edge<Self::NodeId>>;

    fn empty(n_vertices: usize, n_links: usize) -> Self
    where
        Self: Sized,
    {
        MutableGraph::parcial(n_vertices, n_links, std::iter::empty())
    }

    fn add_link(&mut self, from: Self::NodeId, to: Self::NodeId) -> bool;
}

pub trait ContractableGraph: MutableGraph {
    fn contract(&mut self, edge: Edge<Self::NodeId>);
}

pub struct ExactSizeIter<I> {
    pub iter: I,
    pub size: usize,
}

impl<I: Iterator> Iterator for ExactSizeIter<I> {
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.size, self.iter.next()) {
            (0, _) => None,
            (_, None) => panic!("Passed iterator was smaller than expected"),
            (_, i) => {
                self.size -= 1;
                i
            }
        }
    }
}

impl<I: Iterator> ExactSizeIterator for ExactSizeIter<I> {
    #[inline]
    fn len(&self) -> usize {
        self.size
    }
}

pub trait ToExactSizeIter: Iterator + Sized {
    #[inline]
    fn to_exact_size(self, size: usize) -> ExactSizeIter<Self> {
        ExactSizeIter { iter: self, size }
    }
}

impl<I: Iterator> ToExactSizeIter for I {}

#[macro_export]
macro_rules! graph {
    ( $graph:ty = ($n_vertices:expr, $n_links:expr) { $($from:expr => $to:expr);*$(;)? }) => (
        ::static_assertions::const_assert!($n_links >= <[_]>::len(&[$($from),*]));
        <$graph as $crate::graphs::MutableGraph>::parcial(
            $n_vertices,
            $n_links,
            [$(($from, $to),)*].iter().map(|&x| x)
        )
    );
    ( $graph:ty = ($n_vertices:expr $(, _)?) { $($from:expr => $to:expr);*$(;)? }) => (
        <$graph as $crate::graphs::Graph>::new(
            $n_vertices,
            [$(($from, $to),)*].iter().map(|&x| x)
        )
    );
}
