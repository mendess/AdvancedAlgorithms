pub mod csr;
pub mod edge_list;
pub mod matrix;
pub mod test_graphs;

pub trait Graph {
    type EdgeWeight;
    fn vertices(&self) -> usize;
    fn edges(&self) -> usize;
}

pub type Edge = WEdge<()>;
pub type WEdge<E> = (usize, usize, E);

pub trait FromEdges: Graph<EdgeWeight = ()> {
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        Iter: ExactSizeIterator<Item = (usize, usize)>,
        I: IntoIterator<IntoIter = Iter, Item = (usize, usize)>;
}

pub trait WFromEdges: Graph {
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        Iter: ExactSizeIterator<Item = WEdge<Self::EdgeWeight>>,
        I: IntoIterator<IntoIter = Iter, Item = WEdge<Self::EdgeWeight>>;
}

impl<G> FromEdges for G
where
    G: WFromEdges,
    G: Graph<EdgeWeight = ()>,
{
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        Iter: ExactSizeIterator<Item = (usize, usize)>,
        I: IntoIterator<IntoIter = Iter, Item = (usize, usize)>,
    {
        WFromEdges::from_edges(n, list.into_iter().map(|(f, t)| (f, t, ())))
    }
}

pub trait EdgeListGraph: Graph {
    type Edges: IntoIterator<Item = WEdge<Self::EdgeWeight>>;

    fn as_edges(&self) -> &[WEdge<Self::EdgeWeight>];
    fn as_edges_mut(&mut self) -> &mut [WEdge<Self::EdgeWeight>];
    fn into_edges(self) -> Self::Edges;
}

pub trait Mutable: Graph<EdgeWeight = ()> {
    fn add_link(&mut self, from: usize, to: usize) -> bool;
}

pub trait WMutable: Graph {
    fn add_weighed_link(&mut self, from: usize, to: usize, w: Self::EdgeWeight) -> bool;
}

impl<G> Mutable for G
where
    G: WMutable,
    G: Graph<EdgeWeight = ()>,
{
    fn add_link(&mut self, from: usize, to: usize) -> bool {
        self.add_weighed_link(from, to, ())
    }
}

impl<'g, G> Graph for &'g G
where
    G: Graph,
{
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
    ($graph:ty = ($n:expr $(, _)?) { $($from:expr => $to:expr);*$(;)? }) => (
        <$graph as $crate::graphs::FromEdges>::from_edges(
            $n,
            [$(($from, $to),)*].iter().map(|&x| x)
        )
    );
    ( $graph:ty = ($n:expr $(, _)?) { $($from:expr => $to:expr , $w:expr);*$(;)? }) => (
        <$graph as $crate::graphs::WFromEdges>::from_edges(
            $n,
            [$(($from, $to, $w),)*].iter().map(|&x| x)
        )
    );
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct To<E> {
    pub to: usize,
    pub weight: E,
}

impl<E> PartialEq<usize> for To<E> {
    fn eq(&self, other: &usize) -> bool {
        self.to == *other
    }
}

impl From<usize> for To<()> {
    fn from(to: usize) -> Self {
        To { to, weight: () }
    }
}

#[cfg(test)]
mod test {
    use crate::graph;
    use crate::graphs::{matrix::Adjacency, Graph};

    #[test]
    fn test_macro() {
        let graph: Adjacency<()> = graph![Adjacency = (3) {
            0 => 1;
            0 => 2;
            1 => 0;
        }];
        assert_eq!(graph.edges(), 3);
    }

    #[test]
    fn test_macro_weights() {
        let graph: Adjacency<i32> = graph![Adjacency<_> = (3) {
            0 => 1, 42;
            0 => 2, 33;
            2 => 0, 25;
        }];
        assert_eq!(graph.edges(), 3);
        assert_eq!(graph[0].iter().map(|i| i.weight).sum::<i32>(), 42 + 33);
        assert_eq!(graph[1].iter().map(|i| i.weight).sum::<i32>(), 0);
        assert_eq!(graph[2].iter().map(|i| i.weight).sum::<i32>(), 25);
    }
}
