pub mod csr;
pub mod matrix;

use std::ops::Index;
use rand::Rng;

pub type Edge<G> = (<G as Graph>::NodeName, <G as Graph>::NodeName);

pub trait Graph
where
    Self: Index<<Self as Graph>::NodeName>,
    for<'a> &'a Self::Output: IntoIterator<Item = &'a Self::NodeName>,
{
    type NodeName;
    fn new<I>(n_vertices: usize, n_links: usize, edges: I) -> Self
    where
        I: IntoIterator<Item = (Self::NodeName, Self::NodeName)>;

    fn vertices(&self) -> usize;
    fn edges(&self) -> usize;
    fn random_edge<R: Rng>(&self, rng: R) -> Edge<Self>;
}

pub trait MutableGraph: Graph
where
    for<'a> &'a Self::Output: IntoIterator<Item = &'a Self::NodeName>,
{
    fn empty(n_vertices: usize, n_links: usize) -> Self
    where
        Self: Sized,
    {
        Graph::new(n_vertices, n_links, std::iter::empty())
    }

    fn add_link(&mut self, from: usize, to: usize) -> bool;

    fn contract(&mut self, edge: Edge<Self>);
}
