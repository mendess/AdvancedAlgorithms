// use static_assertions::{assert_not_impl_any, const_assert};
use super::Graph;
use itertools::Itertools;
use rand::Rng;
use static_assertions::const_assert;
use std::{
    convert::TryFrom,
    fmt::{self, Debug},
    mem,
    ops::Index,
};

type NodeId = <GraphCSR as Graph>::NodeId;
type Edge = super::Edge<NodeId>;

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
pub struct GraphCSR {
    links: Vec<usize>,
    row_indexes: Box<[usize]>,
}

impl Graph for GraphCSR {
    type NodeId = usize;
    /// Create a new GraphCSR
    fn new<I>(n_vertices: usize, edges: I) -> Self
    where
        I: ExactSizeIterator<Item = Edge>,
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

    fn random_edge<R: Rng>(&self, mut rng: R) -> Edge {
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
        (from, self.links[neighbour_idx])
    }
}

impl GraphCSR {
    pub fn neighbours(&self, i: usize) -> impl Iterator<Item = &<Self as Graph>::NodeId> {
        let from = self.row_indexes[i];
        let to = self.row_indexes[i + 1];
        self.links[from..to].iter()
    }

    /// Add a link to the GraphCSR
    ///
    /// # Panics
    /// If a link is added beyond the value passed to the constructor
    fn add_link(&mut self, from: usize, to: usize) -> bool {
        if from > self.row_indexes.len() {
            return false;
        }
        let start_of_neighbours = self.row_indexes[from];
        if !self.links.insert_checked(start_of_neighbours, to) {
            return false;
        }
        self.row_indexes[(from + 1)..]
            .iter_mut()
            .for_each(|i| *i += 1);
        true
    }

    pub fn convert_to_r(self) -> GraphR {
        let mut final_links = convert_vec::<_, isize>(self.links);
        for (s, e) in self.row_indexes.iter().copied().tuple_windows() {
            let link_slice = &mut final_links[s..e];
            link_slice.sort();
            if let Some(&(mut rank)) = link_slice.first() {
                for i in &mut link_slice[1..] {
                    *i -= rank;
                    rank += *i;
                }
                const REPEAT_THRESHOLD: usize = 2;
                const_assert!(REPEAT_THRESHOLD >= 2);
                let mut start = 0;
                let mut repeats = 1;
                for i in 1..link_slice.len() {
                    if link_slice[i] == link_slice[start] {
                        repeats += 1;
                    } else {
                        if repeats > REPEAT_THRESHOLD {
                            link_slice[start] =
                                -isize::try_from(repeats).expect("Number of repeats too high");
                            link_slice[start + 2..i]
                                .iter_mut()
                                .for_each(|x| *x = isize::min_value());
                        }
                        start = i;
                        repeats = 1;
                    }
                }
            }
        }
        // clean all the `isize::min_value()` elements
        let mut writer = 0;
        for reader in 0..final_links.len() {
            if final_links[reader] != isize::min_value() {
                final_links[writer] = final_links[reader];
                writer += 1;
            }
        }
        final_links.truncate(writer);
        final_links.shrink_to_fit();
        unimplemented!()
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
impl Index<usize> for GraphCSR {
    type Output = [usize];
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
            .try_for_each(|(i, s)| writeln!(f, "{}: {}", i, s.iter().format(" -> ")))
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

#[allow(dead_code)]
pub struct GraphR {
    links: Box<[isize]>,
    row_indexes: Box<[usize]>,
}

#[allow(unused_variables)]
impl Index<isize> for GraphR {
    type Output = [isize];
    fn index(&self, i: isize) -> &Self::Output {
        unimplemented!()
    }
}

fn convert_vec<F, T>(mut v: Vec<F>) -> Vec<T> {
    let len = v.len();
    let capacity = v.capacity();
    let ptr = v.as_mut_ptr();
    mem::forget(v);
    unsafe { Vec::from_raw_parts(ptr as *mut T, len, capacity) }
}
