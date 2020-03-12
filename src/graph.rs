pub mod csr;
pub mod matrix;

use std::ops::Index;

pub trait Graph
where
    Self: Index<<Self as Graph>::NodeName>,
    for<'a> &'a Self::Output: IntoIterator<Item = &'a Self::NodeName>,
{
    type NodeName;
}

pub trait StaticGraph: Graph
where
    for<'a> &'a Self::Output: IntoIterator<Item = &'a Self::NodeName>,
{
    fn new<I>(n_vertices: usize, n_links: usize, edges: I) -> Self
    where
        I: IntoIterator<Item = (Self::NodeName, Self::NodeName)>;
}

pub trait MutableGraph: StaticGraph
where
    for<'a> &'a Self::Output: IntoIterator<Item = &'a Self::NodeName>,
{
    fn new(n_vertices: usize, n_links: usize) -> Self
    where
        Self: Sized,
    {
        StaticGraph::new(n_vertices, n_links, std::iter::empty())
    }

    fn add_link(&mut self, from: usize, to: usize) -> bool;
}
