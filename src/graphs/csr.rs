// use static_assertions::{assert_not_impl_any, const_assert};
use super::{Edge, FromEdges, Graph};
use itertools::Itertools;
use std::{
    fmt::{self, Debug},
    ops::Index,
};

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
pub struct CSR {
    columns: Vec<usize>,
    row_indexes: Box<[usize]>,
}

impl Graph for CSR {
    type NodeWeight = ();
    type EdgeWeight = ();
    fn vertices(&self) -> usize {
        self.row_indexes.len() - 1
    }

    fn edges(&self) -> usize {
        self.columns.len()
    }
}

impl FromEdges for CSR {
    /// Create a new CSR
    fn from_edges<I, Iter>(n: usize, list: I) -> Self
    where
        I: IntoIterator<IntoIter = Iter, Item = Edge>,
        Iter: ExactSizeIterator<Item = Edge>,
    {
        let mut edges = list.into_iter();
        let mut s = Self {
            columns: Vec::with_capacity(edges.len()),
            row_indexes: vec![0; n + 1].into_boxed_slice(),
        };
        assert!(edges.all(|(from, to, (), ())| { s.add_link(from, to) }));
        s
    }
}

impl CSR {
    pub fn neighbours(&self, i: usize) -> impl Iterator<Item = &usize> {
        let from = self.row_indexes[i];
        let to = self.row_indexes[i + 1];
        self.columns[from..to].iter()
    }

    /// Add a link to the CSR
    fn add_link(&mut self, from: usize, to: usize) -> bool {
        if from > self.row_indexes.len() {
            return false;
        }
        let start_of_neighbours = self.row_indexes[from];
        if !self.columns.insert_checked(start_of_neighbours, to) {
            return false;
        }
        self.row_indexes[(from + 1)..]
            .iter_mut()
            .for_each(|i| *i += 1);
        true
    }

    /// Iterate over the neighbours of each edge.
    fn neighbourhoods(&self) -> impl Iterator<Item = &[usize]> {
        self.row_indexes
            .iter()
            .tuple_windows()
            .map(move |(&s, &e)| &self.columns[s..e])
    }
}

/// Indexing a graph with a node returns a view of the neighbours of that node.
impl Index<usize> for CSR {
    type Output = [usize];
    fn index(&self, i: usize) -> &Self::Output {
        let from = self.row_indexes[i];
        let to = self.row_indexes[i + 1];
        &self.columns[from..to]
    }
}

impl Debug for CSR {
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
