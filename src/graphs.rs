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
    ( $graph:ty = ($n_vertices:expr $(, _)?) { $($from:expr => $to:expr);*$(;)? }) => (
        <$graph as $crate::graphs::FromEdges>::from_edges(
            $n_vertices,
            [$(($from, $to, (), ()),)*].iter().map(|&x| x)
        )
    );
}
// ( $graph:ty = ($n_vertices:expr, $n_links:expr) { $($from:expr => $to:expr);*$(;)? }) => (
//     ::static_assertions::const_assert!($n_links >= <[_]>::len(&[$($from),*]));
//     <$graph as $crate::graphs::FromEdges>::parcial(
//         $n_vertices,
//         $n_links,
//         [$(($from, $to),)*].iter().map(|&x| x)
//     )
// );
