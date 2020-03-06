#![allow(dead_code)]
use itertools::Itertools;
use std::{ops::Index, fmt::{self, Debug}};

/// CSR (compressed sparse row)
/// [2, 5, 6, 1, 3, 5, 6, 10] |neighbourhoods| = n_links
///  ^        ^  ^         ^
/// [0,       3, 4,        7] |neighbourood_boundaries| = n_vertices + 1
///
/// :add_link(1, 50)
/// [2, 5, 6, 50, 1, 3, 5, 6, 10]
///  ^        ^      ^         ^
/// [0,       3,     5,        8]
///
/// Changing is hard
///
/// You can do row or column oriented. This is imporant if it's a matrix and not a graph and
/// multiplication is done.
#[derive(Default)]
pub struct GraphCSR {
    links: Vec<isize>,
    row_indexes: Box<[usize]>,
}

impl GraphCSR {
    /// Create a new GraphCSR
    pub fn new(n_vertices: usize, n_links: usize) -> Self {
        Self {
            links: Vec::with_capacity(n_links),
            row_indexes: vec![0; n_vertices + 1].into_boxed_slice(),
        }
    }

    /// Add a link to the GraphCSR
    ///
    /// # Example
    /// ```
    /// use aava::graph::GraphCSR;
    ///
    /// let mut g = GraphCSR::new(2, 1);
    /// g.add_link(0, 1);
    /// assert_eq!(g[0], &[1]);
    /// ```
    ///
    /// # Panics
    /// If a link is added beyond the value passed to the constructor
    pub fn add_link(&mut self, from: usize, to: usize) {
        let start_of_neighbours = self.row_indexes[from];
        self.links.insert_checked(start_of_neighbours, to as isize);
        self.row_indexes[(from + 1)..]
            .iter_mut()
            .for_each(|i| *i += 1);
    }

    /// Iterate over the neighbours of each edge.
    ///
    /// # Example
    /// ```
    /// use aava::graph::GraphCSR;
    ///
    /// let mut g = GraphCSR::new(2, 1);
    /// g.add_link(0, 1);
    /// assert_eq!(g.neighbourhoods(), &[[1]]);
    /// ```
    pub fn neighbourhoods(&self) -> impl Iterator<Item = &[isize]> {
        self.row_indexes
            .iter()
            .tuple_windows()
            .map(move |(&s, &e)| &self.links[s..e])
    }

    pub fn convert_to_r(self) -> GraphR {
        let mut final_links = self.links.into_boxed_slice();
        for (&s, &e) in self.row_indexes.iter().tuple_windows() {
            final_links[s..e].sort();
            let mut rank = final_links[s];
            for i in &mut final_links[(s + 1)..e] {
                *i = *i - rank;
                rank += *i;
            }
            // join into pairs
        }
        unimplemented!()
    }

}

impl Index<usize> for GraphCSR {
    type Output = [isize];
    fn index(&self, i: usize) -> &Self::Output {
        let from = self.row_indexes[i];
        let to = self.row_indexes[i + 1];
        &self.links[from..to]
    }
}

impl Debug for GraphCSR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.neighbourhoods()
            .enumerate()
            .try_for_each(|(i, s)| write!(f, "{}: {}\n", i, s.iter().format(" -> ")))
    }
}

// *X >= 0 -> single
// *X < 0 -> pair(abs(*X), X + 1)

pub struct GraphR {
    links: Box<[isize]>,
    row_indexes: Box<[usize]>,
}

trait StaticVec<T> {
    fn insert_checked(&mut self, index: usize, val: T);
}

impl<T> StaticVec<T> for Vec<T> {
    #[inline(always)]
    fn insert_checked(&mut self, index: usize, val: T) {
        debug_assert_ne!(self.len(), self.capacity());
        self.insert(index, val)
    }
}
