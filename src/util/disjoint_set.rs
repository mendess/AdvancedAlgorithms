use static_assertions::const_assert_eq;
use std::mem::size_of;

pub trait DisjointSet {
    fn new(components: usize) -> Self;
    fn find(&mut self, id: usize) -> usize;
    fn union(&mut self, i: usize, j: usize);
    fn are_connected(&mut self, i: usize, j: usize) -> bool;
    fn components(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct SimpleDisjointSet {
    ids: Box<[usize]>,
    ranks: Box<[u64]>,
    components: usize,
}

impl DisjointSet for SimpleDisjointSet {
    fn new(components: usize) -> Self {
        Self {
            ranks: vec![0; components].into(),
            ids: (0..components).collect::<Vec<_>>().into(),
            components,
        }
    }

    fn find(&mut self, id: usize) -> usize {
        if self.ids[id] != id {
            self.ids[id] = self.find(self.ids[id])
        }
        self.ids[id]
    }

    fn union(&mut self, i: usize, j: usize) {
        let mut parent = self.find(i);
        let mut child = self.find(j);
        if self.ranks[parent] < self.ranks[child] {
            std::mem::swap(&mut parent, &mut child)
        }
        self.ids[child] = parent;
        self.ranks[parent] += 1;
        self.components -= 1;
    }

    fn are_connected(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    fn components(&self) -> usize {
        self.components
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct Node {
    id: usize,
    rank: u64,
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
pub struct UndoDisjointSet {
    components: usize,
    num_sets: usize,
    nodes: Box<[Node]>,
    history: History,
}

impl DisjointSet for UndoDisjointSet {
    fn components(&self) -> usize {
        self.components
    }

    fn new(num_sets: usize) -> Self {
        Self {
            components: num_sets,
            num_sets,
            nodes: (0..num_sets)
                .map(|i| Node { id: i, rank: 1 })
                .collect::<Vec<_>>()
                .into(),
            history: Default::default(),
        }
    }

    fn find(&mut self, id: usize) -> usize {
        if self.nodes[id].id != id {
            self.nodes[id].id = self.find(self.nodes[id].id);
        }
        self.nodes[id].id
    }

    fn union(&mut self, i: usize, j: usize) {
        let mut parent = self.find(i);
        let mut child = self.find(j);
        if parent == child {
            return;
        }
        let nodes = &mut self.nodes;
        if nodes[parent].rank < nodes[child].rank {
            std::mem::swap(&mut parent, &mut child)
        }
        self.history
            .push(Change::new(parent, nodes[parent]).changed());
        self.history.push(Change::new(child, nodes[child]));
        nodes[child].id = parent;
        nodes[parent].rank += 1;
        self.components -= 1;
    }

    fn are_connected(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
}

impl UndoDisjointSet {
    pub fn save_state(&mut self) {
        self.history.save_point();
    }

    pub fn restore_state(&mut self) {
        while let Some(Operation::Change(c)) = self.history.pop() {
            self.nodes[c.id] = c.old_state;
            self.components += usize::from(c.components_changed);
        }
    }
}

use std::fmt::{self, Debug};

impl Debug for UndoDisjointSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for i in 0..self.nodes.len() {
            write!(f, "{:2},", i)?;
        }
        writeln!(f)?;
        for i in self.nodes.iter() {
            write!(f, "{:2},", i.id)?;
        }
        writeln!(f)?;
        for i in self.nodes.iter() {
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
        let mut ds = UndoDisjointSet::new(10);
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
        let ds = UndoDisjointSet::new(10);
        assert_eq!(find_test(ds.clone()), ds)
    }

    #[test]
    fn undo_union_find_test() {
        let mut ds = UndoDisjointSet::new(10);
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
        connected_test(UndoDisjointSet::new(10))
    }

    #[test]
    fn simple_union_test() {
        let mut ds = SimpleDisjointSet::new(10);
        ds.union(0, 3);
        ds.union(2, 4);
        ds.union(0, 6);
        ds.union(9, 1);
        ds.union(0, 9);
    }

    #[test]
    fn simple_find_test() {
        find_test(SimpleDisjointSet::new(10));
    }

    #[test]
    fn simple_connected_test() {
        connected_test(SimpleDisjointSet::new(10))
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
