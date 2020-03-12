use super::*;
use rand::seq::IteratorRandom;
use std::{mem, ops::Index};

type Neighbours = Vec<usize>;

pub struct Adjacency {
    matrix: Vec<Neighbours>,
    missing_links: usize,
}

impl Graph for Adjacency {
    type NodeName = usize;
}

impl StaticGraph for Adjacency {
    fn new<I>(n_vertices: usize, n_links: usize, i: I) -> Self
    where
        I: IntoIterator<Item = (Self::NodeName, Self::NodeName)>,
    {
        let mut s = Self {
            matrix: vec![vec![]; n_vertices],
            missing_links: n_links,
        };
        i.into_iter().for_each(|(from, to)| { s.add_link(from, to); });
        s
    }
}

impl MutableGraph for Adjacency {
    fn add_link(&mut self, from: usize, to: usize) -> bool {
        if from >= self.matrix.len() || self.missing_links == 0 {
            false
        } else {
            self.matrix[from].push(to);
            self.missing_links -= 1;
            true
        }
    }
}

impl Adjacency {
    fn contract(&mut self) {
        let mut rng = rand::thread_rng();
        let (i, j): (usize, usize) = self
            .matrix
            .iter()
            .enumerate()
            .flat_map(|(i, neigh)| neigh.iter().enumerate().map(move |(j, _)| (i, j)))
            .choose(&mut rng)
            .expect("Graph was empty");

        let old_neigh = mem::replace(&mut self.matrix[j], Vec::new());
        self.matrix[i].extend(old_neigh.iter().filter(|n| **n != i));
    }

    pub fn possible_min_cut(mut self) -> usize {
        let mut vertices = self.matrix.len();
        while vertices > 2 {
            self.contract();
            vertices -= 1;
        }
        self.matrix.iter().flat_map(|n| n.iter()).count()
    }
}

impl Index<usize> for Adjacency {
    type Output = [usize];
    fn index(&self, u: usize) -> &Self::Output {
        &self.matrix[u]
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{matrix::Adjacency, Graph};
    #[test]
    fn add_link() {
        let mut g = Adjacency::new(3, 6);
        assert!(g.add_link(1, 2));
        assert!(g[1].contains(&2));
    }

    #[test]
    #[should_panic]
    fn add_invalid_link() {
        let mut g = Adjacency::new(3, 2);
        assert!(g.add_link(3, 1));
    }

    #[test]
    #[should_panic]
    fn add_invalid_link2() {
        let mut g = Adjacency::new(3, 2);
        assert!(g.add_link(0, 2));
        assert!(g.add_link(0, 3));
        assert!(g.add_link(0, 4));
    }
}
