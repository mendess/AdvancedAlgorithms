use static_assertions::const_assert_eq;
use std::{marker::PhantomData, mem::size_of};

pub trait DisjointSet {
    fn new(components: usize) -> Self;
    fn find(&mut self, id: usize) -> usize;
    fn union(&mut self, i: usize, j: usize);
    fn are_connected(&mut self, i: usize, j: usize) -> bool;
    fn components(&self) -> usize;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PathCompression;
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PathHalving;
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PathSplitting;

mod sealed {
    pub trait Sealed {}
}
pub trait FindMode: sealed::Sealed {}
impl<T> sealed::Sealed for T where T: FindMode {}
impl FindMode for PathCompression {}
impl FindMode for PathHalving {}
impl FindMode for PathSplitting {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleDisjointSet<F: FindMode = PathCompression> {
    nodes: Box<[Node]>,
    components: usize,
    _marker: PhantomData<F>,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct Node {
    id: usize,
    rank: u64,
}

macro_rules! impl_disjoint_for {
    ($mode:ty => $find:item) => {
        impl DisjointSet for SimpleDisjointSet<$mode> {
            fn new(components: usize) -> Self {
                Self {
                    nodes: (0..components)
                        .map(|i| Node { id: i, rank: 1 })
                        .collect::<Vec<_>>()
                        .into(),
                    components,
                    _marker: PhantomData,
                }
            }

            fn union(&mut self, i: usize, j: usize) {
                let mut parent = self.find(i);
                let mut child = self.find(j);
                if self.nodes[parent].rank < self.nodes[child].rank {
                    std::mem::swap(&mut parent, &mut child)
                }
                self.nodes[child].id = parent;
                self.nodes[parent].rank += 1;
                self.components -= 1;
            }

            #[inline]
            fn are_connected(&mut self, i: usize, j: usize) -> bool {
                self.find(i) == self.find(j)
            }

            #[inline]
            fn components(&self) -> usize {
                self.components
            }

            $find
        }
    }
}

impl_disjoint_for!(PathHalving =>
    fn find(&mut self, mut id: usize) -> usize {
        while self.parent(id) != id {
            *self.parent_ref(id) = self.parent(self.parent(id));
            id = self.parent(id);
        }
        id
    }
);

impl_disjoint_for!(PathCompression =>
    fn find(&mut self, id: usize) -> usize {
        if self.parent(id) != id {
            *self.parent_ref(id) = self.find(self.parent(id));
        }
        self.nodes[id].id
    }
);
impl_disjoint_for!(PathSplitting =>
    fn find(&mut self, mut id: usize) -> usize {
        while self.parent(id) != id {
            id = self.parent(id);
            *self.parent_ref(id) = self.parent(self.parent(id));
        }
        id
    }
);

impl<F: FindMode> SimpleDisjointSet<F> {
    #[inline]
    fn parent(&self, id: usize) -> usize {
        self.nodes[id].id
    }

    #[inline]
    fn parent_ref(&mut self, id: usize) -> &mut usize {
        &mut self.nodes[id].id
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct Change {
    id: usize,
    old_state: Node,
    components_changed: bool,
}

impl Change {
    fn new(id: usize, old_state: Node) -> Self {
        Self {
            id,
            old_state,
            components_changed: false,
        }
    }

    fn changed(self) -> Self {
        Self {
            components_changed: true,
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operation {
    Change(Change),
    SavePoint,
}

const_assert_eq!(size_of::<Operation>(), size_of::<Option<Operation>>());

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct History(Vec<Operation>);

impl History {
    fn push(&mut self, c: Change) {
        self.0.push(Operation::Change(c))
    }

    fn save_point(&mut self) {
        self.0.push(Operation::SavePoint);
    }

    fn pop(&mut self) -> Option<Operation> {
        self.0.pop()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct UndoDisjointSet<F: FindMode = PathCompression> {
    set: SimpleDisjointSet<F>,
    history: History,
}

impl<F: FindMode> DisjointSet for UndoDisjointSet<F>
where
    SimpleDisjointSet<F>: DisjointSet,
{
    #[inline]
    fn components(&self) -> usize {
        self.set.components()
    }

    #[inline]
    fn new(components: usize) -> Self {
        Self {
            set: SimpleDisjointSet::new(components),
            history: Default::default(),
        }
    }

    #[inline]
    fn find(&mut self, i: usize) -> usize {
        self.set.find(i)
    }

    fn union(&mut self, i: usize, j: usize) {
        let mut parent = self.set.find(i);
        let mut child = self.set.find(j);
        if parent == child {
            return;
        }
        let nodes = &mut self.set.nodes;
        if nodes[parent].rank < nodes[child].rank {
            std::mem::swap(&mut parent, &mut child)
        }
        self.history
            .push(Change::new(parent, nodes[parent]).changed());
        self.history.push(Change::new(child, nodes[child]));
        nodes[child].id = parent;
        nodes[parent].rank += 1;
        self.set.components -= 1;
    }

    #[inline]
    fn are_connected(&mut self, i: usize, j: usize) -> bool {
        self.set.are_connected(i, j)
    }
}

impl<F: FindMode> UndoDisjointSet<F>
where
    UndoDisjointSet<F>: DisjointSet,
{
    pub fn save_state(&mut self) {
        self.history.save_point()
    }

    pub fn restore_state(&mut self) {
        while let Some(Operation::Change(c)) = self.history.pop() {
            self.set.nodes[c.id] = c.old_state;
            self.set.components += usize::from(c.components_changed);
        }
    }
}

use std::fmt::{self, Debug};

impl<F: FindMode> Debug for UndoDisjointSet<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for i in 0..self.set.nodes.len() {
            write!(f, "{:2},", i)?;
        }
        writeln!(f)?;
        for i in self.set.nodes.iter() {
            write!(f, "{:2},", i.id)?;
        }
        writeln!(f)?;
        for i in self.set.nodes.iter() {
            write!(f, "{:2},", i.rank)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_union_test() {
        let mut ds = UndoDisjointSet::<PathCompression>::new(10);
        let anchor = ds.clone();
        ds.save_state();
        ds.union(0, 3);
        ds.union(2, 4);
        ds.union(0, 6);
        ds.union(9, 1);
        ds.restore_state();
        assert_eq!(ds, anchor)
    }

    #[test]
    fn undo_find_test() {
        let ds = UndoDisjointSet::<PathCompression>::new(10);
        assert_eq!(find_test(ds.clone()), ds)
    }

    #[test]
    fn undo_union_find_test() {
        let mut ds = UndoDisjointSet::<PathCompression>::new(10);
        let anchor = ds.clone();
        ds.save_state();
        ds.union(3, 6);
        ds.find(3);
        ds.find(6);
        ds.union(2, 4);
        ds.find(2);
        ds.find(4);
        ds.restore_state();
        assert_eq!(ds, anchor)
    }

    #[test]
    fn undo_connected_test() {
        connected_test(UndoDisjointSet::<PathCompression>::new(10))
    }

    #[test]
    fn simple_union_test() {
        let mut ds = SimpleDisjointSet::<PathCompression>::new(10);
        ds.union(0, 3);
        ds.union(2, 4);
        ds.union(0, 6);
        ds.union(9, 1);
        ds.union(0, 9);
    }

    #[test]
    fn simple_find_test() {
        find_test(SimpleDisjointSet::<PathCompression>::new(10));
    }

    #[test]
    fn simple_connected_test() {
        connected_test(SimpleDisjointSet::<PathCompression>::new(10))
    }

    #[cfg(test)]
    fn find_test<D: DisjointSet>(mut ds: D) -> D {
        for i in 0..10 {
            assert_eq!(i, ds.find(i));
        }
        ds
    }

    #[cfg(test)]
    fn connected_test<D: DisjointSet>(mut ds: D) {
        ds.union(3, 6);
        assert!(ds.are_connected(3, 6));
        ds.union(2, 4);
        assert!(ds.are_connected(2, 4));
        ds.union(2, 3);
        for i in (0..10).filter(|i| [2, 3, 4, 6].contains(&i)) {
            for j in (0..10).filter(|i| [2, 3, 4, 6].contains(&i)) {
                assert!(ds.are_connected(i, j), "{} <-> {}", i, j);
            }
        }
        for i in (0..10).filter(|i| ![2, 3, 4, 6].contains(&i)) {
            for &j in &[2, 3, 4, 6] {
                if i != j {
                    assert!(!ds.are_connected(i, j), "{} <-> {}", i, j);
                }
            }
        }
        for &i in &[2, 3, 4, 6] {
            for j in (0..10).filter(|i| ![2, 3, 4, 6].contains(&i)) {
                if i != j {
                    assert!(!ds.are_connected(i, j), "{} <-> {}", i, j);
                }
            }
        }
    }
}
