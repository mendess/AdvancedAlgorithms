// use static_assertions::{assert_not_impl_any, const_assert};
use super::{Graph, Vertex};
use itertools::Itertools;
use rand::Rng;
use std::{
    fmt::{self, Debug},
    ops::Index,
};

type NodeId<N> = <GraphCSR<N> as Graph>::NodeId;
type Edge<N> = super::Edge<NodeId<N>>;

/// CSR (compressed sparse row)
/// ```md
/// [2, 5, 6, 1, 3, 5, 6, 10] |neighbourhoods| = n_links
///  ^        ^  ^         ^
/// [0,       3, 4,        7] |neighbourood_boundaries| = n_vertices + 1
///
/// add_link(1, 50)
/// [2, 5, 6, 50, 1, 3, 5, 6, 10]
///  ^        ^      ^         ^
/// [0,       3,     5,        8]
/// ```
///
/// Changing is hard
///
/// You can do row or column oriented. This is imporant if it's a matrix and not a graph and
/// multiplication is done.
#[derive(Default)]
pub struct GraphCSR<N = usize> {
    links: Vec<N>,
    row_indexes: Box<[usize]>,
}

impl<N> Graph for GraphCSR<N>
where
    N: Vertex,
{
    type NodeId = N;
    /// Create a new GraphCSR
    fn new<I>(n_vertices: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge<N>>,
    {
        let mut s = Self {
            links: Vec::with_capacity(edges.len()),
            row_indexes: vec![0; n_vertices + 1].into_boxed_slice(),
        };
        edges.for_each(|(from, to)| {
            s.add_link(from, to);
        });
        s
    }

    fn vertices(&self) -> usize {
        self.row_indexes.len() - 1
    }

    fn edges(&self) -> usize {
        self.links.len()
    }

    fn random_edge<R: Rng>(&self, mut rng: R) -> Edge<N> {
        let neighbour_idx = rng.gen_range(0, self.edges());
        let from = match self.row_indexes.binary_search(&neighbour_idx) {
            Ok(mut idx) => {
                while self.row_indexes[idx] == neighbour_idx {
                    idx += 1
                }
                idx - 1
            }
            Err(idx) => idx - 1,
        };
        (N::from(from), self.links[neighbour_idx])
    }
}

impl<N> GraphCSR<N>
where
    N: Vertex,
{
    pub fn neighbours(&self, i: N) -> impl Iterator<Item = &<Self as Graph>::NodeId> {
        let from = self.row_indexes[i.into()];
        let to = self.row_indexes[i.into() + 1];
        self.links[from..to].iter()
    }

    /// Add a link to the GraphCSR
    ///
    /// # Panics
    /// If a link is added beyond the value passed to the constructor
    fn add_link(&mut self, from: N, to: N) -> bool {
        if from.into() > self.row_indexes.len() {
            return false;
        }
        let start_of_neighbours = self.row_indexes[from.into()];
        if !self.links.insert_checked(start_of_neighbours, to) {
            return false;
        }
        self.row_indexes[(from.into() + 1)..]
            .iter_mut()
            .for_each(|i| *i += 1);
        true
    }

    /// Iterate over the neighbours of each edge.
    fn neighbourhoods(&self) -> impl Iterator<Item = &[<Self as Graph>::NodeId]> {
        self.row_indexes
            .iter()
            .tuple_windows()
            .map(move |(&s, &e)| &self.links[s..e])
    }
}

/// Indexing a graph with a node returns a view of the neighbours of that node.
impl<N> Index<N> for GraphCSR<N>
where
    N: Into<usize> + Copy,
{
    type Output = [N];
    fn index(&self, i: N) -> &Self::Output {
        let from = self.row_indexes[i.into()];
        let to = self.row_indexes[i.into() + 1];
        &self.links[from..to]
    }
}

impl<N> Debug for GraphCSR<N>
where
    N: Vertex + From<usize> + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.neighbourhoods()
            .enumerate()
            .try_for_each(|(i, s)| writeln!(f, "{:?}: {:?}", i, s.iter().format(" -> ")))
    }
}

// *X >= 0 -> single
// *X < 0 -> pair(abs(*X), X + 1)

trait StaticVec<T> {
    fn insert_checked(&mut self, index: usize, val: T) -> bool;
}

impl<T> StaticVec<T> for Vec<T> {
    #[inline(always)]
    fn insert_checked(&mut self, index: usize, val: T) -> bool {
        if self.len() != self.capacity() {
            self.insert(index, val);
            true
        } else {
            false
        }
    }
}
